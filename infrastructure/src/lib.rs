pub mod db {
    pub mod mssql {
        pub mod banks {
            pub mod adapter {
                pub mod bank_repository_adapter;
            }
            pub mod entity {
                pub mod bank_entity;
                pub mod bank_status_entity;
            }
            pub mod repository {
                pub mod bank_repository;
            }
        }
        pub mod commerces {
            pub mod adapter {
                pub mod commerce_repository_adapter;
            }
            pub mod entity {
                pub mod wrappers {
                    pub mod commerce_db_info_wrapper;
                }
                pub mod account_entity;
                pub mod alias_type_entity;
                pub mod commerce_entity;
                pub mod commerce_status_entity;
            }
            pub mod repository {
                pub mod account_repository;
                pub mod commerce_repository;
                pub mod commerce_status_repository;
            }
        }
    }
}

pub mod entrypoint {
    pub mod controller {
        pub mod commerces {
            pub mod dto {
                pub mod account_dto;
                pub mod commerce_dto;
            }
            pub mod commerce_controller;
        }
    }
}
