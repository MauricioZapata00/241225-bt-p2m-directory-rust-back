#[macro_use]
extern crate rocket;

use rocket::{State, http::Status, response::status, serde::json::Json};
use domain::model::commerce::Commerce;
use infrastructure::db::mssql::banks::repository::bank_repository::SqlxBankRepository;
use infrastructure::db::mssql::commerces::repository::account_repository::SqlxAccountRepository;
use infrastructure::db::mssql::commerces::repository::commerce_repository::SqlxCommerceRepository;
use infrastructure::db::mssql::commerces::repository::commerce_status_repository::SqlxCommerceStatusRepository;
use infrastructure::entrypoint::commerces::commerce_controller::CommerceController;

struct AppState {
    sqlx_account_repository: SqlxAccountRepository,
    sqlx_bank_repository: SqlxBankRepository,
    sqlx_commerce_status_repository: SqlxCommerceStatusRepository,
    sqlx_commerce_repository: SqlxCommerceRepository
}

impl AppState {
    async fn new() -> Self {

    }
}
#[get("/entrypoint")]
async fn list_commerces(controller: &State<CommerceController>) -> Result<Json<Vec<Commerce>>, Status> {
    controller.get_commerce().await
}

#[launch]
async fn rocket() -> rocket::Rocket<rocket::Build> {

    let figment = rocket::Config::figment()
        .merge(("port", 8008));
    let commerce_controller = CommerceController::new();

    rocket::custom(figment)
        .manage(commerce_controller)
        .mount("/", routes![list_commerces])
}
