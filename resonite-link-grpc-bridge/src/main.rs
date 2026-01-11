use crate::server::LinkProxy;
use clap::Parser;
use tonic::transport::Server;

mod pb;
mod server;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// The Resolink address to bind to.
    ///
    /// normally ws://localhost:<resonite provided port>
    #[arg(short, long)]
    resolink_addr: String,

    /// The GRPC address for the server to be available at.
    ///
    /// 127.0.0.1:8080
    #[arg(short, long)]
    grpc_addr: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args = Args::parse();

    let client = resonite_link_client::Client::connect(&args.resolink_addr, None).await?;

    Server::builder()
        .add_service(pb::resonite_link_server::ResoniteLinkServer::new(
            LinkProxy::new(client),
        ))
        .serve(args.grpc_addr.parse()?)
        .await?;

    Ok(())
}
