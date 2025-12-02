// Main entry point for the weather application (src/main.rs)

mod controllers;
mod models;
mod repositories;
mod views;

use controllers::cl_controller::ClController;
use repositories::weather_repository::ApiWeatherRepository;
use std::io::{self, Write};

/// Entry point for the weather application
fn main() {
    println!("Welcome to the Rust Weather App!");
    print!("Where are you? ");
    io::stdout().flush().unwrap();

    // Read user input for location
    let mut location = String::new();
    io::stdin()
        .read_line(&mut location)
        .expect("Failed to read line");

    // Initialize repository and controller
    let repository = ApiWeatherRepository::new();
    let controller = ClController::new(repository);

    // Display weather information
    controller.show_weather(location.trim());
}
