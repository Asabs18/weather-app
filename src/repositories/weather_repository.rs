// src/repositories/weather_repository.rs

use crate::models::weather_info::{WeatherData, WeatherInfo};

// Trait for fetching weather data from various sources
pub trait WeatherRepository {
    // Fetches weather information for the given location
    fn fetch_weather(&self, location: &str) -> Result<WeatherInfo, String>;
}

// Mock implementation of WeatherRepository with hardcoded data
pub struct MockWeatherRepository;

impl WeatherRepository for MockWeatherRepository {
    fn fetch_weather(&self, location: &str) -> Result<WeatherInfo, String> {
        // Match location to hardcoded weather data
        let weather_data = match location.to_lowercase().as_str() {
            "london" => WeatherData {
                temperature: Some(15.0),
            },
            "boston" => WeatherData {
                temperature: Some(22.0),
            },
            _ => return Err(format!("No data available for {}", location)),
        };

        Ok(WeatherInfo::new(location.to_string(), weather_data))
    }
}
