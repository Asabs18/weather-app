# Weather App

A command-line weather application built in Rust using MVC architecture.

## Project Structure

```
weather-app/
├── src/
│   ├── main.rs              # Application entry point
│   ├── constants.rs         # API URLs and configuration constants
│   ├── errors.rs            # Custom error types (WeatherError)
│   ├── controllers/         # Controller layer - coordinates between models and views
│   │   ├── mod.rs
│   │   └── cl_controller.rs # Command-line controller
│   ├── models/              # Model layer - data structures
│   │   ├── mod.rs
│   │   └── weather_info.rs  # Weather data models (current, hourly, daily)
│   ├── repositories/        # Repository layer - data fetching
│   │   ├── mod.rs
│   │   └── weather_repository.rs # WeatherRepository trait and API implementation
│   ├── utils/               # Utility functions
│   │   ├── mod.rs
│   │   └── conversions.rs   # Unit conversion utilities (metric ↔ imperial)
│   └── views/               # View layer - presentation
│       ├── mod.rs
│       └── cl_view.rs       # Command-line view
└── Cargo.toml
```

## Architecture

This project follows the **Model-View-Controller (MVC)** pattern with a repository layer:

- **Models** (`models/`): Define data structures for current weather, hourly forecasts, and daily forecasts
- **Views** (`views/`): Handle presentation logic with dual unit display (metric and imperial)
- **Controllers** (`controllers/`): Coordinate between repositories and views with proper error handling
- **Repositories** (`repositories/`): Handle data fetching via external APIs with custom error types
- **Utils** (`utils/`): Provide conversion utilities for temperature, distance, speed, and pressure
- **Constants** (`constants.rs`): Centralize API URLs and configuration
- **Errors** (`errors.rs`): Define custom error types for better error handling

## Features

- Interactive command-line interface
- **Real-time weather data** from Open-Meteo API
- **Geocoding support** via OpenStreetMap Nominatim API
- **Current weather conditions** with temperature, humidity, wind speed, and pressure
- **24-hour hourly forecast** with detailed conditions
- **7-day daily forecast** with high/low temperatures and precipitation
- **Dual unit display** - shows both metric and imperial measurements
- **Custom error handling** with descriptive error messages
- Support for **any global location** (not limited to predefined cities)
- Extensible architecture with repository pattern for easy testing and data source swapping

### APIs Used

- **OpenStreetMap Nominatim**: Converts location names to geographic coordinates
- **Open-Meteo**: Provides real-time weather data (temperature, etc.)

## Dependencies

- `reqwest` - HTTP client for making API requests (with blocking and JSON features)
- `serde` - Serialization framework for parsing JSON responses
- `serde_json` - JSON parsing utilities
- `urlencoding` - URL encoding for location names in API requests

## Building

```bash
cargo build
```

## Running

```bash
cargo run
```