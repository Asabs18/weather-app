//! Weather data models
//!
//! All measurements stored in metric units (scientific standard):
//! - Temperature: Celsius
//! - Distance: millimeters/meters
//! - Speed: km/h
//! - Pressure: hPa

use serde::Deserialize;

/// Current weather conditions
#[derive(Clone, Debug, Deserialize)]
pub struct CurrentWeather {
    // Temperature in Celsius
    pub temperature: Option<f64>,
    // Feels-like temperature in Celsius
    pub apparent_temperature: Option<f64>,
    // Relative humidity percentage (0-100)
    pub humidity: Option<f64>,
    // Precipitation amount in mm
    pub precipitation: Option<f64>,
    // Weather condition code (WMO code)
    pub weather_code: Option<i32>,
    // Wind speed in km/h
    pub wind_speed: Option<f64>,
    // Wind direction in degrees (0-360)
    pub wind_direction: Option<f64>,
    // Cloud cover percentage (0-100)
    pub cloud_cover: Option<f64>,
    // Atmospheric pressure in hPa
    pub pressure: Option<f64>,
    // Visibility in meters
    pub visibility: Option<f64>,
}

/// Hourly forecast data point
/// Note: Some fields fetched from API but not yet displayed in view
#[derive(Clone, Debug, Deserialize)]
#[allow(dead_code)]
pub struct HourlyForecast {
    // Time of forecast (ISO 8601 format)
    pub time: String,
    // Temperature in Celsius
    pub temperature: Option<f64>,
    // Apparent temperature in Celsius
    pub apparent_temperature: Option<f64>,
    // Precipitation probability (0-100)
    pub precipitation_probability: Option<f64>,
    // Precipitation amount in mm
    pub precipitation: Option<f64>,
    // Weather condition code
    pub weather_code: Option<i32>,
    // Wind speed in km/h
    pub wind_speed: Option<f64>,
    // Humidity percentage
    pub humidity: Option<f64>,
}

/// Daily forecast data point
#[derive(Clone, Debug, Deserialize)]
pub struct DailyForecast {
    // Date of forecast (ISO 8601 format)
    pub date: String,
    // Maximum temperature in Celsius
    pub temperature_max: Option<f64>,
    // Minimum temperature in Celsius
    pub temperature_min: Option<f64>,
    // Weather condition code
    pub weather_code: Option<i32>,
    // Total precipitation in mm
    pub precipitation_sum: Option<f64>,
    // Precipitation probability (0-100)
    pub precipitation_probability: Option<f64>,
    // Maximum wind speed in km/h
    pub wind_speed_max: Option<f64>,
    // Sunrise time (ISO 8601 format)
    pub sunrise: Option<String>,
    // Sunset time (ISO 8601 format)
    pub sunset: Option<String>,
}

/// Complete weather data for a location
#[derive(Clone, Debug, Deserialize)]
pub struct WeatherData {
    // Current weather conditions
    pub current: CurrentWeather,
    // Hourly forecasts (typically 24-48 hours)
    pub hourly: Vec<HourlyForecast>,
    // Daily forecasts (typically 7-14 days)
    pub daily: Vec<DailyForecast>,
}

/// Complete weather information including location and data
#[derive(Clone, Debug)]
pub struct WeatherInfo {
    // Location name
    pub location: String,
    // Coordinates
    pub latitude: f64,
    pub longitude: f64,
    // Weather data for this location
    pub weather_data: WeatherData,
}

impl WeatherInfo {
    // Creates a new WeatherInfo instance
    pub fn new(location: String, latitude: f64, longitude: f64, weather_data: WeatherData) -> Self {
        WeatherInfo {
            location,
            latitude,
            longitude,
            weather_data,
        }
    }
}
