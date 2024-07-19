/*
 * OpenAPI Petstore
 *
 * This is a sample server Petstore server. For this sample, you can use the api key `special-key` to test the authorization filters.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use std::sync::Arc;
use std::borrow::Borrow;
use std::pin::Pin;
#[allow(unused_imports)]
use std::option::Option;

use hyper;
use hyper_util::client::legacy::connect::Connect;
use futures::Future;

use crate::models;
use super::{Error, configuration};
use super::request as __internal_request;

pub struct PetApiClient<C: Connect>
    where C: Clone + std::marker::Send + Sync + 'static {
    configuration: Arc<configuration::Configuration<C>>,
}

impl<C: Connect> PetApiClient<C>
    where C: Clone + std::marker::Send + Sync {
    pub fn new(configuration: Arc<configuration::Configuration<C>>) -> PetApiClient<C> {
        PetApiClient {
            configuration,
        }
    }
}

pub trait PetApi: Send {
    fn add_pet(&self, pet: models::Pet) -> Pin<Box<dyn Future<Output = Result<models::Pet, Error>> + Send>>;
    fn delete_pet(&self, pet_id: i64, api_key: Option<&str>) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
    fn find_pets_by_status(&self, status: Vec<String>) -> Pin<Box<dyn Future<Output = Result<Vec<models::Pet>, Error>> + Send>>;
    fn find_pets_by_tags(&self, tags: Vec<String>) -> Pin<Box<dyn Future<Output = Result<Vec<models::Pet>, Error>> + Send>>;
    fn get_pet_by_id(&self, pet_id: i64) -> Pin<Box<dyn Future<Output = Result<models::Pet, Error>> + Send>>;
    fn update_pet(&self, pet: models::Pet) -> Pin<Box<dyn Future<Output = Result<models::Pet, Error>> + Send>>;
    fn update_pet_with_form(&self, pet_id: i64, name: Option<&str>, status: Option<&str>) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>;
    fn upload_file(&self, pet_id: i64, additional_metadata: Option<&str>, file: Option<std::path::PathBuf>) -> Pin<Box<dyn Future<Output = Result<models::ApiResponse, Error>> + Send>>;
}

impl<C: Connect>PetApi for PetApiClient<C>
    where C: Clone + std::marker::Send + Sync {
    #[allow(unused_mut)]
    fn add_pet(&self, pet: models::Pet) -> Pin<Box<dyn Future<Output = Result<models::Pet, Error>> + Send>> {
        let mut req = __internal_request::Request::new(hyper::Method::POST, "/pet".to_string())
            .with_auth(__internal_request::Auth::Oauth)
        ;
        req = req.with_body_param(pet);

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn delete_pet(&self, pet_id: i64, api_key: Option<&str>) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let mut req = __internal_request::Request::new(hyper::Method::DELETE, "/pet/{petId}".to_string())
            .with_auth(__internal_request::Auth::Oauth)
        ;
        req = req.with_path_param("petId".to_string(), pet_id.to_string());
        if let Some(param_value) = api_key {
            req = req.with_header_param("api_key".to_string(), param_value.to_string());
        }
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn find_pets_by_status(&self, status: Vec<String>) -> Pin<Box<dyn Future<Output = Result<Vec<models::Pet>, Error>> + Send>> {
        let mut req = __internal_request::Request::new(hyper::Method::GET, "/pet/findByStatus".to_string())
            .with_auth(__internal_request::Auth::Oauth)
        ;
        req = req.with_query_param("status".to_string(), status.join(",").to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn find_pets_by_tags(&self, tags: Vec<String>) -> Pin<Box<dyn Future<Output = Result<Vec<models::Pet>, Error>> + Send>> {
        let mut req = __internal_request::Request::new(hyper::Method::GET, "/pet/findByTags".to_string())
            .with_auth(__internal_request::Auth::Oauth)
        ;
        req = req.with_query_param("tags".to_string(), tags.join(",").to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn get_pet_by_id(&self, pet_id: i64) -> Pin<Box<dyn Future<Output = Result<models::Pet, Error>> + Send>> {
        let mut req = __internal_request::Request::new(hyper::Method::GET, "/pet/{petId}".to_string())
            .with_auth(__internal_request::Auth::ApiKey(__internal_request::ApiKey{
                in_header: true,
                in_query: false,
                param_name: "api_key".to_owned(),
            }))
        ;
        req = req.with_path_param("petId".to_string(), pet_id.to_string());

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn update_pet(&self, pet: models::Pet) -> Pin<Box<dyn Future<Output = Result<models::Pet, Error>> + Send>> {
        let mut req = __internal_request::Request::new(hyper::Method::PUT, "/pet".to_string())
            .with_auth(__internal_request::Auth::Oauth)
        ;
        req = req.with_body_param(pet);

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn update_pet_with_form(&self, pet_id: i64, name: Option<&str>, status: Option<&str>) -> Pin<Box<dyn Future<Output = Result<(), Error>> + Send>> {
        let mut req = __internal_request::Request::new(hyper::Method::POST, "/pet/{petId}".to_string())
            .with_auth(__internal_request::Auth::Oauth)
        ;
        req = req.with_path_param("petId".to_string(), pet_id.to_string());
        if let Some(param_value) = name {
            req = req.with_form_param("name".to_string(), param_value.to_string());
        }
        if let Some(param_value) = status {
            req = req.with_form_param("status".to_string(), param_value.to_string());
        }
        req = req.returns_nothing();

        req.execute(self.configuration.borrow())
    }

    #[allow(unused_mut)]
    fn upload_file(&self, pet_id: i64, additional_metadata: Option<&str>, file: Option<std::path::PathBuf>) -> Pin<Box<dyn Future<Output = Result<models::ApiResponse, Error>> + Send>> {
        let mut req = __internal_request::Request::new(hyper::Method::POST, "/pet/{petId}/uploadImage".to_string())
            .with_auth(__internal_request::Auth::Oauth)
        ;
        req = req.with_path_param("petId".to_string(), pet_id.to_string());
        if let Some(param_value) = additional_metadata {
            req = req.with_form_param("additionalMetadata".to_string(), param_value.to_string());
        }
        if let Some(param_value) = file {
            req = req.with_form_param("file".to_string(), unimplemented!());
        }

        req.execute(self.configuration.borrow())
    }

}
