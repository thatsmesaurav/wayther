use crate::ui::WeatherWindow;
use crate::utils::{get_api_key, get_location};
use gtk4::prelude::*;
use gtk4::{glib, Application};

pub fn run() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("com.wayther.app")
        .build();

    app.connect_activate(|app| {
        let api_key = match get_api_key() {
            Some(key) => key,
            None => {
                eprintln!("Error: OpenWeatherMap API key not found!");
                eprintln!("Please set OPENWEATHERMAP_API_KEY environment variable");
                eprintln!("or create ~/.config/wayther/config.toml with:");
                eprintln!("  api_key = \"your_api_key_here\"");
                std::process::exit(1);
            }
        };

        let default_location = get_location();
        let weather_window = WeatherWindow::new(app, api_key, default_location);
        weather_window.window.present();
    });

    app.run()
}
