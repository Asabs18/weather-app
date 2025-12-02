// src/views/cl_view.rs

use crate::models::weather_info::WeatherInfo;

// Command-line view for displaying weather information
pub struct ClView;

impl ClView {
    // Converts Celsius to Fahrenheit
    fn celsius_to_fahrenheit(celsius: f64) -> f64 {
        (celsius * 9.0 / 5.0) + 32.0
    }

    // Displays weather information to the console
    pub fn display(weather_info: &WeatherInfo) {
        println!("Location: {}", weather_info.location);
        
        match weather_info.weather_data.temperature {
            Some(celsius) => {
                let fahrenheit = Self::celsius_to_fahrenheit(celsius);
                println!("Temperature: {:.1}°C / {:.1}°F", celsius, fahrenheit);
            }
            None => {
                println!("No temperature data available");
            }
        }
    }
}
