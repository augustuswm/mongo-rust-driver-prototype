#[macro_use]
mod macros;

pub mod crud;
pub mod eq;
pub mod sdam;
pub mod server_selection;

use serde_json::Map;
use serde_json::Value as Json;

pub trait FromJson: Sized {
    fn from_json(object: &Map<String, Json>) -> Self;
}

pub trait FromJsonResult: Sized {
    fn from_json(object: &Map<String, Json>) -> Result<Self, String>;
}
