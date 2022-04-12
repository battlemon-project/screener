pub type GenericError = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, GenericError>;

pub mod config;
pub mod routes;
pub mod startup;
pub mod telemetry;