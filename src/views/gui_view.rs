//! GUI view using egui for desktop application

use crate::models::weather_info::{DailyForecast, WeatherInfo};
use crate::repositories::weather_repository::{ApiWeatherRepository, WeatherRepository};
use crate::utils::conversions::Temperature;
use eframe::egui;

pub struct WeatherApp {
    location_input: String,
    weather_info: Option<WeatherInfo>,
    error_message: Option<String>,
    repository: ApiWeatherRepository,
    selected_tab: Tab,
}

#[derive(PartialEq)]
enum Tab {
    Current,
    Hourly,
    Daily,
}

// Color scheme
struct Colors;
impl Colors {
    // Background colors
    const BG_PRIMARY: egui::Color32 = egui::Color32::from_rgb(15, 23, 42); // Dark blue-gray
    const BG_SECONDARY: egui::Color32 = egui::Color32::from_rgb(30, 41, 59); // Lighter blue-gray
    const BG_CARD: egui::Color32 = egui::Color32::from_rgb(51, 65, 85); // Card background
    #[allow(dead_code)]
    const BG_CARD_ALT: egui::Color32 = egui::Color32::from_rgb(45, 55, 72); // Alternate card

    // Accent colors
    const ACCENT_BLUE: egui::Color32 = egui::Color32::from_rgb(59, 130, 246); // Bright blue
    const ACCENT_CYAN: egui::Color32 = egui::Color32::from_rgb(34, 211, 238); // Cyan
    #[allow(dead_code)]
    const ACCENT_GREEN: egui::Color32 = egui::Color32::from_rgb(52, 211, 153); // Green
    const ACCENT_YELLOW: egui::Color32 = egui::Color32::from_rgb(251, 191, 36); // Yellow/sun
    const ACCENT_ORANGE: egui::Color32 = egui::Color32::from_rgb(251, 146, 60); // Orange

    // Text colors
    const TEXT_PRIMARY: egui::Color32 = egui::Color32::from_rgb(248, 250, 252); // Near white
    const TEXT_SECONDARY: egui::Color32 = egui::Color32::from_rgb(203, 213, 225); // Light gray
    const TEXT_MUTED: egui::Color32 = egui::Color32::from_rgb(148, 163, 184); // Muted gray

    // Status colors
    const ERROR_RED: egui::Color32 = egui::Color32::from_rgb(239, 68, 68); // Error red
    #[allow(dead_code)]
    const SUCCESS_GREEN: egui::Color32 = egui::Color32::from_rgb(34, 197, 94); // Success green
}

impl Default for WeatherApp {
    fn default() -> Self {
        Self {
            location_input: String::new(),
            weather_info: None,
            error_message: None,
            repository: ApiWeatherRepository::new(),
            selected_tab: Tab::Current,
        }
    }
}

