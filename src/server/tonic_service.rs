//! Trait alias for the `tower::Service` expected by `tonic::transport::server::Router`

use std::convert::Infallible;

use axum::http::Request;
use tonic::body::Body;
use tonic::codegen::Service;
use tonic::server::NamedService;

/// Trait alias for the `tower::Service` expected by `tonic::transport::server::Router`
pub trait TonicService
where
    Self: Service<
            Request<Body>,
            Error = Infallible,
            Response: axum::response::IntoResponse,
            Future: Send + 'static,
        >,
    Self: NamedService + Clone,
    Self: Send + Sync + 'static,
{
}
impl<T> TonicService for T
where
    Self: Service<
            Request<Body>,
            Error = Infallible,
            Response: axum::response::IntoResponse,
            Future: Send + 'static,
        >,
    Self: NamedService + Clone,
    Self: Send + Sync + 'static,
{
}
