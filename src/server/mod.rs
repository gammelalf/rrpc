use futures_core::stream::BoxStream;

use crate::False;
use crate::Method;
use crate::True;
use crate::server::both_streaming::BothStreamingHandler;
use crate::server::client_streaming::ClientStreamingHandler;
use crate::server::non_streaming::NonStreamingHandler;
use crate::server::server_streaming::ServerStreamingHandler;
use crate::server::tonic_service::TonicService;

pub mod both_streaming;
pub mod client_streaming;
pub mod non_streaming;
pub mod server_streaming;
pub mod tonic_service;

pub trait NonStreamingMethod
where
    Self: Method<IsRequestStreamed = False, IsResponseStreamed = False>,
{
    fn service(handler: impl NonStreamingHandler<Self>) -> impl TonicService;
}

pub trait ClientStreamingMethod
where
    Self: Method<IsRequestStreamed = True, IsResponseStreamed = False>,
{
    fn service(handler: impl ClientStreamingHandler<Self>) -> impl TonicService;
}

pub trait ServerStreamingMethod
where
    Self: Method<IsRequestStreamed = False, IsResponseStreamed = True>,
{
    fn service(handler: impl ServerStreamingHandler<Self>) -> impl TonicService;
}

pub trait BothStreamingMethod
where
    Self: Method<IsRequestStreamed = True, IsResponseStreamed = True>,
{
    fn service(handler: impl BothStreamingHandler<Self>) -> impl TonicService;
}

/// Type alias for a boxed stream of responses
pub type ResponseStream<Response> = BoxStream<'static, tonic::Result<Response>>;
