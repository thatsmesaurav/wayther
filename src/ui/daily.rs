use crate::models::DailyForecast;
use crate::ui::IconLoader;
use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, Frame, Image, Label, Orientation, ProgressBar, ScrolledWindow};
use std::collections::HashMap;

pub struct DailyForecastWidget {
    pub container: Frame,
    items_box: GtkBox,
    pub icon_images: Vec<Image>,
}

impl DailyForecastWidget {
    pub fn new() -> Self {
        let container = Frame::new(Some("10-Day Forecast"));
        container.set_margin_start(12);
        container.set_margin_end(12);
        container.set_margin_bottom(12);
        container.set_vexpand(true);

        let scrolled = ScrolledWindow::new();
        scrolled.set_vscrollbar_policy(gtk4::PolicyType::Automatic);
        scrolled.set_hscrollbar_policy(gtk4::PolicyType::Never);
        scrolled.set_vexpand(true);
        scrolled.set_min_content_height(200);

        let items_box = GtkBox::new(Orientation::Vertical, 8);
        items_box.set_margin_top(12);
        items_box.set_margin_bottom(12);
        items_box.set_margin_start(12);
        items_box.set_margin_end(12);

        scrolled.set_child(Some(&items_box));
        container.set_child(Some(&scrolled));

        Self {
            container,
            items_box,
            icon_images: Vec::new(),
        }
    }

    pub fn update(&mut self, forecast: &[DailyForecast]) -> HashMap<usize, String> {
        while let Some(child) = self.items_box.first_child() {
            self.items_box.remove(&child);
        }
        self.icon_images.clear();

        let all_temps: Vec<f64> = forecast
            .iter()
            .flat_map(|f| vec![f.temp_min, f.temp_max])
            .collect();
        let global_min = all_temps.iter().cloned().fold(f64::INFINITY, f64::min);
        let global_max = all_temps.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let temp_range = global_max - global_min;

        let mut icon_codes = HashMap::new();

        for (idx, item) in forecast.iter().enumerate() {
            let row = GtkBox::new(Orientation::Horizontal, 8);
            row.set_valign(Align::Center);

            let day_label = Label::new(Some(&item.day));
            day_label.set_width_chars(6);
            day_label.set_halign(Align::Start);
            day_label.add_css_class("daily-day");

            let icon_image = Image::new();
            icon_image.set_pixel_size(32);

            let low_label = Label::new(Some(&format!("{}°", item.temp_min.round() as i32)));
            low_label.set_width_chars(4);
            low_label.add_css_class("daily-low");

            let progress = ProgressBar::new();
            progress.set_hexpand(true);
            if temp_range > 0.0 {
                let fraction = (item.temp_max - item.temp_min) / temp_range;
                progress.set_fraction(fraction.clamp(0.1, 1.0));
            } else {
                progress.set_fraction(0.5);
            }

            let high_label = Label::new(Some(&format!("{}°", item.temp_max.round() as i32)));
            high_label.set_width_chars(4);
            high_label.add_css_class("daily-high");

            row.append(&day_label);
            row.append(&icon_image);
            row.append(&low_label);
            row.append(&progress);
            row.append(&high_label);

            self.items_box.append(&row);
            self.icon_images.push(icon_image);
            icon_codes.insert(idx, item.icon.clone());
        }

        icon_codes
    }

    pub fn set_icon(&self, index: usize, data: &[u8]) {
        if let Some(image) = self.icon_images.get(index) {
            if let Some(pixbuf) = IconLoader::bytes_to_pixbuf(data, 32) {
                image.set_from_pixbuf(Some(&pixbuf));
            }
        }
    }
}

impl Default for DailyForecastWidget {
    fn default() -> Self {
        Self::new()
    }
}
