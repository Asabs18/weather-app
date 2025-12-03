//! Console view for displaying weather information
//!
//! Displays data in both metric and imperial units for user convenience.

use crate::models::weather_info::WeatherInfo;
use crate::utils::conversions::{Distance, Pressure, Speed, Temperature};

/// Stateless view for console output
pub struct ClView;

impl ClView {
    /// Main display entry point
    pub fn display(weather_info: &WeatherInfo) {
        println!("\n=== Weather Report ===");
        println!("Location: {}", weather_info.location);
        println!(
            "Coordinates: {:.4}°N, {:.4}°E",
            weather_info.latitude, weather_info.longitude
        );

        Self::display_current_conditions(&weather_info.weather_data.current);
        Self::display_hourly_forecast(&weather_info.weather_data.hourly);
        Self::display_daily_forecast(&weather_info.weather_data.daily);

        println!("\n======================\n");
    }

    fn display_current_conditions(current: &crate::models::weather_info::CurrentWeather) {
        println!("\n--- Current Conditions ---");

        if let Some(temp) = current.temperature {
            let fahrenheit = Temperature::celsius_to_fahrenheit(temp);
            println!("Temperature: {temp:.1}°C / {fahrenheit:.1}°F");
        }

        if let Some(apparent) = current.apparent_temperature {
            let fahrenheit = Temperature::celsius_to_fahrenheit(apparent);
            println!("Feels Like: {apparent:.1}°C / {fahrenheit:.1}°F");
        }

        if let Some(humidity) = current.humidity {
            println!("Humidity: {humidity:.0}%");
        }

        if let Some(precip) = current.precipitation {
            let inches = Distance::mm_to_inches(precip);
            println!("Precipitation: {precip:.1} mm / {inches:.2} in");
        }

        if let Some(code) = current.weather_code {
            println!(
                "Weather Code: {} ({})",
                code,
                Self::weather_code_description(code)
            );
        }

        if let Some(speed) = current.wind_speed {
            let mph = Speed::kmh_to_mph(speed);
            print!("Wind: {speed:.1} km/h / {mph:.1} mph");
            if let Some(direction) = current.wind_direction {
                println!(
                    " from {}° ({})",
                    direction,
                    Self::wind_direction_name(direction)
                );
            } else {
                println!();
            }
        }

        if let Some(clouds) = current.cloud_cover {
            println!("Cloud Cover: {clouds:.0}%");
        }

        if let Some(pressure) = current.pressure {
            let inhg = Pressure::hpa_to_inhg(pressure);
            println!("Pressure: {pressure:.1} hPa / {inhg:.2} inHg");
        }

        if let Some(visibility) = current.visibility {
            let feet = Distance::meters_to_feet(visibility);
            println!("Visibility: {visibility:.0} meters / {feet:.0} feet");
        }
    }

    fn display_hourly_forecast(hourly: &[crate::models::weather_info::HourlyForecast]) {
        if hourly.is_empty() {
            return;
        }

        println!("\n--- Hourly Forecast (Next 24 Hours) ---");
        for (i, hour) in hourly.iter().enumerate() {
            if i >= 24 {
                break;
            }
            Self::display_hourly_item(hour);
        }
    }

    fn display_hourly_item(hour: &crate::models::weather_info::HourlyForecast) {
        let time_display = if let Some(t) = hour.time.split('T').nth(1) {
            t.split(':').next().unwrap_or(&hour.time)
        } else {
            &hour.time
        };

        print!("{time_display}:00 - ");

        if let Some(temp) = hour.temperature {
            let fahrenheit = Temperature::celsius_to_fahrenheit(temp);
            print!("{temp:.1}°C / {fahrenheit:.1}°F");
        }

        if let Some(code) = hour.weather_code {
            print!(" ({})", Self::weather_code_description(code));
        }

        if let Some(precip_prob) = hour.precipitation_probability {
            print!(" | Rain: {precip_prob:.0}%");
        }

        if let Some(precip) = hour.precipitation {
            if precip > 0.0 {
                let inches = Distance::mm_to_inches(precip);
                print!(" ({precip:.1}mm / {inches:.2}in)");
            }
        }

        if let Some(wind) = hour.wind_speed {
            let mph = Speed::kmh_to_mph(wind);
            print!(" | Wind: {wind:.0} km/h / {mph:.0} mph");
        }

        println!();
    }

    /// Displays daily forecast (next 7 days)
    fn display_daily_forecast(daily: &[crate::models::weather_info::DailyForecast]) {
        if daily.is_empty() {
            return;
        }

        println!("\n--- Daily Forecast (Next 7 Days) ---");
        for day in daily {
            Self::display_daily_item(day);
        }
    }

    fn display_daily_item(day: &crate::models::weather_info::DailyForecast) {
        println!("\n{}", day.date);

        if let (Some(max), Some(min)) = (day.temperature_max, day.temperature_min) {
            let min_f = Temperature::celsius_to_fahrenheit(min);
            let max_f = Temperature::celsius_to_fahrenheit(max);
            println!("  Temperature: {min:.1}°C to {max:.1}°C / {min_f:.1}°F to {max_f:.1}°F");
        }

        if let Some(code) = day.weather_code {
            println!("  Conditions: {}", Self::weather_code_description(code));
        }

        if let Some(precip_sum) = day.precipitation_sum {
            if precip_sum > 0.0 {
                let inches = Distance::mm_to_inches(precip_sum);
                print!("  Precipitation: {precip_sum:.1} mm / {inches:.2} in");
                if let Some(prob) = day.precipitation_probability {
                    print!(" ({prob}% chance)");
                }
                println!();
            }
        }

        if let Some(wind) = day.wind_speed_max {
            let mph = Speed::kmh_to_mph(wind);
            println!("  Max Wind Speed: {wind:.1} km/h / {mph:.1} mph");
        }

        if let (Some(sunrise), Some(sunset)) = (&day.sunrise, &day.sunset) {
            let sunrise_time = sunrise.split('T').nth(1).unwrap_or(sunrise);
            let sunset_time = sunset.split('T').nth(1).unwrap_or(sunset);
            println!("  Sunrise: {sunrise_time} | Sunset: {sunset_time}");
        }
    }

    /// Converts weather code to human-readable description
    fn weather_code_description(code: i32) -> &'static str {
        match code {
            0 => "Clear sky",
            1 => "Mainly clear",
            2 => "Partly cloudy",
            3 => "Overcast",
            45 | 48 => "Fog",
            51 | 53 | 55 => "Drizzle",
            61 | 63 | 65 => "Rain",
            71 | 73 | 75 => "Snow",
            77 => "Snow grains",
            80..=82 => "Rain showers",
            85 | 86 => "Snow showers",
            95 => "Thunderstorm",
            96 | 99 => "Thunderstorm with hail",
            _ => "Unknown",
        }
    }

    /// Converts wind direction degrees to cardinal direction
    fn wind_direction_name(degrees: f64) -> &'static str {
        let normalized = ((degrees % 360.0) + 360.0) % 360.0;
        match normalized {
            d if !(22.5..337.5).contains(&d) => "N",
            d if (22.5..67.5).contains(&d) => "NE",
            d if (67.5..112.5).contains(&d) => "E",
            d if (112.5..157.5).contains(&d) => "SE",
            d if (157.5..202.5).contains(&d) => "S",
            d if (202.5..247.5).contains(&d) => "SW",
            d if (247.5..292.5).contains(&d) => "W",
            d if (292.5..337.5).contains(&d) => "NW",
            _ => "Unknown",
        }
    }
}
