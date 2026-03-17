pub mod client;
mod codec;
pub mod server;
mod utils;

use serde::Serialize;
use serde::de::DeserializeOwned;

pub trait Method
where
    Self: Sized + Send + Sync + 'static,
{
    const ID: &'static str;

    type IsRequestStreamed: Bool;
    type IsResponseStreamed: Bool;

    type Request: Serialize + DeserializeOwned + Send + Sync + 'static;
    type Response: Serialize + DeserializeOwned + Send + Sync + 'static;
}

pub struct True;
pub struct False;
pub trait Bool {
    sealed!(trait);
}
impl Bool for True {
    sealed!(impl);
}
impl Bool for False {
    sealed!(impl);
}
