//! Custom error types for the weather application

use std::fmt;

/// Application-specific errors with descriptive context
#[derive(Debug)]
pub enum WeatherError {
    /// Network connectivity or request failures
    NetworkError(String),
    /// JSON parsing or data format issues
    ParseError(String),
    /// Location not found in geocoding service
    LocationNotFound(String),
    /// API returned error status or invalid response
    ApiError(String),
}

impl fmt::Display for WeatherError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WeatherError::NetworkError(msg) => write!(f, "Network error: {msg}"),
            WeatherError::ParseError(msg) => write!(f, "Parse error: {msg}"),
            WeatherError::LocationNotFound(loc) => write!(f, "Location '{loc}' not found"),
            WeatherError::ApiError(msg) => write!(f, "API error: {msg}"),
        }
    }
}

impl std::error::Error for WeatherError {}
