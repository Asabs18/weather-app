// src/controllers/cl_controller.rs

use crate::repositories::weather_repository::WeatherRepository;
use crate::views::cl_view::ClView;

// Controller for handling command-line weather requests
// Coordinates between the repository (data) and view (display)
pub struct ClController<WeatherRepo: WeatherRepository> {
    repository: WeatherRepo,
}

impl<WeatherRepo: WeatherRepository> ClController<WeatherRepo> {
    // Creates a new controller with the given repository
    pub fn new(repository: WeatherRepo) -> Self {
        ClController { repository }
    }

    // Fetches and displays weather information for the given location
    pub fn show_weather(&self, location: &str) {
        match self.repository.fetch_weather(location) {
            Ok(weather_info) => ClView::display(&weather_info),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
