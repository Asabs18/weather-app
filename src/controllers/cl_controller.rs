use crate::models::weather_info::WeatherInfo;
use crate::views::cl_view::ClView;

pub struct ClController {
    pub cl_view: ClView,
}

impl ClController {
    pub fn new(weather_info: WeatherInfo) -> Self {
        let cl_view = ClView::from_weather_info(weather_info.clone());
        ClController { cl_view }
    }

    pub fn show_weather(&self) {
        self.cl_view.display();
    }
}
