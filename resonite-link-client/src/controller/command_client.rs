use crate::controller::id_generator::IdGenerator;
use crate::messages::{Message, MessageWrapper};
use crate::responses::Response;
use futures_util::{Sink, SinkExt, Stream, StreamExt};
use log::{error, warn};
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::{Mutex, mpsc};
use tokio::sync::{SetOnce, oneshot};
use tokio::task::JoinHandle;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};

pub struct Client {
    outbound: mpsc::UnboundedSender<OutboundMessage>,
    shutdown: Arc<SetOnce<()>>,
    handle: JoinHandle<()>,
}

impl Client {
    pub async fn send(&self, message: Message) -> Result<Response, ClientError> {
        let (tx, rx) = oneshot::channel::<Result<Response, ClientError>>();
        let outbound_message = OutboundMessage {
            message,
            response: tx,
        };
        self.outbound
            .send(outbound_message)
            .map_err(|_| ClientError::ConnectionClosed)?;
        // The end ? unwraps the channel error, the inner result is directly returned.
        rx.await.map_err(|_| ClientError::ConnectionClosed)?
    }

    pub fn is_closed(&self) -> bool {
        self.shutdown.initialized()
    }

    pub async fn close(self) {
        self.shutdown.set(()).unwrap();
        self.handle.await.unwrap();
    }

    pub async fn connect(address: &str, id_prefix: Option<&str>) -> Result<Self, ClientError> {
        let (msg_sender, msg_recv) = mpsc::unbounded_channel::<OutboundMessage>();
        let set_once: Arc<SetOnce<()>> = Default::default();

        let (client, _) = connect_async(address)
            .await
            .map_err(ClientError::FailureToConnect)?;

        let handle = tokio::spawn(serve_connection(
            msg_recv,
            set_once.clone(),
            client,
            id_prefix.map(Into::into),
        ));

        Ok(Self {
            outbound: msg_sender,
            shutdown: set_once,
            handle,
        })
    }
}

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("failed to connect to server {0}")]
    FailureToConnect(tokio_tungstenite::tungstenite::Error),
    #[error("socket connection is closed")]
    ConnectionClosed,
    #[error("outbound message is invalid {0}")]
    MessageRenderingError(serde_json::Error),
}

type Responder = oneshot::Sender<Result<Response, ClientError>>;

struct OutboundMessage {
    message: Message,
    response: Responder,
}

async fn serve_connection(
    outbound: mpsc::UnboundedReceiver<OutboundMessage>,
    shutdown: Arc<SetOnce<()>>,
    ws: WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>,
    id_prefix: Option<String>,
) {
    let (write, read) = ws.split();

    let in_flight_messages: Arc<Mutex<HashMap<String, Responder>>> = Default::default();

    let writer = tokio::spawn(ws_writer(
        outbound,
        write,
        in_flight_messages.clone(),
        id_prefix,
        shutdown.clone(),
    ));
    let reader = tokio::spawn(ws_reader(read, in_flight_messages, shutdown));

    if let (Ok(first), Ok(second)) = tokio::join!(writer, reader) {
        match first.reunite(second).unwrap().close(None).await {
            Ok(_) => (),
            Err(tokio_tungstenite::tungstenite::Error::AlreadyClosed) => (),
            Err(e) => error!("failed to close connection: {}", e),
        }
    }
}

async fn ws_writer<
    WS: Sink<tokio_tungstenite::tungstenite::Message, Error = impl Debug> + Unpin,
>(
    mut to_send: mpsc::UnboundedReceiver<OutboundMessage>,
    mut ws: WS,
    in_flight_messages: Arc<Mutex<HashMap<String, Responder>>>,
    id_prefix: Option<impl Into<String>>,
    shutdown: Arc<SetOnce<()>>,
) -> WS {
    let mut id_gen = match id_prefix {
        None => IdGenerator::default(),
        Some(prefix) => IdGenerator::new_with_prefix(prefix),
    };

    while let Some(outbound) = tokio::select! {
        msg = to_send.recv() => msg,
        _ = shutdown.wait() => return ws,
    } {
        let id = id_gen.next();

        let payload = MessageWrapper {
            message_id: id.clone(),
            inner: outbound.message,
        };

        let payload = match serde_json::to_string(&payload) {
            Ok(payload) => payload,
            Err(e) => {
                // If discarded, nothing got the message; which is fine.
                _ = outbound
                    .response
                    .send(Err(ClientError::MessageRenderingError(e)));
                continue;
            }
        };

        // TODO: Remove this response channel if the websocket fails to send.
        in_flight_messages
            .lock()
            .await
            .insert(id, outbound.response);

        if let Err(e) = ws
            .send(tokio_tungstenite::tungstenite::Message::Text(
                payload.into(),
            ))
            .await
        {
            warn!("error sending message: {:?}", e);
        }
    }

    ws
}

