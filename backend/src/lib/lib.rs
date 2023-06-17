pub mod database;
pub mod models;
pub mod schema;

mod model_implementations {
    use super::models::*;

    pub mod invoice;
    pub mod invoice_proof;
    pub mod user;
}
