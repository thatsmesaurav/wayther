# Wayther

A lightweight weather application for Wayland compositors, designed to work seamlessly with Waybar and Sway.

## Features

- **Native Wayland support** - Built with GTK4 and gtk4-layer-shell
- **Waybar integration** - JSON output mode for custom waybar modules
- **Current conditions** - Temperature, feels like, humidity, wind, visibility, AQI, sunrise/sunset
- **Hourly forecast** - 24-hour forecast with horizontal scrolling
- **10-day forecast** - Extended forecast with temperature range bars
- **Keyboard navigation** - Press `Escape` to close the window
- **Customizable styling** - Override styles with your own CSS

## Installation

### Dependencies

- GTK4
- gtk4-layer-shell
- Rust toolchain

### Building from source

```bash
git clone https://github.com/yourusername/wayther.git
cd wayther
cargo build --release
```

The binary will be available at `target/release/wayther`.

## Configuration

Create a configuration file at `~/.config/wayther/config.toml`:

```toml
api_key = "your_openweathermap_api_key"
location = "New York, US"
```

### Getting an API key

1. Sign up at [OpenWeatherMap](https://openweathermap.org/api)
2. Generate a free API key
3. Add it to your config file or set the `OPENWEATHERMAP_API_KEY` environment variable

## Usage

### Standalone application

```bash
wayther
```

This opens the weather widget anchored to the top-right corner of your screen.

### Waybar integration

Run wayther in bar mode to output JSON for waybar:

```bash
wayther bar
```

Example output:
```json
{"class":"cloudy","text":" -5°","tooltip":"New York, US\nScattered clouds\nH: 2° L: -5°\nHumidity: 67%\nWind: 23 km/h"}
```

#### Waybar configuration

Add this to your waybar `config.jsonc`:

```jsonc
"custom/weather": {
    "exec": "wayther bar",
    "return-type": "json",
    "interval": 600,
    "tooltip": true,
    "on-click": "wayther"
}
```

Add `"custom/weather"` to your modules list:

```jsonc
"modules-right": ["custom/weather", "clock", "tray"]
```

#### Waybar CSS styling

Add to your waybar `style.css`:

```css
#custom-weather {
    font-family: "JetBrainsMono Nerd Font", monospace;
    font-size: 14px;
    padding: 0 10px;
}

#custom-weather.clear {
    color: #f9e2af;
}

#custom-weather.cloudy {
    color: #9399b2;
}

#custom-weather.rainy {
    color: #89b4fa;
}

#custom-weather.stormy {
    color: #cba6f7;
}

#custom-weather.snowy {
    color: #f5c2e7;
}

#custom-weather.foggy {
    color: #a6adc8;
}

#custom-weather.error {
    color: #f38ba8;
}
```

## Sway configuration

Add a keybinding to toggle the weather widget:

```bash
# ~/.config/sway/config
bindsym $mod+w exec wayther
```

## Custom styling

You can override the default GTK styles by creating `~/.config/wayther/style.css`:

```css
.widget-container {
    background-color: rgba(30, 30, 46, 0.95);
    border-radius: 16px;
    padding: 20px;
}

.city-label {
    font-size: 22px;
    font-weight: 600;
    color: #cdd6f4;
}

.temperature-hero {
    font-size: 72px;
    font-weight: 300;
    color: #cdd6f4;
}

.description {
    font-size: 14px;
    color: #a6adc8;
}

.detail-label {
    font-size: 11px;
    color: #6c7086;
}

.detail-value {
    font-size: 15px;
    color: #cdd6f4;
}

.hourly-time {
    font-size: 12px;
    color: #a6adc8;
}

.hourly-temp {
    font-size: 14px;
    color: #cdd6f4;
}

.daily-day {
    color: #cdd6f4;
}

.daily-high {
    color: #cdd6f4;
}

.daily-low {
    color: #6c7086;
}

progressbar trough {
    background-color: #313244;
}

progressbar progress {
    background: linear-gradient(to right, #89b4fa, #f9e2af);
}
```

## Weather icons

The waybar module uses Nerd Font icons:

| Condition | Day | Night |
|-----------|-----|-------|
| Clear |  |  |
| Few clouds |  |  |
| Scattered clouds |  |  |
| Broken clouds |  |  |
| Shower rain |  |  |
| Rain |  |  |
| Thunderstorm |  |  |
| Snow |  |  |
| Mist |  |  |

Make sure you have a [Nerd Font](https://www.nerdfonts.com/) installed for icons to display correctly.

