#[macro_use]
extern crate rocket;

use std::env;
use std::sync::Arc;
use std::time::Duration;
use rocket::{State, http::Status, serde::json::Json};
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use rocket::figment::Figment;
use tracing_subscriber::FmtSubscriber;
use tracing::{info, Level};
use application::service::commerces::create_commerce_service::CreateCommerceService;
use application::service::commerces::validate_commerce_to_store_service::ValidateCommerceToStoreService;

use domain::model::commerce::Commerce;
use domain::model::generic_response::GenericResponse;
use infrastructure::db::mysql::banks::adapter::bank_repository_adapter::BankRepositoryAdapter;
use infrastructure::db::mysql::banks::repository::bank_repository::SqlxBankRepository;
use infrastructure::db::mysql::commerces::adapter::commerce_repository_adapter::CommerceRepositoryAdapter;
use infrastructure::db::mysql::commerces::repository::account_repository::SqlxAccountRepository;
use infrastructure::db::mysql::commerces::repository::commerce_repository::SqlxCommerceRepository;
use infrastructure::db::mysql::commerces::repository::commerce_status_repository::SqlxCommerceStatusRepository;
use infrastructure::entrypoint::commerces::commerce_controller::CommerceController;
use infrastructure::entrypoint::commerces::dto::commerce_dto::CommerceDto;

type BankRepo = BankRepositoryAdapter;
type CommerceRepo = CommerceRepositoryAdapter;

type ValidateService = ValidateCommerceToStoreService<BankRepo, CommerceRepo>;

type CreateService = CreateCommerceService<ValidateService, CommerceRepo>;

type AppCommerceController = CommerceController<CreateService>;

struct AppState {
    commerce_controller: AppCommerceController
}

async fn create_db_pool(figment: &Figment) -> Result<MySqlPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        figment
            .extract_inner("database_url")
            .expect("database_url configuration missing")
    });

    MySqlPoolOptions::new()
        .max_connections(5)               // Maximum number of connections in the pool
        .min_connections(1)                // Minimum number of idle connections to maintain
        .acquire_timeout(Duration::from_secs(3))     // Maximum time to wait for a connection
        .idle_timeout(Duration::from_secs(8))        // How long to keep an idle connection
        .max_lifetime(Duration::from_secs(30))       // Maximum lifetime of a connection
        .connect(&database_url)
        .await
}

impl AppState {
    async fn new() -> Self {
        let figment = rocket::Config::figment();



        let pool = create_db_pool(&figment)
            .await
            .expect("Failed to create pool");

        let pool_arc = Arc::from(pool);

        let account_repo = SqlxAccountRepository::new(pool_arc.clone());
        let bank_repo = SqlxBankRepository::new(pool_arc.clone());
        let commerce_status_repo = SqlxCommerceStatusRepository::new(
            pool_arc.clone());
        let account_repo_arc = Arc::from(account_repo);
        let bank_repo_arc = Arc::from(bank_repo);
        let commerce_status_repo_arc = Arc::from(commerce_status_repo);

        let commerce_repo = SqlxCommerceRepository::new(pool_arc.clone(),
        account_repo_arc.clone(), bank_repo_arc.clone(), commerce_status_repo_arc.clone());
        let commerce_repo_arc = Arc::from(commerce_repo);

        let bank_repository_adapter = BankRepositoryAdapter::new(
            bank_repo_arc.clone());
        let bank_repository_adapter_arc = Arc::from(bank_repository_adapter);
        let commerce_repository_adapter = CommerceRepositoryAdapter::new(
            commerce_repo_arc.clone());
        let commerce_repository_adapter_arc = Arc::from(
            commerce_repository_adapter);

        let validate_commerce_to_store_use_case = ValidateCommerceToStoreService::new(
            bank_repository_adapter_arc.clone(),
            commerce_repository_adapter_arc.clone(),
        );

        let validate_commerce_to_store_service_arc = Arc::from(
            validate_commerce_to_store_use_case);

        let create_commerce_use_case = CreateCommerceService::new(
            validate_commerce_to_store_service_arc.clone(),
            commerce_repository_adapter_arc.clone(),
        );
        let create_commerce_use_case_arc = Arc::from(
            create_commerce_use_case);

        let commerce_controller = CommerceController::new(
            create_commerce_use_case_arc.clone());

        Self {
            commerce_controller
        }

    }
}
#[post("/commerces", format = "json", data = "<commerce>")]
async fn store_commerce(state: &State<AppState>, commerce: Json<CommerceDto>)
    -> Result<(Status, Json<Commerce>), (Status, Json<GenericResponse>)> {
    state.commerce_controller.create_commerce(commerce.into_inner()).await
}

#[launch]
async fn rocket() -> _ {


    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting default subscriber failed");
    info!("Logging initialized");
    info!("Initializing app...");


    let state = AppState::new().await;
    let config = rocket::Config::figment()
        .merge(("port", 8008))
        .merge(("address", "0.0.0.0"));

    rocket::custom(config)
        .manage(state)
        .mount("/api", routes![store_commerce])
}
