use egui::{Color32, RichText};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(thiserror::Error, Debug)]
pub enum GeometryApiError {
    #[error("Failed fetching geometries")]
    RequestFailed(#[from] ureq::Error),
    #[error("Url parsing failed")]
    UrlParsing(#[from] url::ParseError),
    #[error("Request failed: {0}")]
    BadRequest(&'static str),
    #[error("Async request failed")]
    #[cfg(feature = "async")]
    AsyncRequestFailed(#[from] reqwest::Error),
}

pub enum ApiHealth {
    Ok,
    Error(String),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GeometryApi {
    pub api_url: String,
}

impl GeometryApi {
    pub fn new() -> Self {
        Self {
            api_url: String::from("http://localhost:8000"),
        }
    }

    pub fn healtchcheck_url(&self) -> Result<String, GeometryApiError> {
        let mut url = Url::parse(&self.api_url)?;
        url.path_segments_mut().unwrap().push("healthcheck");

        Ok(url.to_string())
    }

    pub fn test_connection(&self) -> Result<ApiHealth, GeometryApiError> {
        let url = self.healtchcheck_url()?;
        let req = ureq::get(&url);
        let response = req.call()?;
        match response.status() {
            200 => Ok(ApiHealth::Ok),
            _ => Ok(ApiHealth::Error(response.status_text().to_string())),
        }
    }

    pub fn connection_status(&self) -> RichText {
        match self.test_connection() {
            Ok(api_health) => match api_health {
                ApiHealth::Ok => RichText::new("OK!").color(Color32::GREEN),
                ApiHealth::Error(e) => RichText::new(format!("Error: {e}")).color(Color32::RED),
            },
            Err(err) => RichText::new(format!("Application error: {err}")).color(Color32::RED),
        }
    }
}
