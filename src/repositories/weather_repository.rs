//! Weather data repository for fetching from external APIs

use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::Value;

use crate::constants::{
    DAILY_FORECAST_DAYS, HOURLY_FORECAST_LIMIT, NOMINATIM_API_URL, OPEN_METEO_API_URL, USER_AGENT,
};
use crate::errors::WeatherError;
use crate::models::weather_info::{
    CurrentWeather, DailyForecast, HourlyForecast, WeatherData, WeatherInfo,
};

/// API response structure from Open-Meteo
#[derive(Debug, Deserialize)]
struct OpenMeteoWeather {
    current: OpenMeteoCurrent,
    hourly: Option<OpenMeteoHourly>,
    daily: Option<OpenMeteoDaily>,
}

/// Current weather from Open-Meteo API
/// Field names match API response format (snake_case with units)
#[derive(Debug, Deserialize)]
struct OpenMeteoCurrent {
    /// Temperature at 2 meters above ground in Celsius
    temperature_2m: Option<f64>,
    /// Apparent temperature (feels like) in Celsius
    apparent_temperature: Option<f64>,
    /// Relative humidity percentage
    relative_humidity_2m: Option<f64>,
    /// Precipitation amount in mm
    precipitation: Option<f64>,
    /// Weather condition code (WMO)
    weather_code: Option<i32>,
    /// Wind speed in km/h
    wind_speed_10m: Option<f64>,
    /// Wind direction in degrees
    wind_direction_10m: Option<f64>,
    /// Cloud cover percentage
    cloud_cover: Option<f64>,
    /// Surface pressure in hPa
    surface_pressure: Option<f64>,
    /// Visibility in meters
    visibility: Option<f64>,
}

/// Hourly forecast arrays from Open-Meteo API
#[derive(Debug, Deserialize)]
struct OpenMeteoHourly {
    time: Vec<String>,
    temperature_2m: Vec<Option<f64>>,
    apparent_temperature: Vec<Option<f64>>,
    precipitation_probability: Vec<Option<f64>>,
    precipitation: Vec<Option<f64>>,
    weather_code: Vec<Option<i32>>,
    wind_speed_10m: Vec<Option<f64>>,
    relative_humidity_2m: Vec<Option<f64>>,
}

/// Daily forecast arrays from Open-Meteo API
#[derive(Debug, Deserialize)]
struct OpenMeteoDaily {
    time: Vec<String>,
    temperature_2m_max: Vec<Option<f64>>,
    temperature_2m_min: Vec<Option<f64>>,
    weather_code: Vec<Option<i32>>,
    precipitation_sum: Vec<Option<f64>>,
    precipitation_probability_max: Vec<Option<f64>>,
    wind_speed_10m_max: Vec<Option<f64>>,
    sunrise: Vec<Option<String>>,
    sunset: Vec<Option<String>>,
}

/// Trait for weather data sources (enables dependency injection)
pub trait WeatherRepository {
    fn fetch_weather(&self, location: &str) -> Result<WeatherInfo, WeatherError>;
}

/// Implementation using OpenStreetMap Nominatim (geocoding) and Open-Meteo (weather)
pub struct ApiWeatherRepository {
    client: Client,
}

impl Default for ApiWeatherRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiWeatherRepository {
    pub fn new() -> Self {
        ApiWeatherRepository {
            client: Client::new(),
        }
    }

    /// Converts location name to coordinates using Nominatim geocoding API
    fn fetch_coordinates(&self, location: &str) -> Result<(f64, f64), WeatherError> {
        let location = location.trim();
        let url = format!(
            "{}?q={}&format=json&limit=1",
            NOMINATIM_API_URL,
            urlencoding::encode(location)
        );

        let response = self
            .client
            .get(&url)
            .header("User-Agent", USER_AGENT)
            .send()
            .map_err(|e| WeatherError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            return Err(WeatherError::ApiError(format!(
                "Geocoding API returned status: {}",
                response.status()
            )));
        }

        let json: Value = response
            .json()
            .map_err(|e| WeatherError::ParseError(e.to_string()))?;

        let array = json.as_array().ok_or_else(|| {
            WeatherError::ParseError("Invalid response format from geocoding API".to_string())
        })?;

        if array.is_empty() {
            return Err(WeatherError::LocationNotFound(location.to_string()));
        }

        let first_result = &array[0];
        let lat = first_result["lat"]
            .as_str()
            .and_then(|s| s.parse::<f64>().ok())
            .ok_or_else(|| WeatherError::ParseError("Failed to parse latitude".to_string()))?;
        let lon = first_result["lon"]
            .as_str()
            .and_then(|s| s.parse::<f64>().ok())
            .ok_or_else(|| WeatherError::ParseError("Failed to parse longitude".to_string()))?;

        Ok((lat, lon))
    }

