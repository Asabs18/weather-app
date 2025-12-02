// src/repositories/weather_repository.rs

use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::Value;

use crate::models::weather_info::{WeatherData, WeatherInfo};

/// Response structure from Open-Meteo weather API
#[derive(Debug, Deserialize)]
struct OpenMeteoWeather {
    current: OpenMeteoCurrent,
}

/// Current weather conditions from Open-Meteo API
#[derive(Debug, Deserialize)]
struct OpenMeteoCurrent {
    /// Temperature at 2 meters above ground in Celsius
    temperature_2m: f64,
}

/// Trait for fetching weather data from various sources
pub trait WeatherRepository {
    /// Fetches weather information for the given location
    fn fetch_weather(&self, location: &str) -> Result<WeatherInfo, String>;
}

/// Real implementation of WeatherRepository using OpenStreetMap and Open-Meteo APIs
pub struct ApiWeatherRepository {
    client: Client,
}

impl ApiWeatherRepository {
    /// Creates a new ApiWeatherRepository with a configured HTTP client
    pub fn new() -> Self {
        ApiWeatherRepository {
            client: Client::new(),
        }
    }

    /// Fetches coordinates (latitude, longitude) for a location using OpenStreetMap Nominatim
    fn fetch_coordinates(&self, location: &str) -> Result<(f64, f64), String> {
        let location = location.trim();
        let url = format!(
            "https://nominatim.openstreetmap.org/search?q={}&format=json&limit=1",
            urlencoding::encode(location)
        );

        let response = self
            .client
            .get(&url)
            .header("User-Agent", "RustWeatherApp/1.0")
            .send()
            .map_err(|e| format!("Failed to fetch coordinates: {}", e))?;

        if !response.status().is_success() {
            return Err(format!(
                "Geocoding API returned status: {}",
                response.status()
            ));
        }

        let json: Value = response
            .json()
            .map_err(|e| format!("Failed to parse coordinates response: {}", e))?;

        let array = json
            .as_array()
            .ok_or_else(|| "Invalid response format from geocoding API".to_string())?;

        if array.is_empty() {
            return Err(format!("Location '{}' not found", location));
        }

        let first_result = &array[0];
        let lat = first_result["lat"]
            .as_str()
            .and_then(|s| s.parse::<f64>().ok())
            .ok_or_else(|| "Failed to parse latitude".to_string())?;
        let lon = first_result["lon"]
            .as_str()
            .and_then(|s| s.parse::<f64>().ok())
            .ok_or_else(|| "Failed to parse longitude".to_string())?;

        Ok((lat, lon))
    }
}

impl WeatherRepository for ApiWeatherRepository {
    fn fetch_weather(&self, location: &str) -> Result<WeatherInfo, String> {
        // Get coordinates for the location
        let (lat, lon) = self.fetch_coordinates(location)?;

        // Fetch weather data from Open-Meteo API
        let url = format!(
            "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m",
            lat, lon
        );

        let weather: OpenMeteoWeather = self
            .client
            .get(&url)
            .send()
            .map_err(|e| format!("Failed to fetch weather: {}", e))?
            .json()
            .map_err(|e| format!("Failed to parse weather data: {}", e))?;

        let weather_data = WeatherData {
            temperature: Some(weather.current.temperature_2m),
        };

        Ok(WeatherInfo::new(location.to_string(), weather_data))
    }
}
