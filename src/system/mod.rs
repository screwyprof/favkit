pub mod api;
pub mod favorites;
pub mod macos;

pub use self::favorites::api::Favorites;
pub use self::macos::api::RealMacOsApi;