    /// Maps API response to domain model
    fn parse_current_weather(current: &OpenMeteoCurrent) -> CurrentWeather {
        CurrentWeather {
            temperature: current.temperature_2m,
            apparent_temperature: current.apparent_temperature,
            humidity: current.relative_humidity_2m,
            precipitation: current.precipitation,
            weather_code: current.weather_code,
            wind_speed: current.wind_speed_10m,
            wind_direction: current.wind_direction_10m,
            cloud_cover: current.cloud_cover,
            pressure: current.surface_pressure,
            visibility: current.visibility,
        }
    }

    fn parse_hourly_forecasts(hourly: Option<OpenMeteoHourly>) -> Vec<HourlyForecast> {
        if let Some(hourly) = hourly {
            let count = hourly.time.len().min(HOURLY_FORECAST_LIMIT);
            (0..count)
                .map(|i| HourlyForecast {
                    time: hourly.time[i].clone(),
                    temperature: hourly.temperature_2m.get(i).and_then(|v| *v),
                    apparent_temperature: hourly.apparent_temperature.get(i).and_then(|v| *v),
                    precipitation_probability: hourly
                        .precipitation_probability
                        .get(i)
                        .and_then(|v| *v),
                    precipitation: hourly.precipitation.get(i).and_then(|v| *v),
                    weather_code: hourly.weather_code.get(i).and_then(|v| *v),
                    wind_speed: hourly.wind_speed_10m.get(i).and_then(|v| *v),
                    humidity: hourly.relative_humidity_2m.get(i).and_then(|v| *v),
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Parses daily forecast data from API response
    fn parse_daily_forecasts(daily: Option<OpenMeteoDaily>) -> Vec<DailyForecast> {
        if let Some(daily) = daily {
            (0..daily.time.len())
                .map(|i| DailyForecast {
                    date: daily.time[i].clone(),
                    temperature_max: daily.temperature_2m_max.get(i).and_then(|v| *v),
                    temperature_min: daily.temperature_2m_min.get(i).and_then(|v| *v),
                    weather_code: daily.weather_code.get(i).and_then(|v| *v),
                    precipitation_sum: daily.precipitation_sum.get(i).and_then(|v| *v),
                    precipitation_probability: daily
                        .precipitation_probability_max
                        .get(i)
                        .and_then(|v| *v),
                    wind_speed_max: daily.wind_speed_10m_max.get(i).and_then(|v| *v),
                    sunrise: daily.sunrise.get(i).and_then(|v| v.clone()),
                    sunset: daily.sunset.get(i).and_then(|v| v.clone()),
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    // Constructs the Open-Meteo API URL with query parameters
    fn build_weather_api_url(lat: f64, lon: f64) -> String {
        format!(
            "{OPEN_METEO_API_URL}?latitude={lat}&longitude={lon}&current=temperature_2m,apparent_temperature,relative_humidity_2m,precipitation,weather_code,wind_speed_10m,wind_direction_10m,cloud_cover,surface_pressure,visibility&hourly=temperature_2m,apparent_temperature,precipitation_probability,precipitation,weather_code,wind_speed_10m,relative_humidity_2m&daily=temperature_2m_max,temperature_2m_min,weather_code,precipitation_sum,precipitation_probability_max,wind_speed_10m_max,sunrise,sunset&forecast_days={DAILY_FORECAST_DAYS}"
        )
    }
}

// Implement the WeatherRepository trait for ApiWeatherRepository
impl WeatherRepository for ApiWeatherRepository {
    // Fetches weather information for a given location
    fn fetch_weather(&self, location: &str) -> Result<WeatherInfo, WeatherError> {
        let (lat, lon) = self.fetch_coordinates(location)?;

        let url = Self::build_weather_api_url(lat, lon);
        let weather: OpenMeteoWeather = self
            .client
            .get(&url)
            .send()
            .map_err(|e| WeatherError::NetworkError(e.to_string()))?
            .json()
            .map_err(|e| WeatherError::ParseError(e.to_string()))?;

        let current_weather = Self::parse_current_weather(&weather.current);
        let hourly_forecasts = Self::parse_hourly_forecasts(weather.hourly);
        let daily_forecasts = Self::parse_daily_forecasts(weather.daily);

        let weather_data = WeatherData {
            current: current_weather,
            hourly: hourly_forecasts,
            daily: daily_forecasts,
        };

        Ok(WeatherInfo::new(
            location.to_string(),
            lat,
            lon,
            weather_data,
        ))
    }
}
