use std::convert::Infallible;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

use axum::http;
use tonic::body::Body;
use tonic::codegen::tokio_stream::Stream;
use tonic::server::Grpc;
use tonic::server::NamedService;
use tower::Service;

use crate::Method;
use crate::True;
use crate::codec::json::JsonCodec;
use crate::server::BothStreamingMethod;
use crate::server::ResponseStream;
use crate::server::tonic_service::TonicService;

impl<T> BothStreamingMethod for T
where
    Self: Method<IsRequestStreamed = True, IsResponseStreamed = True>,
{
    fn service(handler: impl BothStreamingHandler<Self>) -> impl TonicService {
        BothStreamingService::<Self, _>::new(handler)
    }
}

/// [`tower::Service`] serving client-streaming methods
pub struct BothStreamingService<M, H> {
    method: PhantomData<M>,
    handler: H,
}

impl<M, H> BothStreamingService<M, H> {
    /// Wraps a `BothStreamingHandler` to implement [`tower::Service`]
    pub fn new(handler: H) -> Self
    where
        M: BothStreamingMethod,
        H: BothStreamingHandler<M>,
    {
        Self {
            method: PhantomData,
            handler,
        }
    }
}

impl<M, H> Clone for BothStreamingService<M, H>
where
    H: Clone,
{
    fn clone(&self) -> Self {
        Self {
            method: PhantomData,
            handler: self.handler.clone(),
        }
    }
}
impl<M, H> NamedService for BothStreamingService<M, H>
where
    M: Method,
{
    const NAME: &'static str = M::ID;
}
impl<M, H> Service<http::Request<Body>> for BothStreamingService<M, H>
where
    M: BothStreamingMethod,
    H: BothStreamingHandler<M>,
{
    type Response = http::Response<Body>;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: http::Request<Body>) -> Self::Future {
        let this = self.clone();
        Box::pin(async move {
            let response = Grpc::new(JsonCodec::<M::Request, M::Response>::new())
                .streaming(this, request)
                .await;
            Ok(response)
        })
    }
}

impl<M, H> tonic::server::StreamingService<M::Request> for BothStreamingService<M, H>
where
    M: BothStreamingMethod,
    H: BothStreamingHandler<M>,
{
    type Response = M::Response;
    type ResponseStream = Pin<Box<dyn Stream<Item = tonic::Result<Self::Response>> + Send>>;
    type Future =
        Pin<Box<dyn Future<Output = tonic::Result<tonic::Response<Self::ResponseStream>>> + Send>>;

    fn call(&mut self, request: tonic::Request<tonic::Streaming<M::Request>>) -> Self::Future {
        Box::pin(self.handler.call(request))
    }
}

pub trait BothStreamingHandler<M>
where
    M: BothStreamingMethod,
    Self: Clone + Send + Sync + 'static,
{
    fn call(
        &mut self,
        request: tonic::Request<tonic::Streaming<M::Request>>,
    ) -> impl Future<Output = tonic::Result<tonic::Response<ResponseStream<M::Response>>>> + Send + 'static;
}
impl<M, H, F> BothStreamingHandler<M> for H
where
    M: BothStreamingMethod,
    Self: Clone + Send + Sync + 'static,
    H: FnMut(tonic::Request<tonic::Streaming<M::Request>>) -> F,
    F: Future<Output = tonic::Result<tonic::Response<ResponseStream<M::Response>>>>
        + Send
        + 'static,
{
    fn call(&mut self, request: tonic::Request<tonic::Streaming<M::Request>>) -> F {
        #![allow(refining_impl_trait)]
        self(request)
    }
}
