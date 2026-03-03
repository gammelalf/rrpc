use std::convert::Infallible;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

use axum::http;
use tonic::body::Body;
use tonic::server::Grpc;
use tonic::server::NamedService;
use tower::Service;

use crate::False;
use crate::Method;
use crate::codec::json::JsonCodec;
use crate::server::NonStreamingMethod;
use crate::server::tonic_service::TonicService;

impl<T> NonStreamingMethod for T
where
    Self: Method<IsRequestStreamed = False, IsResponseStreamed = False>,
{
    fn service(handler: impl NonStreamingHandler<Self>) -> impl TonicService {
        NonStreamingService::<Self, _>::new(handler)
    }
}

/// [`tower::Service`] serving non-streaming methods
pub struct NonStreamingService<M, H> {
    method: PhantomData<M>,
    handler: H,
}

impl<M, H> NonStreamingService<M, H> {
    /// Wraps a `NonStreamingMethod` to implement [`tower::Service`]
    pub fn new(handler: H) -> Self
    where
        M: NonStreamingMethod,
        H: NonStreamingHandler<M>,
    {
        Self {
            method: PhantomData,
            handler,
        }
    }
}

impl<M, H> Clone for NonStreamingService<M, H>
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
impl<M, H> NamedService for NonStreamingService<M, H>
where
    M: Method,
{
    const NAME: &'static str = M::ID;
}
impl<M, H> Service<http::Request<Body>> for NonStreamingService<M, H>
where
    M: NonStreamingMethod,
    H: NonStreamingHandler<M>,
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
                .unary(this, request)
                .await;
            Ok(response)
        })
    }
}

impl<M, H> tonic::server::UnaryService<M::Request> for NonStreamingService<M, H>
where
    M: NonStreamingMethod,
    H: NonStreamingHandler<M>,
{
    type Response = M::Response;
    type Future =
        Pin<Box<dyn Future<Output = tonic::Result<tonic::Response<Self::Response>>> + Send>>;

    fn call(&mut self, request: tonic::Request<M::Request>) -> Self::Future {
        Box::pin(self.handler.call(request))
    }
}

pub trait NonStreamingHandler<M>
where
    M: NonStreamingMethod,
    Self: Clone + Send + Sync + 'static,
{
    fn call(
        &mut self,
        request: tonic::Request<M::Request>,
    ) -> impl Future<Output = tonic::Result<tonic::Response<M::Response>>> + Send + 'static;
}
impl<M, H, F> NonStreamingHandler<M> for H
where
    M: NonStreamingMethod,
    Self: Clone + Send + Sync + 'static,
    H: FnMut(tonic::Request<M::Request>) -> F,
    F: Future<Output = tonic::Result<tonic::Response<M::Response>>> + Send + 'static,
{
    fn call(&mut self, request: tonic::Request<M::Request>) -> F {
        #![allow(refining_impl_trait)]
        self(request)
    }
}
