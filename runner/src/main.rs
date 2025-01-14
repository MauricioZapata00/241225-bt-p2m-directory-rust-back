#[macro_use]
extern crate rocket;

use rocket::{State, http::Status, response::status, serde::json::Json};
use domain::model::commerce::Commerce;
use infrastructure::commerces::controller::commerce_controller::CommerceController;

#[get("/commerces")]
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
