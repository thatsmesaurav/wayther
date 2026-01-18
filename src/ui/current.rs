use crate::models::CurrentWeather;
use crate::ui::IconLoader;
use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, Image, Label, Orientation};

pub struct CurrentWeatherWidget {
    pub container: GtkBox,
    temp_label: Label,
    desc_label: Label,
    high_low_label: Label,
    icon_image: Image,
}

impl CurrentWeatherWidget {
    pub fn new() -> Self {
        let container = GtkBox::new(Orientation::Vertical, 2);
        container.set_halign(Align::Center);
        container.set_valign(Align::Center);
        container.set_width_request(160);

        let icon_image = Image::new();
        icon_image.set_pixel_size(80);

        let temp_label = Label::new(Some("--°"));
        temp_label.add_css_class("temperature-hero");

        let desc_label = Label::new(Some("Loading..."));
        desc_label.add_css_class("description");

        let high_low_label = Label::new(Some("H:--° L:--°"));
        high_low_label.add_css_class("high-low-small");

        container.append(&icon_image);
        container.append(&temp_label);
        container.append(&desc_label);
        container.append(&high_low_label);

        Self {
            container,
            temp_label,
            desc_label,
            high_low_label,
            icon_image,
        }
    }

    pub fn update(&self, weather: &CurrentWeather) {
        self.temp_label
            .set_text(&format!("{}°", weather.temp.round() as i32));
        self.desc_label.set_text(&capitalize(&weather.description));
        self.high_low_label.set_text(&format!(
            "H:{}° L:{}°",
            weather.temp_max.round() as i32,
            weather.temp_min.round() as i32
        ));
    }

    pub fn set_icon(&self, data: &[u8]) {
        if let Some(pixbuf) = IconLoader::bytes_to_pixbuf(data, 80) {
            self.icon_image.set_from_pixbuf(Some(&pixbuf));
        }
    }
}

impl Default for CurrentWeatherWidget {
    fn default() -> Self {
        Self::new()
    }
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
