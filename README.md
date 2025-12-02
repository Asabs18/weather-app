# Weather App

A command-line weather application built in Rust using MVC architecture.

## Project Structure

```
weather-app/
├── src/
│   ├── main.rs              # Application entry point
│   ├── controllers/         # Controller layer - coordinates between models and views
│   │   ├── mod.rs
│   │   └── cl_controller.rs # Command-line controller
│   ├── models/              # Model layer - data structures
│   │   ├── mod.rs
│   │   └── weather_info.rs  # WeatherInfo and WeatherData structs
│   ├── repositories/        # Repository layer - data fetching
│   │   ├── mod.rs
│   │   └── weather_repository.rs # WeatherRepository trait and API implementation
│   └── views/               # View layer - presentation
│       ├── mod.rs
│       └── cl_view.rs       # Command-line view
└── Cargo.toml
```

## Architecture

This project follows the **Model-View-Controller (MVC)** pattern with a repository layer:

- **Models** (`models/`): Define data structures (`WeatherInfo`, `WeatherData`)
- **Views** (`views/`): Handle presentation logic (console output)
- **Controllers** (`controllers/`): Coordinate between repositories and views
- **Repositories** (`repositories/`): Handle data fetching via external APIs

## Features

- Interactive command-line interface
- **Real-time weather data** from Open-Meteo API
- **Geocoding support** via OpenStreetMap Nominatim API
- Temperature display in Celsius
- Support for **any global location** (not limited to predefined cities)
- Extensible architecture for adding more weather data fields

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