impl eframe::App for WeatherApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply dark theme
        ctx.set_visuals(create_custom_visuals());

        egui::CentralPanel::default()
            .frame(
                egui::Frame::none()
                    .fill(Colors::BG_PRIMARY)
                    .inner_margin(egui::Margin::same(20.0)),
            )
            .show(ctx, |ui| {
                ui.add_space(20.0);

                // Header with search
                ui.vertical_centered(|ui| {
                    ui.label(
                        egui::RichText::new("Weather Dashboard")
                            .size(36.0)
                            .strong()
                            .color(Colors::TEXT_PRIMARY),
                    );
                    ui.add_space(8.0);
                });

                // Search bar
                egui::Frame::none()
                    .fill(Colors::BG_SECONDARY)
                    .rounding(12.0)
                    .inner_margin(egui::Margin::symmetric(20.0, 15.0))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new("Location:")
                                    .color(Colors::TEXT_SECONDARY)
                                    .size(15.0),
                            );

                            let text_edit = egui::TextEdit::singleline(&mut self.location_input)
                                .hint_text("Enter city name...")
                                .desired_width(ui.available_width() - 120.0)
                                .font(egui::TextStyle::Body);

                            let response = ui.add(text_edit);

                            if response.lost_focus()
                                && ui.input(|i| i.key_pressed(egui::Key::Enter))
                            {
                                self.fetch_weather();
                            }

                            let button = egui::Button::new(
                                egui::RichText::new("Search")
                                    .size(15.0)
                                    .color(Colors::TEXT_PRIMARY),
                            )
                            .fill(Colors::ACCENT_BLUE)
                            .rounding(8.0)
                            .min_size(egui::vec2(90.0, 32.0));

                            if ui.add(button).clicked() {
                                self.fetch_weather();
                            }
                        });
                    });

                ui.add_space(15.0);

                // Error display
                if let Some(error) = &self.error_message {
                    egui::Frame::none()
                        .fill(egui::Color32::from_rgb(127, 29, 29))
                        .rounding(8.0)
                        .inner_margin(12.0)
                        .show(ui, |ui| {
                            ui.label(
                                egui::RichText::new(format!("Error: {error}"))
                                    .color(Colors::ERROR_RED)
                                    .size(14.0),
                            );
                        });
                    ui.add_space(10.0);
                }

                // Weather display
                if let Some(weather) = &self.weather_info {
                    self.display_location_header(ui, weather);
                    ui.add_space(15.0);

                    // Tab navigation
                    ui.horizontal(|ui| {
                        let tab_button = |ui: &mut egui::Ui, label: &str, is_selected: bool| {
                            let button = egui::Button::new(
                                egui::RichText::new(label).size(15.0).color(if is_selected {
                                    Colors::TEXT_PRIMARY
                                } else {
                                    Colors::TEXT_MUTED
                                }),
                            )
                            .fill(if is_selected {
                                Colors::ACCENT_BLUE
                            } else {
                                Colors::BG_CARD
                            })
                            .rounding(8.0)
                            .min_size(egui::vec2(100.0, 36.0));
                            ui.add(button)
                        };

                        if tab_button(ui, "Current", self.selected_tab == Tab::Current).clicked() {
                            self.selected_tab = Tab::Current;
                        }
                        ui.add_space(5.0);
                        if tab_button(ui, "Hourly", self.selected_tab == Tab::Hourly).clicked() {
                            self.selected_tab = Tab::Hourly;
                        }
                        ui.add_space(5.0);
                        if tab_button(ui, "Daily", self.selected_tab == Tab::Daily).clicked() {
                            self.selected_tab = Tab::Daily;
                        }
                    });

                    ui.add_space(15.0);

                    // Tab content
                    egui::ScrollArea::vertical().show(ui, |ui| match self.selected_tab {
                        Tab::Current => self.display_current_weather(ui, weather),
                        Tab::Hourly => self.display_hourly_forecast(ui, weather),
                        Tab::Daily => self.display_daily_forecast(ui, weather),
                    });
                } else {
                    ui.vertical_centered(|ui| {
                        ui.add_space(120.0);
                        ui.label(
                            egui::RichText::new("Enter a location to get started")
                                .size(20.0)
                                .color(Colors::TEXT_MUTED),
                        );
                    });
                }

                ui.add_space(20.0);
            });
    }
}

impl WeatherApp {
    fn fetch_weather(&mut self) {
        self.error_message = None;

        match self.repository.fetch_weather(&self.location_input) {
            Ok(weather) => {
                self.weather_info = Some(weather);
            }
            Err(e) => {
                self.error_message = Some(format!("{e}"));
                self.weather_info = None;
            }
        }
    }

