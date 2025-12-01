mod controllers;
mod models;
mod views;

use std::io;

fn main() {
    println!("Welcome to the Rust Weather App!");

    println!("Where are you?");

    let mut location = String::new();

    io::stdin()
        .read_line(&mut location)
        .expect("Failed to read line");

    let weather_info = models::weather_info::WeatherInfo::new(
        location.trim().to_string(),
        models::weather_info::WeatherData { temperature: 18.5 },
    );

    let cl_controller = controllers::cl_controller::ClController::new(weather_info);

    cl_controller.show_weather();
}
