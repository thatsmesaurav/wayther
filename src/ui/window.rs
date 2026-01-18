use crate::api::{GeocodingClient, WeatherClient};
use crate::models::{Location, WeatherData};
use crate::ui::{
    CurrentWeatherWidget, DailyForecastWidget, HourlyForecastWidget, IconLoader,
    WeatherDetailsWidget,
};
use gtk4::prelude::*;
use gtk4::{ApplicationWindow, Box as GtkBox, CssProvider, Label, Orientation, ScrolledWindow};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use tokio::sync::mpsc;

pub struct WeatherWindow {
    pub window: ApplicationWindow,
}

struct AppState {
    current_widget: CurrentWeatherWidget,
    details_widget: WeatherDetailsWidget,
    hourly_widget: RefCell<HourlyForecastWidget>,
    daily_widget: RefCell<DailyForecastWidget>,
    city_label: Label,
    current_location: RefCell<Option<Location>>,
    api_key: String,
    icon_loader: Arc<IconLoader>,
}

impl WeatherWindow {
    pub fn new(app: &gtk4::Application, api_key: String, default_location: Option<String>) -> Self {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Wayther")
            .default_width(700)
            .default_height(700)
            .build();

        // Initialize layer shell
        window.init_layer_shell();
        window.set_layer(Layer::Top);
        window.set_keyboard_mode(KeyboardMode::Exclusive);

        // Anchor to top-right corner
        window.set_anchor(Edge::Top, true);
        window.set_anchor(Edge::Right, true);

        // Margin from edges (adjust for your waybar height)
        window.set_margin(Edge::Top, 10);
        window.set_margin(Edge::Right, 10);

        // Remove default background and add our class for transparent styling
        window.remove_css_class("background");
        window.add_css_class("wayther-window");

        Self::setup_css();

        // Close on Escape key
        let key_controller = gtk4::EventControllerKey::new();
        let window_clone = window.clone();
        key_controller.connect_key_pressed(move |_, key, _, _| {
            if key == gtk4::gdk::Key::Escape {
                window_clone.close();
                gtk4::glib::Propagation::Stop
            } else {
                gtk4::glib::Propagation::Proceed
            }
        });
        window.add_controller(key_controller);

        let main_box = GtkBox::new(Orientation::Vertical, 0);
        main_box.add_css_class("widget-container");

        // City label at top
        let city_label = Label::new(Some(""));
        city_label.add_css_class("city-label");
        city_label.set_margin_top(16);
        city_label.set_margin_bottom(8);
        main_box.append(&city_label);

        // Main scrollable content
        let scrolled = ScrolledWindow::new();
        scrolled.set_vexpand(true);
        scrolled.set_hscrollbar_policy(gtk4::PolicyType::Never);

        let content_box = GtkBox::new(Orientation::Vertical, 16);
        content_box.set_margin_start(12);
        content_box.set_margin_end(12);

        // Top section: Current weather (left) + Details (right)
        let top_section = GtkBox::new(Orientation::Horizontal, 16);
        top_section.set_margin_top(16);
        top_section.set_margin_bottom(16);
        top_section.set_halign(gtk4::Align::Fill);

        let current_widget = CurrentWeatherWidget::new();
        top_section.append(&current_widget.container);

        let details_widget = WeatherDetailsWidget::new();
        details_widget.container.set_hexpand(true);
        top_section.append(&details_widget.container);

        content_box.append(&top_section);

        // Hourly forecast section
        let hourly_widget = HourlyForecastWidget::new();
        content_box.append(&hourly_widget.container);

        // Daily forecast section
        let daily_widget = DailyForecastWidget::new();
        content_box.append(&daily_widget.container);

        scrolled.set_child(Some(&content_box));
        main_box.append(&scrolled);

        window.set_child(Some(&main_box));

        let state = Rc::new(AppState {
            current_widget,
            details_widget,
            hourly_widget: RefCell::new(hourly_widget),
            daily_widget: RefCell::new(daily_widget),
            city_label,
            current_location: RefCell::new(None),
            api_key,
            icon_loader: Arc::new(IconLoader::new()),
        });

        let (weather_tx, weather_rx) = mpsc::channel::<Result<WeatherData, String>>(1);
        let weather_tx = Rc::new(weather_tx);

        Self::setup_weather_receiver(state.clone(), weather_rx);

        if let Some(location_query) = default_location {
            let state = state.clone();
            let weather_tx = weather_tx.clone();

            gtk4::glib::spawn_future_local(async move {
                let geocoding_client = GeocodingClient::new(state.api_key.clone());
                match geocoding_client.search_city(&location_query).await {
                    Ok(locations) if !locations.is_empty() => {
                        let location = locations.into_iter().next().unwrap();
                        state.city_label.set_text(&location.display_name());
                        *state.current_location.borrow_mut() = Some(location.clone());
                        Self::load_weather(&state.api_key, location, weather_tx);
                    }
                    Ok(_) => {
                        eprintln!("No location found for: {}", location_query);
                    }
                    Err(e) => {
                        eprintln!("Failed to geocode default location: {}", e);
                    }
                }
            });
        }

        Self { window }
    }