    fn display_location_header(&self, ui: &mut egui::Ui, weather: &WeatherInfo) {
        egui::Frame::none()
            .fill(Colors::BG_SECONDARY)
            .rounding(12.0)
            .inner_margin(15.0)
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.label(
                        egui::RichText::new(&weather.location)
                            .size(26.0)
                            .strong()
                            .color(Colors::TEXT_PRIMARY),
                    );
                    ui.label(
                        egui::RichText::new(format!(
                            "{:.4}°N, {:.4}°E",
                            weather.latitude, weather.longitude
                        ))
                        .size(13.0)
                        .color(Colors::TEXT_MUTED),
                    );
                });
            });
    }

    fn display_current_weather(&self, ui: &mut egui::Ui, weather: &WeatherInfo) {
        let current = &weather.weather_data.current;

        ui.horizontal_top(|ui| {
            // Main temperature card
            egui::Frame::none()
                .fill(Colors::BG_CARD)
                .rounding(12.0)
                .inner_margin(25.0)
                .show(ui, |ui| {
                    ui.set_min_width(240.0);
                    ui.vertical_centered(|ui| {
                        if let Some(code) = current.weather_code {
                            ui.label(
                                egui::RichText::new(weather_code_to_icon(code))
                                    .size(80.0)
                                    .color(weather_code_to_color(code)),
                            );
                        }

                        ui.add_space(10.0);

                        if let Some(temp) = current.temperature {
                            let temp_f = Temperature::celsius_to_fahrenheit(temp);
                            ui.label(
                                egui::RichText::new(format!("{temp_f:.1}°F"))
                                    .size(52.0)
                                    .strong()
                                    .color(Colors::TEXT_PRIMARY),
                            );
                            ui.label(
                                egui::RichText::new(format!("{temp:.1}°C"))
                                    .size(18.0)
                                    .color(Colors::TEXT_SECONDARY),
                            );
                        }

                        ui.add_space(8.0);

                        if let Some(code) = current.weather_code {
                            ui.label(
                                egui::RichText::new(weather_code_to_description(code))
                                    .size(16.0)
                                    .color(Colors::TEXT_SECONDARY),
                            );
                        }
                    });
                });

            ui.add_space(15.0);

            // Details grid
            egui::Frame::none()
                .fill(Colors::BG_CARD)
                .rounding(12.0)
                .inner_margin(20.0)
                .show(ui, |ui| {
                    ui.set_min_width(ui.available_width());

                    egui::Grid::new("current_weather_grid")
                        .spacing([25.0, 18.0])
                        .show(ui, |ui| {
                            if let Some(apparent_temp) = current.apparent_temperature {
                                let apparent_f = Temperature::celsius_to_fahrenheit(apparent_temp);
                                ui.label(
                                    egui::RichText::new("Feels Like")
                                        .color(Colors::TEXT_SECONDARY)
                                        .size(14.0),
                                );
                                ui.label(
                                    egui::RichText::new(format!(
                                        "{apparent_f:.1}°F / {apparent_temp:.1}°C"
                                    ))
                                    .color(Colors::TEXT_PRIMARY)
                                    .size(15.0),
                                );
                                ui.end_row();
                            }

                            if let Some(humidity) = current.humidity {
                                ui.label(
                                    egui::RichText::new("Humidity")
                                        .color(Colors::TEXT_SECONDARY)
                                        .size(14.0),
                                );
                                ui.label(
                                    egui::RichText::new(format!("{humidity:.0}%"))
                                        .color(Colors::ACCENT_CYAN)
                                        .size(15.0),
                                );
                                ui.end_row();
                            }

                            if let Some(wind) = current.wind_speed {
                                ui.label(
                                    egui::RichText::new("Wind Speed")
                                        .color(Colors::TEXT_SECONDARY)
                                        .size(14.0),
                                );
                                let mut wind_text = format!("{wind:.1} km/h");
                                if let Some(direction) = current.wind_direction {
                                    wind_text.push_str(&format!(" ({}°)", direction as i32));
                                }
                                ui.label(
                                    egui::RichText::new(wind_text)
                                        .color(Colors::TEXT_PRIMARY)
                                        .size(15.0),
                                );
                                ui.end_row();
                            }

                            if let Some(precip) = current.precipitation {
                                ui.label(
                                    egui::RichText::new("Precipitation")
                                        .color(Colors::TEXT_SECONDARY)
                                        .size(14.0),
                                );
                                ui.label(
                                    egui::RichText::new(format!("{precip:.1} mm"))
                                        .color(Colors::ACCENT_CYAN)
                                        .size(15.0),
                                );
                                ui.end_row();
                            }

                            if let Some(pressure) = current.pressure {
                                ui.label(
                                    egui::RichText::new("Pressure")
                                        .color(Colors::TEXT_SECONDARY)
                                        .size(14.0),
                                );
                                ui.label(
                                    egui::RichText::new(format!("{pressure:.0} hPa"))
                                        .color(Colors::TEXT_PRIMARY)
                                        .size(15.0),
                                );
                                ui.end_row();
                            }

                            if let Some(cloud) = current.cloud_cover {
                                ui.label(
                                    egui::RichText::new("Cloud Cover")
                                        .color(Colors::TEXT_SECONDARY)
                                        .size(14.0),
                                );
                                ui.label(
                                    egui::RichText::new(format!("{cloud:.0}%"))
                                        .color(Colors::TEXT_PRIMARY)
                                        .size(15.0),
                                );
                                ui.end_row();
                            }

                            if let Some(visibility) = current.visibility {
                                ui.label(
                                    egui::RichText::new("Visibility")
                                        .color(Colors::TEXT_SECONDARY)
                                        .size(14.0),
                                );
                                ui.label(
                                    egui::RichText::new(format!("{:.1} km", visibility / 1000.0))
                                        .color(Colors::TEXT_PRIMARY)
                                        .size(15.0),
                                );
                                ui.end_row();
                            }
                        });
                });
        });
    }

    fn display_hourly_forecast(&self, ui: &mut egui::Ui, weather: &WeatherInfo) {
        ui.label(
            egui::RichText::new("Next 24 Hours")
                .size(20.0)
                .strong()
                .color(Colors::TEXT_PRIMARY),
        );
        ui.add_space(12.0);

        egui::ScrollArea::horizontal().show(ui, |ui| {
            ui.horizontal(|ui| {
                for hour in weather.weather_data.hourly.iter().take(24) {
                    egui::Frame::none()
                        .fill(Colors::BG_CARD)
                        .rounding(10.0)
                        .inner_margin(15.0)
                        .show(ui, |ui| {
                            ui.set_min_width(95.0);
                            ui.set_max_width(95.0);
                            ui.vertical_centered(|ui| {
                                // Time
                                let time_parts: Vec<&str> = hour.time.split('T').collect();
                                if time_parts.len() > 1 {
                                    let time_part = time_parts[1].split(':').next().unwrap_or("??");
                                    ui.label(
                                        egui::RichText::new(format!("{time_part}:00"))
                                            .strong()
                                            .size(13.0)
                                            .color(Colors::TEXT_SECONDARY),
                                    );
                                }

                                ui.add_space(8.0);

                                // Weather icon
                                if let Some(code) = hour.weather_code {
                                    ui.label(
                                        egui::RichText::new(weather_code_to_icon(code))
                                            .size(36.0)
                                            .color(weather_code_to_color(code)),
                                    );
                                    ui.label(
                                        egui::RichText::new(weather_code_to_description(code))
                                            .size(10.0)
                                            .color(Colors::TEXT_MUTED),
                                    );
                                }

                                ui.add_space(8.0);

                                // Temperature
                                if let Some(temp) = hour.temperature {
                                    let temp_f = Temperature::celsius_to_fahrenheit(temp);
                                    ui.label(
                                        egui::RichText::new(format!("{temp_f:.1}°F"))
                                            .size(18.0)
                                            .strong()
                                            .color(Colors::TEXT_PRIMARY),
                                    );
                                }

                                ui.add_space(6.0);

                                // Precipitation probability
                                if let Some(precip_prob) = hour.precipitation_probability {
                                    if precip_prob > 0.0 {
                                        ui.label(
                                            egui::RichText::new(format!("{precip_prob:.0}%"))
                                                .size(12.0)
                                                .color(Colors::ACCENT_CYAN),
                                        );
                                    }
                                }

                                // Wind
                                if let Some(wind) = hour.wind_speed {
                                    ui.label(
                                        egui::RichText::new(format!("{wind:.0} km/h"))
                                            .size(11.0)
                                            .color(Colors::TEXT_MUTED),
                                    );
                                }
                            });
                        });

                    ui.add_space(8.0);
                }
            });
        });
    }

    fn display_daily_forecast(&self, ui: &mut egui::Ui, weather: &WeatherInfo) {
        ui.label(
            egui::RichText::new("7-Day Forecast")
                .size(20.0)
                .strong()
                .color(Colors::TEXT_PRIMARY),
        );
        ui.add_space(12.0);

        for day in weather.weather_data.daily.iter().take(7) {
            egui::Frame::none()
                .fill(Colors::BG_CARD)
                .rounding(10.0)
                .inner_margin(18.0)
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());

                    ui.horizontal(|ui| {
                        // Date
                        ui.vertical(|ui| {
                            ui.set_width(90.0);
                            let date = format_date(&day.date);
                            ui.label(
                                egui::RichText::new(date)
                                    .size(15.0)
                                    .strong()
                                    .color(Colors::TEXT_PRIMARY),
                            );
                        });

                        ui.add_space(10.0);

                        // Weather icon
                        if let Some(code) = day.weather_code {
                            ui.vertical(|ui| {
                                ui.label(
                                    egui::RichText::new(weather_code_to_icon(code))
                                        .size(32.0)
                                        .color(weather_code_to_color(code)),
                                );
                                ui.label(
                                    egui::RichText::new(weather_code_to_description(code))
                                        .size(10.0)
                                        .color(Colors::TEXT_MUTED),
                                );
                            });
                        }

                        ui.add_space(15.0);

                        // Temperature range
                        ui.vertical(|ui| {
                            ui.set_width(180.0);
                            if let (Some(max), Some(min)) =
                                (day.temperature_max, day.temperature_min)
                            {
                                let max_f = Temperature::celsius_to_fahrenheit(max);
                                let min_f = Temperature::celsius_to_fahrenheit(min);
                                ui.label(
                                    egui::RichText::new(format!("High: {max_f:.1}°F / {max:.1}°C"))
                                        .color(Colors::ACCENT_ORANGE)
                                        .size(14.0),
                                );
                                ui.label(
                                    egui::RichText::new(format!("Low:  {min_f:.1}°F / {min:.1}°C"))
                                        .color(Colors::ACCENT_CYAN)
                                        .size(14.0),
                                );
                            }
                        });

                        ui.add_space(15.0);

                        // Precipitation
                        if let Some(precip_prob) = day.precipitation_probability {
                            ui.vertical(|ui| {
                                ui.set_width(80.0);
                                ui.label(
                                    egui::RichText::new("Precip")
                                        .color(Colors::TEXT_SECONDARY)
                                        .size(13.0),
                                );
                                ui.label(
                                    egui::RichText::new(format!("{precip_prob:.0}%"))
                                        .color(Colors::ACCENT_CYAN)
                                        .size(14.0),
                                );
                            });
                        }

                        ui.add_space(10.0);

                        // Wind
                        if let Some(wind) = day.wind_speed_max {
                            ui.vertical(|ui| {
                                ui.set_width(80.0);
                                ui.label(
                                    egui::RichText::new("Wind")
                                        .color(Colors::TEXT_SECONDARY)
                                        .size(13.0),
                                );
                                ui.label(
                                    egui::RichText::new(format!("{wind:.0} km/h"))
                                        .color(Colors::TEXT_PRIMARY)
                                        .size(14.0),
                                );
                            });
                        }

                        ui.add_space(10.0);

                        // Sun times
                        self.display_sun_times(ui, day);
                    });
                });

            ui.add_space(8.0);
        }
    }

    fn display_sun_times(&self, ui: &mut egui::Ui, day: &DailyForecast) {
        ui.vertical(|ui| {
            ui.set_width(70.0);
            if let Some(sunrise) = &day.sunrise {
                let time = extract_time(sunrise);
                ui.label(
                    egui::RichText::new(format!("Rise {time}"))
                        .color(Colors::ACCENT_YELLOW)
                        .size(12.0),
                );
            }
            if let Some(sunset) = &day.sunset {
                let time = extract_time(sunset);
                ui.label(
                    egui::RichText::new(format!("Set {time}"))
                        .color(Colors::ACCENT_ORANGE)
                        .size(12.0),
                );
            }
        });
    }
}

