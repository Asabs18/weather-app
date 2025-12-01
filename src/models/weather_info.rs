pub struct WeatherData {
    pub temperature: f32,
}

pub struct WeatherInfo {
    pub location: String,
    pub weather_data: WeatherData,
}

impl WeatherInfo {
    pub fn new(location: String, weather_data: WeatherData) -> Self {
        WeatherInfo {
            location,
            weather_data,
        }
    }

    pub fn clone(&self) -> Self {
        WeatherInfo {
            location: self.location.clone(),
            weather_data: WeatherData {
                temperature: self.weather_data.temperature,
            },
        }
    }
}
