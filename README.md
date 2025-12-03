# Weather App 🌤️

A modern weather application built in Rust featuring both a command-line interface and a beautiful desktop GUI. Get real-time weather data, hourly forecasts, and 7-day predictions for any location worldwide.

![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

## ✨ Features

- 🖥️ **Dual Interface**: Choose between CLI or modern desktop GUI
- 🌍 **Global Coverage**: Get weather data for any location worldwide
- 📊 **Comprehensive Data**: Current conditions, 24-hour hourly forecast, and 7-day daily forecast
- 🌡️ **Dual Units**: Displays both Fahrenheit and Celsius temperatures
- 🎨 **Beautiful GUI**: Dark-themed desktop interface with weather icons
- ⚡ **Real-time Data**: Powered by Open-Meteo API
- 🏗️ **Clean Architecture**: MVC pattern with repository layer for maintainability

## 📦 Installation

### Prerequisites
- Rust 1.70 or higher
- Cargo (comes with Rust)

### Clone and Build

```bash
git clone https://github.com/Asabs18/weather-app.git
cd weather-app
cargo build --release
```

## 🚀 Usage

### GUI Application (Recommended)

```bash
cargo run --release --bin gui_main
```

### CLI Application

```bash
cargo run --release --bin weather-app
```

Or after building, run the executables directly:
- Windows: `target\release\gui_main.exe` or `target\release\weather-app.exe`
- Linux/Mac: `./target/release/gui_main` or `./target/release/weather-app`

## 🏗️ Project Structure

```
weather-app/
├── src/
│   ├── main.rs                      # CLI application entry point
│   ├── lib.rs                       # Library exposing all modules
│   ├── bin/
│   │   └── gui_main.rs              # GUI application entry point
│   ├── constants.rs                 # API URLs and configuration
│   ├── errors.rs                    # Custom error types
│   ├── controllers/
│   │   └── cl_controller.rs         # CLI controller logic
│   ├── models/
│   │   └── weather_info.rs          # Weather data models
│   ├── repositories/
│   │   └── weather_repository.rs    # API data fetching
│   ├── utils/
│   │   └── conversions.rs           # Temperature and unit conversions
│   └── views/
│       ├── cl_view.rs               # CLI display logic
│       └── gui_view.rs              # GUI interface implementation
└── Cargo.toml
```

## 🏛️ Architecture

This project follows the **Model-View-Controller (MVC)** pattern:

- **Models** 📋: Weather data structures (current, hourly, daily forecasts)
- **Views** 🎨: CLI and GUI presentation layers
- **Controllers** 🎮: Coordinate data flow between repositories and views
- **Repositories** 🗄️: Handle external API communication
- **Utils** 🔧: Shared utilities (temperature conversions, etc.)

## 🌐 APIs Used

- **[OpenStreetMap Nominatim](https://nominatim.org/)**: Geocoding service to convert location names to coordinates
- **[Open-Meteo](https://open-meteo.com/)**: Free weather API providing real-time weather data and forecasts

## 📚 Dependencies

```toml
reqwest = { version = "0.12.24", features = ["blocking", "json"] }
serde = { version = "1.0.228", features = ["derive"] }
serde_json = "1.0.145"
urlencoding = "2.1"
eframe = "0.29"      # GUI framework
egui = "0.29"        # Immediate mode GUI library
```

## 🎯 Key Features Breakdown

### Current Weather
- Temperature (°F/°C)
- Feels like temperature
- Humidity percentage
- Wind speed
- Atmospheric pressure
- Precipitation
- Weather conditions with icons

### Hourly Forecast (24 hours)
- Hour-by-hour breakdown
- Temperature trends
- Weather condition changes
- Precipitation probability
- Wind speed variations

### Daily Forecast (7 days)
- High and low temperatures
- Weather conditions
- Precipitation probability
- Max wind speeds
- Sunrise and sunset times

## 🛠️ Development

### Run Tests
```bash
cargo test
```

### Format Code
```bash
cargo fmt
```

### Lint Code
```bash
cargo clippy
```

### Build for Release
```bash
cargo build --release
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

- Weather data provided by [Open-Meteo](https://open-meteo.com/)
- Geocoding by [OpenStreetMap Nominatim](https://nominatim.org/)
- GUI powered by [egui](https://github.com/emilk/egui)
- GUI powered by [egui](https://github.com/emilk/egui)

---

Made with ❤️ and Rust 🦀