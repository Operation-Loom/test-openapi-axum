use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::{CookieJar, Host};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetHelloResponse {
    /// Successful response
    Status200_SuccessfulResponse(models::Greeting),
}

/// Default
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Default<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// Returns a greeting message.
    ///
    /// GetHello - GET /hello
    async fn get_hello(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<GetHelloResponse, E>;
}
