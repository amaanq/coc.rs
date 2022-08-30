/// Base API wrapper
pub mod api;
mod models;
pub use models::*;

/// Clash of Stats API wrapper
mod cos;
pub use cos::*;
mod cos_models;
pub use cos_models::*;

pub mod credentials;
/// Developer Site API wrapper
mod dev;

mod tests;
mod tests_cos;
