use std::error::Error as StdError;
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

// Define a custom trait that extends both Error and Any
trait ErrorExt: StdError + Any + Send + Sync {
    fn as_error(&self) -> &(dyn StdError + Send + Sync);
}


// Implement it for all types that implement both Error and Any
impl<T: StdError + Any + Send + Sync> ErrorExt for T {
    fn as_error(&self) -> &(dyn StdError + Send + Sync) {
        self
    }
}
// Define the error mapper function type
type ErrorMapperFn = Box<dyn Fn(&(dyn ErrorExt + 'static))
    -> (Status, Json<GenericResponse>) + Send + Sync>;



lazy_static! {
    static ref ERROR_MAPPER: HashMap<TypeId, ErrorMapperFn> = {
        let mut error_map = HashMap::new();

        let commerce_handler: ErrorMapperFn = Box::new(|error| {
            let commerce_error = error.downcast_ref::<CommerceError>().unwrap();
            (
                Status::BadRequest,
                Json(GenericResponse::new(
                    String::from(commerce_error.get_code()),
                    String::from("ERROR"),
                    String::from(commerce_error.get_message())
                ))
            )
        });

        let bank_handler: ErrorMapperFn = Box::new(|error| {
            let bank_error = error.downcast_ref::<BankError>().unwrap();
            (
                Status::BadRequest,
                Json(GenericResponse::new(
                    String::from(bank_error.get_code()),
                    String::from("ERROR"),
                    String::from(bank_error.get_message())
                ))
            )
        });

        let db_handler: ErrorMapperFn = Box::new(|error| {
            let db_error = error.downcast_ref::<DatabaseError>().unwrap();
            (
                Status::ServiceUnavailable,
                Json(GenericResponse::new(
                    String::from("ERR-UNKNOWN"),
                    String::from("ERROR"),
                    String::from(db_error.get_message())
                ))
            )
        });

        error_map.insert(TypeId::of::<CommerceError>(), commerce_handler);
        error_map.insert(TypeId::of::<BankError>(), bank_handler);
        error_map.insert(TypeId::of::<DatabaseError>(), db_handler);

        error_map
    };
}

fn map_errors_to_responses(error: Box<dyn StdError + Send + Sync>)
    -> Result<(Status, Json<Commerce>), (Status, Json<GenericResponse>)> {
    // Use as_any() to get the TypeId
    let error = error.as_ref();
    let type_id = Any::type_id(error);

    if let Some(mapper_fn) = ERROR_MAPPER.get(&type_id) {
        let error = &error as &(dyn ErrorExt + 'static);
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