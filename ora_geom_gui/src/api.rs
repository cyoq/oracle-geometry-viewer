use egui::{Color32, RichText};
use serde::{Deserialize, Serialize};

pub enum ApiHealth {
    Ok,
    Error(String),
    Unknown,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GeometryApi {
    pub api_url: String,
}

impl GeometryApi {
    pub fn new() -> Self {
        Self {
            api_url: String::from(""),
        }
    }

    pub fn test_connection(&self) -> ApiHealth {
        ApiHealth::Ok
    }

    pub fn connection_status(&self) -> RichText {
        match self.test_connection() {
            ApiHealth::Ok => RichText::new("OK!").color(Color32::GREEN),
            ApiHealth::Error(e) => RichText::new(format!("Error: {e}")).color(Color32::RED),
            ApiHealth::Unknown => RichText::new(""),
        }
    }
}
