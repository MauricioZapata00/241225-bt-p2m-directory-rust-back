use std::error::Error;
use mockall::predicate::*;
use mockall::mock;
use std::sync::Arc;
use async_trait::async_trait;
use domain::exception::database_error::DatabaseError;
use domain::model::account::Account;
use domain::model::commerce::Commerce;
use domain::model::commerce_status::CommerceStatus;
use crate::port::db::commerces::commerce_repository_port::CommerceRepositoryPort;
use crate::use_case::commerces::validate_commerce_to_store::ValidateCommerceToStore;
use crate::service::commerces::create_commerce_service::CreateCommerceService;
use crate::use_case::commerces::create_commerce_use_case::CreateCommerceUseCase;

// Create mocks for our dependencies
mock! {
    ValidateCommerceUseCase {}

    #[async_trait]
    impl ValidateCommerceToStore for ValidateCommerceUseCase {
        async fn process(&self, commerce: Commerce) -> Result<Commerce, Box<dyn Error + Send + Sync>>;
    }
}

mock! {
    CommerceRepo {}

    #[async_trait]
    impl CommerceRepositoryPort for CommerceRepo {
        async fn create_commerce(&self, commerce: &Commerce)
        -> Result<Commerce, DatabaseError>;

        async fn commerce_does_not_exist_by_ruc_and_alias(
            &self,
            ruc: &String,
            alias: &String
        ) -> Result<bool, DatabaseError>;

        async fn commerce_exists_by_ruc_or_legal_business_name(
            &self,
            ruc: &String,
            legal_business_name: &String
        ) -> Result<bool, DatabaseError>;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_process_success() {
        let input_commerce = create_commerce_for_test(); // Create a test commerce
        let validated_commerce = create_commerce_for_test(); // Create expected validated commerce
        let created_commerce = create_commerce_for_test(); // Create expected created commerce

        // Clone validated_commerce before it's moved
        let validated_commerce_clone = validated_commerce.clone();

        let mut validator = MockValidateCommerceUseCase::new();
        let mut repository = MockCommerceRepo::new();

        // Setup validator mock
        validator
            .expect_process()
            .with(eq(input_commerce.clone()))
            .returning(move |_| Ok(validated_commerce.clone()))
            .times(1);

        // Setup repository mock
        repository
            .expect_create_commerce()
            .with(eq(validated_commerce_clone))
            .returning(move |_| Ok(created_commerce.clone()))
            .times(1);

        // Create service with configured mocks
        let service = CreateCommerceService::new(
            Arc::new(validator),
            Arc::new(repository)
        );

        // Execute test
        let result = service.process(input_commerce).await;

        assert!(result.is_ok());
        // Add more specific assertions about the returned commerce
    }

    #[tokio::test]
    async fn test_process_validation_error() {
        let input_commerce = create_commerce_for_test();

        let mut validator = MockValidateCommerceUseCase::new();
        let mut repository = MockCommerceRepo::new();
        // Setup validator mock to return error
        validator
            .expect_process()
            .with(eq(input_commerce.clone()))
            .returning(|_| Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Validation error"
            ))))
            .times(1);


        // Repository should not be called
        repository.expect_create_commerce().times(0);

        // Create service with configured mocks
        let service = CreateCommerceService::new(
            Arc::new(validator),
            Arc::new(repository)
        );

        // Execute test
        let result = service.process(input_commerce).await;

        assert!(result.is_err());
        // Add assertions about the error type/message
    }

    #[tokio::test]
    async fn test_process_repository_error() {
        let input_commerce = create_commerce_for_test();
        let validated_commerce = create_commerce_for_test();

        // Clone validated_commerce before it's moved
        let validated_commerce_clone = validated_commerce.clone();

        let mut validator = MockValidateCommerceUseCase::new();
        let mut repository = MockCommerceRepo::new();

        // Setup validator mock
        validator
            .expect_process()
            .with(eq(input_commerce.clone()))
            .returning(move |_| Ok(validated_commerce.clone()))
            .times(1);

        // Setup repository mock to return error
        repository
            .expect_create_commerce()
            .with(eq(validated_commerce_clone))
            .returning(|_| Err(DatabaseError::Unexpected(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Database error in test"
            )))))
            .times(1);

        // Create service with configured mocks
        let service = CreateCommerceService::new(
            Arc::new(validator),
            Arc::new(repository)
        );

        // Execute test
        let result = service.process(input_commerce).await;

        assert!(result.is_err());
        // Add assertions about the error type/message
    }
}

fn create_commerce_for_test() -> Commerce {
    Commerce::new(
        50,
        String::from("aliasTest"),
        2,
        String::from("legal business name test"),
        Account::new(
            59,
            String::from("84102e21-01b5-4f5d-8771-36f915c4d29e"),
            String::from("841"),
            10
        ),
        String::from("123456789-9-2099"),
        CommerceStatus::new(
            String::from("ACTIVE"),
        )
    )
}