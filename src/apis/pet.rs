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
pub enum AddPetResponse {
    /// Successful operation
    Status200_SuccessfulOperation(models::Pet),
    /// Invalid input
    Status400_InvalidInput,
    /// Validation exception
    Status422_ValidationException,
    /// Unexpected error
    Status0_UnexpectedError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum DeletePetResponse {
    /// Pet deleted
    Status200_PetDeleted,
    /// Invalid pet value
    Status400_InvalidPetValue,
    /// Unexpected error
    Status0_UnexpectedError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum FindPetsByStatusResponse {
    /// successful operation
    Status200_SuccessfulOperation(Vec<models::Pet>),
    /// Invalid status value
    Status400_InvalidStatusValue,
    /// Unexpected error
    Status0_UnexpectedError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum FindPetsByTagsResponse {
    /// successful operation
    Status200_SuccessfulOperation(Vec<models::Pet>),
    /// Invalid tag value
    Status400_InvalidTagValue,
    /// Unexpected error
    Status0_UnexpectedError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum GetPetByIdResponse {
    /// successful operation
    Status200_SuccessfulOperation(models::Pet),
    /// Invalid ID supplied
    Status400_InvalidIDSupplied,
    /// Pet not found
    Status404_PetNotFound,
    /// Unexpected error
    Status0_UnexpectedError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UpdatePetResponse {
    /// Successful operation
    Status200_SuccessfulOperation(models::Pet),
    /// Invalid ID supplied
    Status400_InvalidIDSupplied,
    /// Pet not found
    Status404_PetNotFound,
    /// Validation exception
    Status422_ValidationException,
    /// Unexpected error
    Status0_UnexpectedError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UpdatePetWithFormResponse {
    /// successful operation
    Status200_SuccessfulOperation(models::Pet),
    /// Invalid input
    Status400_InvalidInput,
    /// Unexpected error
    Status0_UnexpectedError(models::Error),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum UploadFileResponse {
    /// successful operation
    Status200_SuccessfulOperation(models::ApiResponse),
    /// No file uploaded
    Status400_NoFileUploaded,
    /// Pet not found
    Status404_PetNotFound,
    /// Unexpected error
    Status0_UnexpectedError(models::Error),
}

/// Pet
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Pet<E: std::fmt::Debug + Send + Sync + 'static = ()>: super::ErrorHandler<E> {
    type Claims;

    /// Add a new pet to the store..
    ///
    /// AddPet - POST /api/v3/pet
    async fn add_pet(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::Pet,
    ) -> Result<AddPetResponse, E>;

    /// Deletes a pet..
    ///
    /// DeletePet - DELETE /api/v3/pet/{petId}
    async fn delete_pet(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        header_params: &models::DeletePetHeaderParams,
        path_params: &models::DeletePetPathParams,
    ) -> Result<DeletePetResponse, E>;

    /// Finds Pets by status..
    ///
    /// FindPetsByStatus - GET /api/v3/pet/findByStatus
    async fn find_pets_by_status(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::FindPetsByStatusQueryParams,
    ) -> Result<FindPetsByStatusResponse, E>;

    /// Finds Pets by tags..
    ///
    /// FindPetsByTags - GET /api/v3/pet/findByTags
    async fn find_pets_by_tags(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        query_params: &models::FindPetsByTagsQueryParams,
    ) -> Result<FindPetsByTagsResponse, E>;

    /// Find pet by ID..
    ///
    /// GetPetById - GET /api/v3/pet/{petId}
    async fn get_pet_by_id(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        claims: &Self::Claims,
        path_params: &models::GetPetByIdPathParams,
    ) -> Result<GetPetByIdResponse, E>;

    /// Update an existing pet..
    ///
    /// UpdatePet - PUT /api/v3/pet
    async fn update_pet(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        body: &models::Pet,
    ) -> Result<UpdatePetResponse, E>;

    /// Updates a pet in the store with form data..
    ///
    /// UpdatePetWithForm - POST /api/v3/pet/{petId}
    async fn update_pet_with_form(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        path_params: &models::UpdatePetWithFormPathParams,
        query_params: &models::UpdatePetWithFormQueryParams,
    ) -> Result<UpdatePetWithFormResponse, E>;

    /// Uploads an image..
    ///
    /// UploadFile - POST /api/v3/pet/{petId}/uploadImage
    async fn upload_file(
        &self,
        method: &Method,
        host: &Host,
        cookies: &CookieJar,
        path_params: &models::UploadFilePathParams,
        query_params: &models::UploadFileQueryParams,
        body: &Bytes,
    ) -> Result<UploadFileResponse, E>;
}
