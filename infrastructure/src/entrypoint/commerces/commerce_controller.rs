use std::error::Error;
use rocket::{http::Status, serde::json::Json};
use std::any::Any;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::any::TypeId;
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
            Err(error_storing_commerce) => map_errors_to_responses(error_storing_commerce)
        }
    }
}

lazy_static! {
    static ref ERROR_MAPPER: HashMap<TypeId, fn(Box<dyn Error + Send + Sync>)
    -> (Status, Json<GenericResponse>)> = {
        let mut error_map = HashMap::new();

        error_map.insert(
            TypeId::of::<CommerceError>(),
            |error| {
                let commerce_error = error.downcast_ref::<CommerceError>().unwrap();
                (
                    Status::BadRequest,
                    Json(GenericResponse::new(
                        commerce_error.get_code(),
                        String::from("ERROR"),
                        commerce_error.get_message()
                    ))
                )
            }
        );

        error_map.insert(
            TypeId::of::<BankError>(),
            |error| {
                let bank_error = error.downcast_ref::<BankError>().unwrap();
                (
                    Status::BadRequest,
                    Json(GenericResponse::new(
                        bank_error.get_code(),
                        String::from("ERROR"),
                        bank_error.get_message()
                    ))
                )
            }
        );

        error_map.insert(
            TypeId::of::<DatabaseError>(),
            |error| {
                let db_error = error.downcast_ref::<DatabaseError>().unwrap();
                (
                    Status::ServiceUnavailable,
                    Json(GenericResponse::new(
                        String::from("ERR-UNKNOWN"),
                        String::from("ERROR"),
                        db_error.get_message()
                    ))
                )
            }
        );

        error_map
    };
}

fn map_errors_to_responses(error: Box<dyn Error>)
    -> Result<(Status, Json<Commerce>), (Status, Json<GenericResponse>)> {
    // Use as_any() to get the TypeId
    let type_id = error.as_any().type_id();

    if let Some(mapper_fn) = ERROR_MAPPER.get(&type_id) {
        let (status, response) = mapper_fn(error);
        Err((status, response))
    } else {
        // Default case for unknown errors
        Err((
            Status::InternalServerError,
            Json(GenericResponse::new(
                String::from("INTERNAL_ERROR"),
                String::from("ERROR"),
                String::from("An unexpected error occurred")
            ))
        ))
    }
}