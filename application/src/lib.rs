pub mod port {
    pub mod db {
        pub mod banks {
            pub mod bank_repository_port;
        }
        pub mod commerces {
            pub mod commerce_repository_port;
        }
    }
}
pub mod service {
    pub mod commerces{
        pub mod create_commerce_service;
        pub mod validate_commerce_to_store_service;
    }
}
pub mod use_case {
    pub mod commerces {
        pub mod create_commerce_use_case;
        pub mod validate_commerce_to_store;
    }
}
