use std::error::Error;

use tonic::{Response, Status, Request, transport::Server};
use turing::{turing_pi_server::{TuringPi, TuringPiServer}, PingResponse};

pub mod turing {
    tonic::include_proto!("turing");
}

#[derive(Debug)]
struct TpiServer;

#[tonic::async_trait]
impl TuringPi for TpiServer {
    async fn ping(
        &self,
        request: Request<turing::PingRequest>,
    ) -> Result<Response<PingResponse>, Status>
    {
        Ok(Response::new(PingResponse { }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let addr = "0.0.0.0:13337".parse()?;

    let tpi_server = TpiServer;

    Server::builder()
        .add_service(TuringPiServer::new(tpi_server))
        .serve(addr)
        .await?;

    Ok(())
}
