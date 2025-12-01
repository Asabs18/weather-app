pub struct WeatherData {
    pub temperature: Option<f32>,
}

pub struct WeatherInfo {
    pub location: String,
    pub weather_data: WeatherData,
}

impl WeatherInfo {
    pub fn new(location: String) -> Self {
        let weather_data = Self::create_weather_data(location.clone());

        WeatherInfo {
            location,
            weather_data,
        }
    }

    pub fn create_weather_data(location: String) -> WeatherData {
        if location.to_lowercase() == "london" {
            return WeatherData {
                temperature: Some(15.0),
            };
        } else if location.to_lowercase() == "boston" {
            return WeatherData {
                temperature: Some(22.0),
            };
        } else {
            return WeatherData { temperature: None };
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
