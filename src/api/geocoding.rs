use crate::models::Location;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GeocodingResponse {
    name: String,
    country: String,
    lat: f64,
    lon: f64,
}

pub struct GeocodingClient {
    api_key: String,
    client: reqwest::Client,
}

impl GeocodingClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
        }
    }

    pub async fn search_city(&self, query: &str) -> Result<Vec<Location>, String> {
        let url = format!(
            "http://api.openweathermap.org/geo/1.0/direct?q={}&limit=5&appid={}",
            query, self.api_key
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API error: {}", response.status()));
        }

        let results: Vec<GeocodingResponse> = response
            .json()
            .await
            .map_err(|e| format!("Parse error: {}", e))?;

        Ok(results
            .into_iter()
            .map(|r| Location {
                name: r.name,
                country: r.country,
                lat: r.lat,
                lon: r.lon,
            })
            .collect())
    }
}
