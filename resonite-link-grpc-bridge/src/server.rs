use crate::pb::resonite_link_server::ResoniteLink;
use crate::pb::{
    AddOrUpdateComponentRequest, AddOrUpdateSlotRequest, ComponentIdRequest, ComponentResponse,
    GetSlotRequest, RemoveSlotRequest, SlotResponse,
};
use resonite_link_client::responses::ResponseKind;
use tonic::{Code, Request, Response, Status};

pub struct LinkProxy {
    client: resonite_link_client::Client,
}

impl LinkProxy {
    pub fn new(client: resonite_link_client::Client) -> Self {
        LinkProxy { client }
    }

    async fn proxy_grpc_req(
        &self,
        req: resonite_link_client::Message,
    ) -> Result<resonite_link_client::Response, Status> {
        let response = self
            .client
            .send(req)
            .await
            .map_err(|e| Status::new(Code::Unavailable, format!("{:?}", e)))?;

        if !response.success {
            return Err(Status::new(
                Code::InvalidArgument,
                response.error_info.unwrap_or_else(|| {
                    "Resonite said there was an error, but did not provide an error message.".into()
                }),
            ));
        }

        Ok(response)
    }

    async fn proxy_slot_req(
        &self,
        req: resonite_link_client::Message,
    ) -> Result<Response<SlotResponse>, Status> {
        let response = self.proxy_grpc_req(req).await?;

        if let ResponseKind::SlotData { depth, data } = response.kind {
            Ok(Response::new(SlotResponse {
                data: data.map(Into::into),
                depth,
            }))
        } else {
            Err(Status::new(Code::Internal, "Incorrect response kind."))
        }
    }

    async fn proxy_component_req(
        &self,
        req: resonite_link_client::Message,
    ) -> Result<Response<ComponentResponse>, Status> {
        let response = self.proxy_grpc_req(req).await?;

        if let ResponseKind::ComponentData { data } = response.kind {
            Ok(Response::new(ComponentResponse {
                data: data.map(Into::into),
            }))
        } else {
            Err(Status::new(Code::Internal, "Incorrect response kind."))
        }
    }
}

#[tonic::async_trait]
impl ResoniteLink for LinkProxy {
    async fn get_slot(
        &self,
        request: Request<GetSlotRequest>,
    ) -> Result<Response<SlotResponse>, Status> {
        let request = request.into_inner();
        self.proxy_slot_req(resonite_link_client::Message::GetSlot {
            slot_id: request.slot_id,
            depth: request.depth,
            include_component_data: request.include_component_data,
        })
        .await
    }

    async fn add_slot(
        &self,
        request: Request<AddOrUpdateSlotRequest>,
    ) -> Result<Response<SlotResponse>, Status> {
        let request = request.into_inner();
        if let Some(slot) = request.data {
            self.proxy_slot_req(resonite_link_client::Message::AddSlot { data: slot.into() })
                .await
        } else {
            Err(Status::new(Code::InvalidArgument, "data is required."))
        }
    }

    async fn update_slot(
        &self,
        request: Request<AddOrUpdateSlotRequest>,
    ) -> Result<Response<SlotResponse>, Status> {
        let request = request.into_inner();
        if let Some(slot) = request.data {
            self.proxy_slot_req(resonite_link_client::Message::UpdateSlot { data: slot.into() })
                .await
        } else {
            Err(Status::new(Code::InvalidArgument, "data is required."))
        }
    }

    async fn remove_slot(
        &self,
        request: Request<RemoveSlotRequest>,
    ) -> Result<Response<SlotResponse>, Status> {
        let request = request.into_inner();
        self.proxy_slot_req(resonite_link_client::Message::RemoveSlot {
            slot_id: request.slot_id,
        })
        .await
    }

    async fn get_component(
        &self,
        request: Request<ComponentIdRequest>,
    ) -> Result<Response<ComponentResponse>, Status> {
        let request = request.into_inner();
        self.proxy_component_req(resonite_link_client::Message::GetComponent {
            component_id: request.component_id,
        })
        .await
    }

    async fn add_component(
        &self,
        request: Request<AddOrUpdateComponentRequest>,
    ) -> Result<Response<ComponentResponse>, Status> {
        let request = request.into_inner();
        if let Some(component) = request.data {
            self.proxy_component_req(resonite_link_client::Message::AddComponent {
                container_slot_id: request.container_slot_id,
                data: component.into(),
            })
            .await
        } else {
            Err(Status::new(Code::InvalidArgument, "data is required."))
        }
    }

    async fn update_component(
        &self,
        request: Request<AddOrUpdateComponentRequest>,
    ) -> Result<Response<ComponentResponse>, Status> {
        let request = request.into_inner();
        if let Some(component) = request.data {
            self.proxy_component_req(resonite_link_client::Message::UpdateComponent {
                data: component.into(),
            })
            .await
        } else {
            Err(Status::new(Code::InvalidArgument, "data is required."))
        }
    }

    async fn remove_component(
        &self,
        request: Request<ComponentIdRequest>,
    ) -> Result<Response<ComponentResponse>, Status> {
        let request = request.into_inner();
        self.proxy_component_req(resonite_link_client::Message::RemoveComponent {
            component_id: request.component_id,
        })
        .await
    }
}
