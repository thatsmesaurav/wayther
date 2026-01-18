use crate::api::GeocodingClient;
use crate::models::Location;
use gtk4::glib::object::IsA;
use gtk4::prelude::*;
use gtk4::{
    Align, Box as GtkBox, Button, Dialog, Entry, Label, ListBox, ListBoxRow, Orientation,
    ResponseType, ScrolledWindow, Window,
};
use std::cell::RefCell;
use std::rc::Rc;
use tokio::sync::mpsc;

pub struct SearchDialog;

impl SearchDialog {
    pub fn show(
        parent: &impl IsA<Window>,
        api_key: &str,
        on_select: impl Fn(Location) + 'static,
    ) {
        let dialog = Dialog::with_buttons(
            Some("Search City"),
            Some(parent),
            gtk4::DialogFlags::MODAL | gtk4::DialogFlags::DESTROY_WITH_PARENT,
            &[("Cancel", ResponseType::Cancel)],
        );
        dialog.set_default_size(400, 300);

        let content = dialog.content_area();
        content.set_spacing(12);
        content.set_margin_top(12);
        content.set_margin_bottom(12);
        content.set_margin_start(12);
        content.set_margin_end(12);

        let search_box = GtkBox::new(Orientation::Horizontal, 8);

        let entry = Entry::new();
        entry.set_placeholder_text(Some("Enter city name..."));
        entry.set_hexpand(true);

        let search_btn = Button::with_label("Search");

        search_box.append(&entry);
        search_box.append(&search_btn);
        content.append(&search_box);

        let scrolled = ScrolledWindow::new();
        scrolled.set_vexpand(true);
        scrolled.set_hscrollbar_policy(gtk4::PolicyType::Never);

        let list_box = ListBox::new();
        list_box.set_selection_mode(gtk4::SelectionMode::Single);
        scrolled.set_child(Some(&list_box));
        content.append(&scrolled);

        let status_label = Label::new(Some("Enter a city name to search"));
        status_label.set_halign(Align::Start);
        status_label.add_css_class("dim-label");
        content.append(&status_label);

        let results: Rc<RefCell<Vec<Location>>> = Rc::new(RefCell::new(Vec::new()));
        let api_key = api_key.to_string();

        let (tx, rx) = mpsc::channel::<Result<Vec<Location>, String>>(1);

        let do_search = {
            let entry = entry.clone();
            let list_box = list_box.clone();
            let status_label = status_label.clone();
            let results = results.clone();
            let api_key = api_key.clone();

            Rc::new(move || {
                let query = entry.text().to_string();
                if query.is_empty() {
                    status_label.set_text("Enter a city name to search");
                    return;
                }

                while let Some(child) = list_box.first_child() {
                    list_box.remove(&child);
                }
                results.borrow_mut().clear();
                status_label.set_text("Searching...");

                let client = GeocodingClient::new(api_key.clone());
                let tx = tx.clone();

                gtk4::glib::spawn_future_local(async move {
                    let result = client.search_city(&query).await;
                    let _ = tx.send(result).await;
                });
            })
        };

        {
            let do_search = do_search.clone();
            search_btn.connect_clicked(move |_| {
                do_search();
            });
        }

        {
            let do_search = do_search.clone();
            entry.connect_activate(move |_| {
                do_search();
            });
        }

        {
            let list_box = list_box.clone();
            let status_label = status_label.clone();
            let results = results.clone();
            let mut rx = rx;

            gtk4::glib::spawn_future_local(async move {
                while let Some(result) = rx.recv().await {
                    match result {
                        Ok(locations) => {
                            if locations.is_empty() {
                                status_label.set_text("No results found");
                            } else {
                                status_label
                                    .set_text(&format!("Found {} result(s)", locations.len()));

                                for location in &locations {
                                    let row = ListBoxRow::new();
                                    let label = Label::new(Some(&location.display_name()));
                                    label.set_halign(Align::Start);
                                    label.set_margin_top(8);
                                    label.set_margin_bottom(8);
                                    label.set_margin_start(8);
                                    row.set_child(Some(&label));
                                    list_box.append(&row);
                                }

                                *results.borrow_mut() = locations;
                            }
                        }
                        Err(e) => {
                            status_label.set_text(&format!("Error: {}", e));
                        }
                    }
                }
            });
        }

        let on_select = Rc::new(on_select);

        {
            let dialog_weak = dialog.downgrade();
            let results = results.clone();
            let on_select = on_select.clone();

            list_box.connect_row_activated(move |_, row| {
                let idx = row.index() as usize;
                if let Some(location) = results.borrow().get(idx).cloned() {
                    on_select(location);
                    if let Some(dialog) = dialog_weak.upgrade() {
                        dialog.close();
                    }
                }
            });
        }

        dialog.connect_response(|dialog, _| {
            dialog.close();
        });

        dialog.present();
    }
}
