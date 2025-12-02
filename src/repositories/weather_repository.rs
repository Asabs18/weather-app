// src/repositories/weather_repository.rs

use serde::Deserialize;

use crate::models::weather_info::{WeatherData, WeatherInfo};

#[derive(Debug, Deserialize)]
struct OpenMeteoWeather {
    current: OpenMeteoCurrent,
}

#[derive(Debug, Deserialize)]
struct OpenMeteoCurrent {
    temperature_2m: f64,
}

// Trait for fetching weather data from various sources
pub trait WeatherRepository {
    // Fetches weather information for the given location
    fn fetch_weather(&self, location: &str) -> Result<WeatherInfo, String>;
}

// Mock implementation of WeatherRepository with hardcoded data
pub struct MockWeatherRepository;

impl WeatherRepository for MockWeatherRepository {
    fn fetch_weather(&self, location: &str) -> Result<WeatherInfo, String> {
        // Get coordinates for the location
        let (lat, lon) = match location.to_lowercase().as_str() {
            "london" => (51.51, -0.13),
            "boston" => (42.36, -71.06),
            _ => return Err(format!("No data available for {}", location)),
        };
        
        let url = format!(
            "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m",
            lat, lon
        );

        let weather: OpenMeteoWeather = reqwest::blocking::get(&url)
            .map_err(|e| format!("Failed to fetch weather: {}", e))?
            .json()
            .map_err(|e| format!("Failed to parse weather data: {}", e))?;

        let weather_data = WeatherData {
            temperature: Some(weather.current.temperature_2m),
        };

        Ok(WeatherInfo::new(location.to_string(), weather_data))
    }
}
