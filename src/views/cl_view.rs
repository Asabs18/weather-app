use crate::models::weather_info::WeatherInfo;

pub struct ClView {
    pub weather_info: WeatherInfo,
}

impl ClView {
    pub fn from_weather_info(weather_info: WeatherInfo) -> Self {
        ClView {
            weather_info: weather_info,
        }
    }

    pub fn display(&self) {
        println!("Location: {}", self.weather_info.location);
        println!(
            "Temperature: {}°C",
            self.weather_info.weather_data.temperature
        );
    }
}
