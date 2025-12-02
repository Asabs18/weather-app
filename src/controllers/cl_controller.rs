//! Command-line controller for coordinating weather data flow

use crate::errors::WeatherError;
use crate::repositories::weather_repository::WeatherRepository;
use crate::views::cl_view::ClView;

/// Controller that coordinates between repository (data) and view (display)
/// Generic over WeatherRepository to allow different data sources
pub struct ClController<WeatherRepo: WeatherRepository> {
    repository: WeatherRepo,
}

impl<WeatherRepo: WeatherRepository> ClController<WeatherRepo> {
    pub fn new(repository: WeatherRepo) -> Self {
        ClController { repository }
    }

    /// Fetches weather data for location and displays it
    pub fn show_weather(&self, location: &str) -> Result<(), WeatherError> {
        let weather_info = self.repository.fetch_weather(location)?;
        ClView::display(&weather_info);
        Ok(())
    }
}
