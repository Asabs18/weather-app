//! Application-wide constants for API URLs and configuration

/// OpenStreetMap Nominatim API for geocoding location names to coordinates
pub const NOMINATIM_API_URL: &str = "https://nominatim.openstreetmap.org/search";

/// Open-Meteo API for fetching weather data
pub const OPEN_METEO_API_URL: &str = "https://api.open-meteo.com/v1/forecast";

/// User agent for API requests (required by Nominatim)
pub const USER_AGENT: &str = "RustWeatherApp/1.0";

/// Number of hourly forecast data points to display
pub const HOURLY_FORECAST_LIMIT: usize = 24;

/// Number of days for daily forecast
pub const DAILY_FORECAST_DAYS: usize = 7;
