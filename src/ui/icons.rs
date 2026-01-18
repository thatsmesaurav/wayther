use gtk4::gdk_pixbuf::Pixbuf;
use gtk4::gio::MemoryInputStream;
use gtk4::glib::Bytes;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct IconLoader {
    cache: Arc<Mutex<HashMap<String, Vec<u8>>>>,
    client: reqwest::Client,
}

impl IconLoader {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
            client: reqwest::Client::new(),
        }
    }

    pub async fn load_icon(&self, icon_code: &str) -> Result<Vec<u8>, String> {
        let mut cache = self.cache.lock().await;

        if let Some(data) = cache.get(icon_code) {
            return Ok(data.clone());
        }

        let url = format!("https://openweathermap.org/img/wn/{}@2x.png", icon_code);

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Failed to fetch icon: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("Icon fetch failed: {}", response.status()));
        }

        let bytes = response
            .bytes()
            .await
            .map_err(|e| format!("Failed to read icon bytes: {}", e))?;

        let data = bytes.to_vec();
        cache.insert(icon_code.to_string(), data.clone());

        Ok(data)
    }

    pub fn bytes_to_pixbuf(data: &[u8], size: i32) -> Option<Pixbuf> {
        let bytes = Bytes::from(data);
        let stream = MemoryInputStream::from_bytes(&bytes);
        Pixbuf::from_stream_at_scale(&stream, size, size, true, gtk4::gio::Cancellable::NONE).ok()
    }
}

impl Default for IconLoader {
    fn default() -> Self {
        Self::new()
    }
}
