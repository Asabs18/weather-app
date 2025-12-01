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
│   │   └── weather_repository.rs # WeatherRepository trait and mock implementation
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
- **Repositories** (`repositories/`): Handle data fetching (currently using mock data)

## Features

- Interactive command-line interface
- Mock weather data for sample locations (London, Boston)
- Extensible architecture for adding real API integration

## Dependencies

- `serde` - Serialization framework (prepared for API integration)

## Building

```bash
cargo build
```

## Running

```bash
cargo run
```