// Helper functions
fn weather_code_to_icon(code: i32) -> &'static str {
    match code {
        0 => "☀",        // Clear sky - sun (U+2600)
        1..=3 => "⛅",   // Partly cloudy (U+26C5)
        45 | 48 => "~",  // Fog - horizontal lines
        51..=57 => "☂",  // Drizzle - umbrella (U+2602)
        61..=67 => "☔", // Rain - umbrella with rain (U+2614)
        71..=77 => "❄",  // Snow (U+2744)
        80..=82 => "☔", // Rain showers - umbrella (U+2614)
        85 | 86 => "❄",  // Snow (U+2744)
        95..=99 => "⚡", // Thunderstorm - lightning bolt (U+26A1)
        _ => "○",        // Default - simple circle (U+25CB)
    }
}

fn weather_code_to_color(code: i32) -> egui::Color32 {
    match code {
        0 => Colors::ACCENT_YELLOW,      // Clear - yellow/sun
        1..=3 => Colors::TEXT_SECONDARY, // Partly cloudy - gray
        45 | 48 => Colors::TEXT_MUTED,   // Fog - muted
        51..=67 => Colors::ACCENT_CYAN,  // Drizzle/Rain - cyan
        71..=77 => Colors::ACCENT_CYAN,  // Snow - cyan
        80..=86 => Colors::ACCENT_CYAN,  // Showers - cyan
        95..=99 => Colors::ACCENT_BLUE,  // Thunderstorm - blue
        _ => Colors::TEXT_SECONDARY,     // Default
    }
}

