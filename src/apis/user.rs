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
pub enum CreateUserResponse {
    /// successful operation
    Status200_SuccessfulOperation(models::User),
    /// Unexpected error
    Status0_UnexpectedError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum CreateUsersWithListInputResponse {
    /// Successful operation
    Status200_SuccessfulOperation(models::User),
    /// Unexpected error
    Status0_UnexpectedError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum DeleteUserResponse {
    /// User deleted
    Status200_UserDeleted,
    /// Invalid username supplied
    Status400_InvalidUsernameSupplied,
    /// User not found
    Status404_UserNotFound,
    /// Unexpected error
    Status0_UnexpectedError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetUserByNameResponse {
    /// successful operation
    Status200_SuccessfulOperation(models::User),
    /// Invalid username supplied
    Status400_InvalidUsernameSupplied,
    /// User not found
    Status404_UserNotFound,
    /// Unexpected error
    Status0_UnexpectedError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum LoginUserResponse {
    /// successful operation
    Status200_SuccessfulOperation {
        body: String,
        x_rate_limit: Option<i32>,
        x_expires_after: Option<chrono::DateTime<chrono::Utc>>,
    },
    /// Invalid username/password supplied
    Status400_InvalidUsername,
    /// Unexpected error
    Status0_UnexpectedError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum LogoutUserResponse {
    /// successful operation
    Status200_SuccessfulOperation,
    /// Unexpected error
    Status0_UnexpectedError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UpdateUserResponse {
    /// successful operation
    Status200_SuccessfulOperation,
    /// bad request
    Status400_BadRequest,
    /// user not found
    Status404_UserNotFound,
    /// Unexpected error
    Status0_UnexpectedError(models::Error),
}

/// User
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait User<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    /// Create user..
    ///
    /// CreateUser - POST /api/v3/user
    async fn create_user(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<models::User>,
    ) -> Result<CreateUserResponse, E>;

    /// Creates list of users with given input array..
    ///
    /// CreateUsersWithListInput - POST /api/v3/user/createWithList
    async fn create_users_with_list_input(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &Option<Vec<models::User>>,
    ) -> Result<CreateUsersWithListInputResponse, E>;

    /// Delete user resource..
    ///
    /// DeleteUser - DELETE /api/v3/user/{username}
    async fn delete_user(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        path_params: &models::DeleteUserPathParams,
    ) -> Result<DeleteUserResponse, E>;

    /// Get user by user name..
    ///
    /// GetUserByName - GET /api/v3/user/{username}
    async fn get_user_by_name(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        path_params: &models::GetUserByNamePathParams,
    ) -> Result<GetUserByNameResponse, E>;

    /// Logs user into the system..
    ///
    /// LoginUser - GET /api/v3/user/login
    async fn login_user(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::LoginUserQueryParams,
    ) -> Result<LoginUserResponse, E>;

    /// Logs out current logged in user session..
    ///
    /// LogoutUser - GET /api/v3/user/logout
    async fn logout_user(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
    ) -> Result<LogoutUserResponse, E>;

    /// Update user resource..
    ///
    /// UpdateUser - PUT /api/v3/user/{username}
    async fn update_user(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        path_params: &models::UpdateUserPathParams,
        body: &Option<models::User>,
    ) -> Result<UpdateUserResponse, E>;
}
