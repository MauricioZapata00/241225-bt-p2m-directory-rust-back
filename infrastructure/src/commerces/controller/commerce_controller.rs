use rocket::{http::Status, serde::json::Json};
use uuid::Uuid;
use domain::model::commerce::Commerce;
use domain::model::account::Account;
use domain::model::commerce_status::CommerceStatus;

pub struct CommerceController {
}

impl CommerceController {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn get_commerce(&self) -> Result<Json<Vec<Commerce>>, Status> {
        let random_v4_id_generator = Uuid::new_v4();
        Ok(Json::from(vec![Commerce::new(1,
                                         String::from("@JabonSucio"),
                                         1,
                                         String::from("Comercios_tontos"),
                                         Account::new(String::from("8526489"), String::from("K7015afM1"), 5),
                                         random_v4_id_generator.to_string(),
                                         CommerceStatus::new(String::from("ACTIVE")))]))
        // match self.service.get_all_todos().await {
        //     Ok(todos) => Ok(Json(todos)),
        //     Err(_) => Err(Status::InternalServerError),
        // }
    }
}