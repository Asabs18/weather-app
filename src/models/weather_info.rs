// src/models/weather_info.rs

// Represents weather data for a location
#[derive(Clone, Debug)]
pub struct WeatherData {
    // Temperature in Celsius
    pub temperature: Option<f32>,
}

// Complete weather information including location and data
#[derive(Clone, Debug)]
pub struct WeatherInfo {
    // Location name
    pub location: String,
    // Weather data for this location
    pub weather_data: WeatherData,
}

impl WeatherInfo {
    // Creates a new WeatherInfo instance
    pub fn new(location: String, weather_data: WeatherData) -> Self {
        WeatherInfo {
            location,
            weather_data,
        }
    }
}