fn weather_code_to_description(code: i32) -> &'static str {
    match code {
        0 => "Clear Sky",
        1 => "Mainly Clear",
        2 => "Partly Cloudy",
        3 => "Overcast",
        45 => "Foggy",
        48 => "Depositing Rime Fog",
        51 => "Light Drizzle",
        53 => "Moderate Drizzle",
        55 => "Dense Drizzle",
        61 => "Slight Rain",
        63 => "Moderate Rain",
        65 => "Heavy Rain",
        71 => "Slight Snow",
        73 => "Moderate Snow",
        75 => "Heavy Snow",
        80 => "Slight Rain Showers",
        81 => "Moderate Rain Showers",
        82 => "Violent Rain Showers",
        85 => "Slight Snow Showers",
        86 => "Heavy Snow Showers",
        95 => "Thunderstorm",
        96 => "Thunderstorm with Hail",
        99 => "Thunderstorm with Heavy Hail",
        _ => "Unknown",
    }
}

fn format_date(date_str: &str) -> String {
    let parts: Vec<&str> = date_str.split('-').collect();
    if parts.len() == 3 {
        let month = match parts[1] {
            "01" => "Jan",
            "02" => "Feb",
            "03" => "Mar",
            "04" => "Apr",
            "05" => "May",
            "06" => "Jun",
            "07" => "Jul",
            "08" => "Aug",
            "09" => "Sep",
            "10" => "Oct",
            "11" => "Nov",
            "12" => "Dec",
            _ => parts[1],
        };
        format!("{} {}", month, parts[2])
    } else {
        date_str.to_string()
    }
}

