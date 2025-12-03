use std::io::{self, Write};
use std::process;
/// Weather application entry point (CLI version)
///
/// Uses MVC architecture with a repository pattern:
/// - Models: Data structures for weather information
/// - Views: Console display logic
/// - Controllers: Coordinate between repository and view
/// - Repositories: Handle API data fetching
use weather_app::controllers::cl_controller::ClController;
use weather_app::repositories::weather_repository::ApiWeatherRepository;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

/// Main application logic with proper error handling
fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to the Rust Weather App!");
    print!("Where are you? ");
    io::stdout().flush()?;

    let mut location = String::new();
    io::stdin().read_line(&mut location)?;

    let repository = ApiWeatherRepository::new();
    let controller = ClController::new(repository);

    controller.show_weather(location.trim())?;
    Ok(())
}
