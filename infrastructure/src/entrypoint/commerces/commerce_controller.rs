use std::error::Error;
use rocket::{http::Status, serde::json::Json};
use std::sync::Arc;
use application::use_case::commerces::create_commerce_use_case::CreateCommerceUseCase;
use domain::exception::bank_error::BankError;
use domain::exception::commerce_error::CommerceError;
use domain::exception::database_error::DatabaseError;
use domain::model::commerce::Commerce;
use domain::model::generic_response::GenericResponse;
use crate::entrypoint::commerces::dto::commerce_dto::CommerceDto;

pub struct CommerceController<CC: CreateCommerceUseCase> {
    create_commerce_use_case: Arc<CC>
}

impl<CC: CreateCommerceUseCase> CommerceController<CC> {
    pub fn new(create_commerce_use_case: Arc<CC>) -> Self {
        Self {
            create_commerce_use_case
        }
    }

    pub async fn create_commerce(&self, commerce_dto: CommerceDto)
        -> Result<(Status, Json<Commerce>), (Status, Json<GenericResponse>)> {
        let commerce = commerce_dto.to_domain().map_err(|commerce_error| {
            let error_response = GenericResponse::new(
                String::from(commerce_error.get_code()),
                String::from("ERROR"),
                String::from(commerce_error.get_message())
            );
            (Status::BadRequest, Json(error_response))
        })?;
        match self.create_commerce_use_case.process(commerce).await {
            Ok(inserted_commerce) => Ok((Status::Created, Json::from(inserted_commerce))),
            Err(error_storing_commerce) => map_errors_to_responses(
                error_storing_commerce)
        }
    }


}


fn map_errors_to_responses(error_storing_commerce: Box<dyn Error + Send + Sync>) -> Result<(Status, Json<Commerce>), (Status, Json<GenericResponse>)> {
    if let Some(commerce_error) = error_storing_commerce.downcast_ref::<CommerceError>() {
        Err((Status::BadRequest, Json(GenericResponse::new(
            String::from(commerce_error.get_code()),
            String::from("ERROR"),
            String::from(commerce_error.get_message())
        ))))
    } else if let Some(bank_error) = error_storing_commerce.downcast_ref::<BankError>() {
        Err((Status::BadRequest, Json(GenericResponse::new(
            String::from(bank_error.get_code()),
            String::from("ERROR"),
            String::from(bank_error.get_message())
        ))))
    } else if let Some(db_error) = error_storing_commerce.downcast_ref::<DatabaseError>() {
        Err((Status::ServiceUnavailable, Json(GenericResponse::new(
            String::from("ERR-UNKNOWN"),
            String::from("ERROR"),
            String::from(db_error.get_message())
        ))))
    } else {
        // Unknown error type
        Err((Status::InternalServerError, Json(GenericResponse::new(
            String::from("INTERNAL-ERROR"),
            String::from("ERROR"),
            String::from("An unexpected error occurred")
        ))))
    }
}