fn extract_time(datetime_str: &str) -> String {
    let parts: Vec<&str> = datetime_str.split('T').collect();
    if parts.len() > 1 {
        let time_parts: Vec<&str> = parts[1].split(':').collect();
        if time_parts.len() >= 2 {
            return format!("{}:{}", time_parts[0], time_parts[1]);
        }
    }
    datetime_str.to_string()
}

fn create_custom_visuals() -> egui::Visuals {
    let mut visuals = egui::Visuals::dark();

    // Override specific colors for better contrast
    visuals.widgets.noninteractive.bg_fill = Colors::BG_CARD;
    visuals.widgets.inactive.bg_fill = Colors::BG_CARD;
    visuals.widgets.hovered.bg_fill = Colors::BG_SECONDARY;
    visuals.widgets.active.bg_fill = Colors::ACCENT_BLUE;

    visuals.selection.bg_fill = Colors::ACCENT_BLUE;
    visuals.selection.stroke = egui::Stroke::new(1.0, Colors::ACCENT_CYAN);

    visuals.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, Colors::TEXT_PRIMARY);
    visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, Colors::TEXT_SECONDARY);

    visuals.extreme_bg_color = Colors::BG_PRIMARY;
    visuals.faint_bg_color = Colors::BG_SECONDARY;

    visuals
}
