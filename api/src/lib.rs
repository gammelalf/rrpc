use base::False;
use base::Method;
use serde::Deserialize;
use serde::Serialize;

pub struct GetRandomNumber;
impl Method for GetRandomNumber {
    const ID: &'static str = "GetRandomNumber";

    type IsRequestStreamed = False;
    type IsResponseStreamed = False;

    type Request = GetRandomNumberRequest;
    type Response = GetRandomNumberResponse;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetRandomNumberRequest {
    pub min: Option<u64>,
    pub max: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetRandomNumberResponse {
    pub number: u64,
}
