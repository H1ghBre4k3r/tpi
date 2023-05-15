use std::error::Error;

use tonic::Request;
use turing::{turing_pi_client::TuringPiClient, PingRequest};

pub mod turing {
    tonic::include_proto!("turing");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = TuringPiClient::connect("http://192.168.2.160:50051").await?;

    let request = Request::new(PingRequest {});

    let response = client.ping(request).await?;

    Ok(())
}
