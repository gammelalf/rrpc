use axum::http::uri::PathAndQuery;
use futures_core::stream::BoxStream;
use tonic::body::Body;
use tonic::client::Grpc;
use tonic::client::GrpcService;

use crate::False;
use crate::Method;
use crate::True;
use crate::codec::json::JsonCodec;

pub trait NonStreamingMethod
where
    Self: Method<IsRequestStreamed = False, IsResponseStreamed = False>,
{
    fn call(
        channel: impl GrpcService<Body, ResponseBody = Body>,
        request: tonic::Request<Self::Request>,
    ) -> impl Future<Output = Result<tonic::Response<Self::Response>, tonic::Status>> {
        async move {
            Grpc::new(channel)
                .unary(
                    request,
                    PathAndQuery::from_static(Self::ID),
                    JsonCodec::<Self::Response, Self::Request>::new(),
                )
                .await
        }
    }
}
impl<T> NonStreamingMethod for T where
    T: Method<IsRequestStreamed = False, IsResponseStreamed = False>
{
}

pub trait ClientStreamingMethod
where
    Self: Method<IsRequestStreamed = True, IsResponseStreamed = False>,
{
    fn call(
        channel: impl GrpcService<Body, ResponseBody = Body>,
        request: tonic::Request<RequestStream<Self::Request>>,
    ) -> impl Future<Output = Result<tonic::Response<Self::Response>, tonic::Status>> {
        async move {
            Grpc::new(channel)
                .client_streaming(
                    request,
                    PathAndQuery::from_static(Self::ID),
                    JsonCodec::<Self::Response, Self::Request>::new(),
                )
                .await
        }
    }
}
impl<T> ClientStreamingMethod for T where
    T: Method<IsRequestStreamed = True, IsResponseStreamed = False>
{
}

pub trait ServerStreamingMethod
where
    Self: Method<IsRequestStreamed = False, IsResponseStreamed = True>,
{
    fn call(
        channel: impl GrpcService<Body, ResponseBody = Body>,
        request: tonic::Request<Self::Request>,
    ) -> impl Future<Output = Result<tonic::Response<tonic::Streaming<Self::Response>>, tonic::Status>>
    {
        async move {
            Grpc::new(channel)
                .server_streaming(
                    request,
                    PathAndQuery::from_static(Self::ID),
                    JsonCodec::<Self::Response, Self::Request>::new(),
                )
                .await
        }
    }
}
impl<T> ServerStreamingMethod for T where
    T: Method<IsRequestStreamed = False, IsResponseStreamed = True>
{
}

pub trait BothStreamingMethod
where
    Self: Method<IsRequestStreamed = True, IsResponseStreamed = True>,
{
    fn call(
        channel: impl GrpcService<Body, ResponseBody = Body>,
        request: tonic::Request<RequestStream<Self::Request>>,
    ) -> impl Future<Output = Result<tonic::Response<tonic::Streaming<Self::Response>>, tonic::Status>>
    {
        async move {
            Grpc::new(channel)
                .streaming(
                    request,
                    PathAndQuery::from_static(Self::ID),
                    JsonCodec::<Self::Response, Self::Request>::new(),
                )
                .await
        }
    }
}
impl<T> BothStreamingMethod for T where
    T: Method<IsRequestStreamed = True, IsResponseStreamed = True>
{
}

/// Type alias for a boxed stream of responses
pub type RequestStream<Request> = BoxStream<'static, Request>;
