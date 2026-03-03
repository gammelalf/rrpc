use std::hash::BuildHasher;
use std::hash::Hasher;
use std::hash::RandomState;

use api::GetRandomNumber;
use api::GetRandomNumberRequest;
use api::GetRandomNumberResponse;
use rrpc::server::NonStreamingMethod;
use tonic::Request;
use tonic::Response;
use tonic::Status;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Server::builder()
        .add_service(GetRandomNumber::service(get_random_number))
        .serve_with_shutdown("127.0.0.1:8080".parse().unwrap(), async move {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await?;
    Ok(())
}

async fn get_random_number(
    request: Request<GetRandomNumberRequest>,
) -> Result<Response<GetRandomNumberResponse>, Status> {
    let GetRandomNumberRequest {
        min: None,
        max: None,
    } = request.into_inner()
    else {
        return Err(Status::unimplemented("Bounds are not implemented"));
    };

    Ok(Response::new(GetRandomNumberResponse {
        number: get_random(),
    }))
}

fn get_random() -> u64 {
    RandomState::new().build_hasher().finish()
}
