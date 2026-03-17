use api::GetRandomNumber;
use api::GetRandomNumberRequest;
use api::GetRandomNumberResponse;
use rrpc::client::NonStreamingMethod;
use tonic::Request;
use tonic::transport::Channel;
use tonic::transport::Uri;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut channel = Channel::builder(Uri::from_static("http://127.0.0.1:8080"))
        .connect()
        .await?;

    let response = GetRandomNumber::call(
        &mut channel,
        Request::new(GetRandomNumberRequest {
            min: None,
            max: None,
        }),
    )
    .await?;

    let GetRandomNumberResponse { number: _ } = response.into_inner();

    Ok(())
}
