mod api;
mod app;
mod models;
mod ui;
mod utils;

use std::env;

fn main() -> gtk4::glib::ExitCode {
    let args: Vec<String> = env::args().collect();

    // Check if "bar" argument is passed for waybar JSON output
    if args.len() > 1 && args[1] == "bar" {
        return run_waybar_mode();
    }

    // Initialize Tokio runtime for reqwest/hyper async HTTP operations
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to create Tokio runtime");

    // Keep runtime alive by entering its context
    let _guard = runtime.enter();

    app::run()
}

fn run_waybar_mode() -> gtk4::glib::ExitCode {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Failed to create Tokio runtime");

    runtime.block_on(async {
        let api_key = match utils::get_api_key() {
            Some(key) => key,
            None => {
                print_waybar_error("No API key");
                return gtk4::glib::ExitCode::FAILURE;
            }
        };

        let location_query = match utils::get_location() {
            Some(loc) => loc,
            None => {
                print_waybar_error("No location");
                return gtk4::glib::ExitCode::FAILURE;
            }
        };

        let geocoding_client = api::GeocodingClient::new(api_key.clone());
        let location = match geocoding_client.search_city(&location_query).await {
            Ok(locations) if !locations.is_empty() => locations.into_iter().next().unwrap(),
            _ => {
                print_waybar_error("Location not found");
                return gtk4::glib::ExitCode::FAILURE;
            }
        };

        let weather_client = api::WeatherClient::new(api_key);
        match weather_client.get_weather(&location).await {
            Ok(data) => {
                let icon = get_weather_icon(&data.current.icon);
                let temp = data.current.temp.round() as i32;

                let tooltip = format!(
                    "{}\n{}\nH: {}° L: {}°\nHumidity: {}%\nWind: {:.0} km/h",
                    location.display_name(),
                    capitalize(&data.current.description),
                    data.current.temp_max.round() as i32,
                    data.current.temp_min.round() as i32,
                    data.details.humidity,
                    data.details.wind_speed * 3.6
                );

                let class = get_weather_class(&data.current.icon);

                let output = serde_json::json!({
                    "text": format!("{} {}°", icon, temp),
                    "tooltip": tooltip,
                    "class": class
                });

                println!("{}", output);
                gtk4::glib::ExitCode::SUCCESS
            }
            Err(e) => {
                print_waybar_error(&e);
                gtk4::glib::ExitCode::FAILURE
            }
        }
    })
}

fn print_waybar_error(msg: &str) {
    let output = serde_json::json!({
        "text": " --°",
        "tooltip": msg,
        "class": "error"
    });
    println!("{}", output);
}

fn get_weather_icon(icon_code: &str) -> &'static str {
    match icon_code {
        "01d" => "",  // clear sky day
        "01n" => "",  // clear sky night
        "02d" => "",  // few clouds day
        "02n" => "",  // few clouds night
        "03d" | "03n" => "",  // scattered clouds
        "04d" | "04n" => "",  // broken clouds
        "09d" | "09n" => "",  // shower rain
        "10d" => "",  // rain day
        "10n" => "",  // rain night
        "11d" | "11n" => "",  // thunderstorm
        "13d" | "13n" => "",  // snow
        "50d" | "50n" => "",  // mist
        _ => "",
    }
}

fn get_weather_class(icon_code: &str) -> &'static str {
    match icon_code {
        "01d" | "01n" => "clear",
        "02d" | "02n" | "03d" | "03n" | "04d" | "04n" => "cloudy",
        "09d" | "09n" | "10d" | "10n" => "rainy",
        "11d" | "11n" => "stormy",
        "13d" | "13n" => "snowy",
        "50d" | "50n" => "foggy",
        _ => "unknown",
    }
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
