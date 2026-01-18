use crate::models::WeatherDetails;
use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, Grid, Label, Orientation};

pub struct WeatherDetailsWidget {
    pub container: GtkBox,
    wind_value: Label,
    humidity_value: Label,
    feels_like_value: Label,
    visibility_value: Label,
    sunrise_value: Label,
    sunset_value: Label,
    uv_value: Label,
    aqi_value: Label,
}

impl WeatherDetailsWidget {
    pub fn new() -> Self {
        let container = GtkBox::new(Orientation::Vertical, 0);
        container.set_valign(Align::Center);

        let grid = Grid::new();
        grid.set_row_spacing(16);
        grid.set_column_spacing(32);
        grid.set_halign(Align::Center);
        grid.set_valign(Align::Center);

        // Layout: 4 columns x 2 rows
        // Col 0: Feels Like / Humidity
        // Col 1: Wind / AQI
        // Col 2: Sunrise / Sunset
        // Col 3: Visibility / UV Index

        // Row 0: Feels Like, Wind, Sunrise, Visibility
        let (feels_row, feels_like_value) = Self::create_detail_row("Feels Like", "--");
        let (wind_row, wind_value) = Self::create_detail_row("Wind", "--");
        let (sunrise_row, sunrise_value) = Self::create_detail_row("Sunrise", "--");
        let (visibility_row, visibility_value) = Self::create_detail_row("Visibility", "--");
        grid.attach(&feels_row, 0, 0, 1, 1);
        grid.attach(&wind_row, 1, 0, 1, 1);
        grid.attach(&sunrise_row, 2, 0, 1, 1);
        grid.attach(&visibility_row, 3, 0, 1, 1);

        // Row 1: Humidity, AQI, Sunset, UV Index
        let (humidity_row, humidity_value) = Self::create_detail_row("Humidity", "--");
        let (aqi_row, aqi_value) = Self::create_detail_row("AQI", "--");
        let (sunset_row, sunset_value) = Self::create_detail_row("Sunset", "--");
        let (uv_row, uv_value) = Self::create_detail_row("UV Index", "--");
        grid.attach(&humidity_row, 0, 1, 1, 1);
        grid.attach(&aqi_row, 1, 1, 1, 1);
        grid.attach(&sunset_row, 2, 1, 1, 1);
        grid.attach(&uv_row, 3, 1, 1, 1);

        container.append(&grid);

        Self {
            container,
            wind_value,
            humidity_value,
            feels_like_value,
            visibility_value,
            sunrise_value,
            sunset_value,
            uv_value,
            aqi_value,
        }
    }

    fn create_detail_row(label_text: &str, default_value: &str) -> (GtkBox, Label) {
        let row = GtkBox::new(Orientation::Vertical, 2);
        row.set_halign(Align::Start);
        row.set_width_request(100);

        let label = Label::new(Some(label_text));
        label.add_css_class("detail-label");
        label.set_halign(Align::Start);

        let value = Label::new(Some(default_value));
        value.add_css_class("detail-value");
        value.set_halign(Align::Start);

        row.append(&label);
        row.append(&value);

        (row, value)
    }

    pub fn update(&self, details: &WeatherDetails) {
        // Wind speed in km/h (API returns m/s)
        let wind_kmh = details.wind_speed * 3.6;
        self.wind_value
            .set_text(&format!("{:.0} km/h", wind_kmh));

        self.humidity_value
            .set_text(&format!("{}%", details.humidity));

        self.feels_like_value
            .set_text(&format!("{}°", details.feels_like.round() as i32));

        // Visibility in km (API returns meters)
        if let Some(vis) = details.visibility {
            let vis_km = vis as f64 / 1000.0;
            self.visibility_value.set_text(&format!("{:.1} km", vis_km));
        } else {
            self.visibility_value.set_text("--");
        }

        self.sunrise_value.set_text(&details.sunrise);
        self.sunset_value.set_text(&details.sunset);

        if let Some(uv) = details.uv_index {
            self.uv_value.set_text(&format!("{:.1}", uv));
        } else {
            self.uv_value.set_text("--");
        }

        if let Some(aqi) = details.aqi {
            let aqi_text = match aqi {
                1 => "Good",
                2 => "Fair",
                3 => "Moderate",
                4 => "Poor",
                5 => "Very Poor",
                _ => "Unknown",
            };
            self.aqi_value.set_text(&format!("{} ({})", aqi, aqi_text));
        } else {
            self.aqi_value.set_text("--");
        }
    }
}

impl Default for WeatherDetailsWidget {
    fn default() -> Self {
        Self::new()
    }
}
