// src/views/cl_view.rs

use crate::models::weather_info::WeatherInfo;

// Command-line view for displaying weather information
pub struct ClView;

impl ClView {
    // Displays weather information to the console
    pub fn display(weather_info: &WeatherInfo) {
        match weather_info.weather_data.temperature {
            Some(temp) => {
                println!("Location: {}", weather_info.location);
                println!("Temperature: {}°C", temp);
            }
            None => {
                println!("No weather data available for {}", weather_info.location);
            }
        }
    }
}
