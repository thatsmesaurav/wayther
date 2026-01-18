use crate::models::HourlyForecast;
use crate::ui::IconLoader;
use gtk4::prelude::*;
use gtk4::{Align, Box as GtkBox, Frame, Image, Label, Orientation, ScrolledWindow};
use std::collections::HashMap;

pub struct HourlyForecastWidget {
    pub container: Frame,
    items_box: GtkBox,
    pub icon_images: Vec<Image>,
}

impl HourlyForecastWidget {
    pub fn new() -> Self {
        let container = Frame::new(Some("Hourly Forecast"));
        container.set_margin_start(12);
        container.set_margin_end(12);
        container.set_margin_bottom(12);

        let scrolled = ScrolledWindow::new();
        scrolled.set_hscrollbar_policy(gtk4::PolicyType::Automatic);
        scrolled.set_vscrollbar_policy(gtk4::PolicyType::Never);
        scrolled.set_min_content_height(120);

        let items_box = GtkBox::new(Orientation::Horizontal, 16);
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

    pub fn update(&mut self, forecast: &[HourlyForecast]) -> HashMap<usize, String> {
        while let Some(child) = self.items_box.first_child() {
            self.items_box.remove(&child);
        }
        self.icon_images.clear();

        let mut icon_codes = HashMap::new();

        for (idx, item) in forecast.iter().enumerate() {
            let item_box = GtkBox::new(Orientation::Vertical, 4);
            item_box.set_halign(Align::Center);

            let time_label = Label::new(Some(if idx == 0 { "Now" } else { &item.time }));
            time_label.add_css_class("hourly-time");

            let icon_image = Image::new();
            icon_image.set_pixel_size(40);

            let temp_label = Label::new(Some(&format!("{}°", item.temp.round() as i32)));
            temp_label.add_css_class("hourly-temp");

            item_box.append(&time_label);
            item_box.append(&icon_image);
            item_box.append(&temp_label);

            self.items_box.append(&item_box);
            self.icon_images.push(icon_image);
            icon_codes.insert(idx, item.icon.clone());
        }

        icon_codes
    }

    pub fn set_icon(&self, index: usize, data: &[u8]) {
        if let Some(image) = self.icon_images.get(index) {
            if let Some(pixbuf) = IconLoader::bytes_to_pixbuf(data, 40) {
                image.set_from_pixbuf(Some(&pixbuf));
            }
        }
    }
}

impl Default for HourlyForecastWidget {
    fn default() -> Self {
        Self::new()
    }
}