    fn load_weather(
        api_key: &str,
        location: Location,
        tx: Rc<mpsc::Sender<Result<WeatherData, String>>>,
    ) {
        let client = WeatherClient::new(api_key.to_string());

        gtk4::glib::spawn_future_local(async move {
            let result = client.get_weather(&location).await;
            let _ = tx.send(result).await;
        });
    }

    fn setup_weather_receiver(
        state: Rc<AppState>,
        mut weather_rx: mpsc::Receiver<Result<WeatherData, String>>,
    ) {
        gtk4::glib::spawn_future_local(async move {
            while let Some(result) = weather_rx.recv().await {
                match result {
                    Ok(data) => {
                        state.current_widget.update(&data.current);
                        state.details_widget.update(&data.details);

                        {
                            let icon_code = data.current.icon.clone();
                            let loader = state.icon_loader.clone();
                            let container = state.current_widget.container.clone();

                            gtk4::glib::spawn_future_local(async move {
                                if let Ok(icon_data) = loader.load_icon(&icon_code).await {
                                    if let Some(pixbuf) = IconLoader::bytes_to_pixbuf(&icon_data, 80)
                                    {
                                        if let Some(image) = container
                                            .first_child()
                                            .and_then(|w| w.downcast::<gtk4::Image>().ok())
                                        {
                                            image.set_from_pixbuf(Some(&pixbuf));
                                        }
                                    }
                                }
                            });
                        }

                        let hourly_icons = state.hourly_widget.borrow_mut().update(&data.hourly);
                        for (idx, icon_code) in hourly_icons {
                            let loader = state.icon_loader.clone();
                            let widget = state.hourly_widget.borrow();

                            if let Some(image) = widget.icon_images.get(idx).cloned() {
                                let image: gtk4::Image = image;
                                gtk4::glib::spawn_future_local(async move {
                                    if let Ok(icon_data) = loader.load_icon(&icon_code).await {
                                        if let Some(pixbuf) =
                                            IconLoader::bytes_to_pixbuf(&icon_data, 40)
                                        {
                                            image.set_from_pixbuf(Some(&pixbuf));
                                        }
                                    }
                                });
                            }
                        }

                        let daily_icons = state.daily_widget.borrow_mut().update(&data.daily);
                        for (idx, icon_code) in daily_icons {
                            let loader = state.icon_loader.clone();
                            let widget = state.daily_widget.borrow();

                            if let Some(image) = widget.icon_images.get(idx).cloned() {
                                let image: gtk4::Image = image;
                                gtk4::glib::spawn_future_local(async move {
                                    if let Ok(icon_data) = loader.load_icon(&icon_code).await {
                                        if let Some(pixbuf) =
                                            IconLoader::bytes_to_pixbuf(&icon_data, 32)
                                        {
                                            image.set_from_pixbuf(Some(&pixbuf));
                                        }
                                    }
                                });
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Weather fetch error: {}", e);
                    }
                }
            }
        });
    }

    fn setup_css() {
        let provider = CssProvider::new();

        // Try loading user CSS from ~/.config/wayther/style.css
        let user_css_loaded = dirs::config_dir()
            .map(|p| p.join("wayther/style.css"))
            .filter(|p| p.exists())
            .map(|path| {
                provider.load_from_path(&path);
                true
            })
            .unwrap_or(false);

        // Fall back to default embedded CSS
        if !user_css_loaded {
            provider.load_from_data(Self::default_css());
        }

        gtk4::style_context_add_provider_for_display(
            &gtk4::gdk::Display::default().expect("Could not get display"),
            &provider,
            gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    fn default_css() -> &'static str {
        r#"
            .wayther-window,
            .wayther-window .background {
                background-color: transparent;
            }
            box {
                background: none;
            }
            .widget-container {
                padding: 16px;
                border-radius: 12px;
                background-color: @window_bg_color;
            }
            .city-label {
                font-size: 24px;
                font-weight: 600;
            }
            .temperature-hero {
                font-size: 80px;
                font-weight: 300;
            }
            .temperature-large {
                font-size: 72px;
                font-weight: 200;
            }
            .description-large {
                font-size: 20px;
                opacity: 0.8;
            }
            .description {
                font-size: 13px;
                opacity: 0.7;
            }
            .high-low {
                font-size: 16px;
                opacity: 0.7;
            }
            .high-low-small {
                font-size: 14px;
                opacity: 0.6;
            }
            .detail-label {
                font-size: 12px;
                opacity: 0.6;
                font-weight: 500;
            }
            .detail-value {
                font-size: 16px;
                font-weight: 500;
            }
            .hourly-time {
                font-size: 12px;
                opacity: 0.8;
            }
            .hourly-temp {
                font-size: 14px;
                font-weight: 500;
            }
            .daily-day {
                font-weight: 500;
            }
            .daily-low {
                opacity: 0.6;
            }
            .daily-high {
                font-weight: 500;
            }
            frame > label {
                font-weight: bold;
                opacity: 0.7;
            }
            progressbar trough {
                min-height: 6px;
                border-radius: 3px;
            }
            progressbar progress {
                min-height: 6px;
                border-radius: 3px;
                background: linear-gradient(to right, #4A90D9, #F5A623);
            }
        "#
    }
}