async fn ws_reader<
    WS: Stream<
            Item = tokio_tungstenite::tungstenite::Result<tokio_tungstenite::tungstenite::Message>,
        > + Unpin,
>(
    mut ws: WS,
    in_flight_messages: Arc<Mutex<HashMap<String, Responder>>>,
    shutdown: Arc<SetOnce<()>>,
) -> WS {
    while let Some(Ok(msg)) = tokio::select! {
        msg = ws.next() => msg,
        _ = shutdown.wait() => return ws,
    } {
        let text = msg.to_text().unwrap();
        let response: Response = match serde_json::from_str(text) {
            Ok(response) => response,
            Err(e) => {
                warn!(
                    "Message from server was not parsed successfully (The Member enum isn't complete, that may be what's wrong): {:?}",
                    e
                );
                continue;
            }
        };

        let id = response.source_message_id.as_ref().unwrap();

        if let Some(resp) = in_flight_messages.lock().await.remove(id) {
            // If discarded nothing got the message, which is fine.
            _ = resp.send(Ok(response));
        } else {
            warn!("Unpaired outbound message: {:?}", response);
        }
    }

    ws
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::responses::ResponseKind;
    use tokio::net::TcpListener;

    async fn serve_echo_requests(canned_responses: Vec<Response>) -> Vec<Message> {
        let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
        let (stream, _) = listener.accept().await.unwrap();
        let mut ws = tokio_tungstenite::accept_async(stream).await.unwrap();

        let mut messages = Vec::new();
        for mut response in canned_responses {
            let msg: MessageWrapper =
                serde_json::from_str(ws.next().await.unwrap().unwrap().to_text().unwrap()).unwrap();

            response.source_message_id = Some(msg.message_id.clone());
            messages.push(msg);
            ws.send(tokio_tungstenite::tungstenite::Message::Text(
                serde_json::to_string(&response).unwrap().into(),
            ))
            .await
            .unwrap();
        }

        messages.into_iter().map(|msg| msg.inner).collect()
    }

    fn make_response(kind: ResponseKind) -> Response {
        Response {
            source_message_id: None,
            success: false,
            error_info: None,
            kind,
        }
    }

    fn make_error_response(error_info: impl Into<String>) -> Response {
        Response {
            source_message_id: None,
            success: true,
            error_info: Some(error_info.into()),
            kind: ResponseKind::Response,
        }
    }

    #[tokio::test]
    async fn ctor_success() {
        let requests = tokio::spawn(serve_echo_requests(vec![
            make_response(ResponseKind::Response),
            make_error_response("Bad things!"),
        ]));

        tokio::time::sleep(std::time::Duration::from_millis(30)).await;

        let client = Client::connect("ws://127.0.0.1:8080", None).await.unwrap();
        let mut responses = vec![
            client
                .send(Message::GetSlot {
                    slot_id: "ROOT".into(),
                    depth: 1,
                    include_component_data: false,
                })
                .await
                .unwrap(),
            client
                .send(Message::GetSlot {
                    slot_id: "ROOT".into(),
                    depth: 1,
                    include_component_data: false,
                })
                .await
                .unwrap(),
        ];

        client.close().await;
        assert_eq!(
            requests.await.unwrap(),
            vec![
                Message::GetSlot {
                    slot_id: "ROOT".into(),
                    depth: 1,
                    include_component_data: false,
                },
                Message::GetSlot {
                    slot_id: "ROOT".into(),
                    depth: 1,
                    include_component_data: false,
                }
            ]
        );

        for response in responses.iter_mut() {
            // Clear the source ID as they're random.
            response.source_message_id = None;
        }
        assert_eq!(
            responses,
            vec![
                make_response(ResponseKind::Response),
                make_error_response("Bad things!"),
            ]
        );
    }
}
