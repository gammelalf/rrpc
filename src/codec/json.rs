use std::marker::PhantomData;
use std::sync::Arc;

use bytes::Buf;
use bytes::BufMut;
use serde::Serialize;
use serde::de::DeserializeOwned;
use tonic::Status;
use tonic::codec::Codec;
use tonic::codec::DecodeBuf;
use tonic::codec::Decoder;
use tonic::codec::EncodeBuf;
use tonic::codec::Encoder;

pub struct JsonCodec<In, Out>(PhantomData<(In, Out)>);
impl<In, Out> JsonCodec<In, Out> {
    pub const fn new() -> Self {
        JsonCodec(PhantomData)
    }
}
impl<In, Out> Default for JsonCodec<In, Out> {
    fn default() -> Self {
        Self::new()
    }
}

impl<In, Out> Codec for JsonCodec<In, Out>
where
    In: DeserializeOwned + Send + 'static,
    Out: Serialize + Send + 'static,
{
    type Encode = Out;
    type Decode = In;
    type Encoder = JsonCodec<(), Out>;
    type Decoder = JsonCodec<In, ()>;

    fn encoder(&mut self) -> Self::Encoder {
        JsonCodec::default()
    }

    fn decoder(&mut self) -> Self::Decoder {
        JsonCodec::default()
    }
}

impl<In, Out> Encoder for JsonCodec<In, Out>
where
    Out: Serialize,
{
    type Item = Out;
    type Error = Status;

    fn encode(&mut self, item: Self::Item, dst: &mut EncodeBuf<'_>) -> Result<(), Self::Error> {
        serde_json::to_writer(dst.writer(), &item).map_err(|error| {
            let mut status = Status::internal("Failed to write json");
            status.set_source(Arc::new(error));
            status
        })
    }
}

impl<In, Out> Decoder for JsonCodec<In, Out>
where
    In: DeserializeOwned,
{
    type Item = In;
    type Error = Status;

    fn decode(&mut self, src: &mut DecodeBuf<'_>) -> Result<Option<Self::Item>, Self::Error> {
        serde_json::from_reader(src.reader()).map_err(|error| {
            let mut status = Status::internal("Failed to parse json");
            status.set_source(Arc::new(error));
            status
        })
    }
}
