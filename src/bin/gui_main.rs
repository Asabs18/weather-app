use eframe::egui;
use weather_app::views::gui_view::WeatherApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("Weather App"),
        ..Default::default()
    };

    eframe::run_native(
        "Weather App",
        options,
        Box::new(|cc| {
            // Setup fonts with better Unicode/emoji support
            setup_custom_fonts(&cc.egui_ctx);
            Ok(Box::new(WeatherApp::default()))
        }),
    )
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // Prioritize emoji fonts in the font families
    // egui's default fonts already include good emoji support,
    // we just need to make sure emoji font comes first for symbols
    if let Some(proportional) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
        // Move emoji font to the front if it exists
        proportional.insert(0, "emoji-icon-font".to_owned());
    }

    if let Some(monospace) = fonts.families.get_mut(&egui::FontFamily::Monospace) {
        monospace.insert(0, "emoji-icon-font".to_owned());
    }

    ctx.set_fonts(fonts);
}
