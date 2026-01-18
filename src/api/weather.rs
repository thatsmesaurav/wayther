use crate::models::{
    AirPollutionResponse, CurrentWeather, CurrentWeatherResponse, DailyForecast, ForecastResponse,
    HourlyForecast, Location, WeatherData, WeatherDetails,
};
use chrono::{DateTime, Datelike, Local, TimeZone, Utc};
use std::collections::HashMap;

pub struct WeatherClient {
    api_key: String,
    client: reqwest::Client,
}

impl WeatherClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_weather(&self, location: &Location) -> Result<WeatherData, String> {
        let (current_response, forecast, aqi) = tokio::join!(
            self.get_current_weather_response(location),
            self.get_forecast(location),
            self.get_air_pollution(location)
        );

        let current_response = current_response?;
        let forecast = forecast?;
        let aqi = aqi.ok();

        let current = self.extract_current_weather(&current_response);
        let details = self.extract_weather_details(&current_response, aqi);
        let hourly = self.process_hourly_forecast(&forecast);
        let daily = self.process_daily_forecast(&forecast);

        Ok(WeatherData {
            current,
            details,
            hourly,
            daily,
        })
    }

    async fn get_current_weather_response(
        &self,
        location: &Location,
    ) -> Result<CurrentWeatherResponse, String> {
        let url = format!(
            "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}&units=metric",
            location.lat, location.lon, self.api_key
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API error: {}", response.status()));
        }

        response
            .json()
            .await
            .map_err(|e| format!("Parse error: {}", e))
    }

    async fn get_air_pollution(&self, location: &Location) -> Result<u32, String> {
        let url = format!(
            "https://api.openweathermap.org/data/2.5/air_pollution?lat={}&lon={}&appid={}",
            location.lat, location.lon, self.api_key
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API error: {}", response.status()));
        }

        let data: AirPollutionResponse = response
            .json()
            .await
            .map_err(|e| format!("Parse error: {}", e))?;

        data.list
            .first()
            .map(|item| item.main.aqi)
            .ok_or_else(|| "No AQI data".to_string())
    }

    fn extract_current_weather(&self, data: &CurrentWeatherResponse) -> CurrentWeather {
        let weather = data.weather.first().cloned().unwrap_or_else(|| {
            crate::models::WeatherCondition {
                id: 0,
                main: "Unknown".to_string(),
                description: "Unknown".to_string(),
                icon: "01d".to_string(),
            }
        });

        CurrentWeather {
            temp: data.main.temp,
            feels_like: data.main.feels_like,
            temp_min: data.main.temp_min,
            temp_max: data.main.temp_max,
            humidity: data.main.humidity,
            description: weather.description,
            icon: weather.icon,
        }
    }

    fn extract_weather_details(
        &self,
        data: &CurrentWeatherResponse,
        aqi: Option<u32>,
    ) -> WeatherDetails {
        let sunrise_dt = Utc.timestamp_opt(data.sys.sunrise, 0).unwrap();
        let sunrise_local: DateTime<Local> = sunrise_dt.into();
        let sunrise = sunrise_local.format("%l:%M %p").to_string().trim().to_string();

        let sunset_dt = Utc.timestamp_opt(data.sys.sunset, 0).unwrap();
        let sunset_local: DateTime<Local> = sunset_dt.into();
        let sunset = sunset_local.format("%l:%M %p").to_string().trim().to_string();

        WeatherDetails {
            wind_speed: data.wind.speed,
            wind_deg: data.wind.deg,
            humidity: data.main.humidity,
            feels_like: data.main.feels_like,
            visibility: data.visibility,
            sunrise,
            sunset,
            uv_index: None, // UV requires One Call API subscription
            aqi,
        }
    }

    async fn get_forecast(&self, location: &Location) -> Result<ForecastResponse, String> {
        let url = format!(
            "https://api.openweathermap.org/data/2.5/forecast?lat={}&lon={}&appid={}&units=metric",
            location.lat, location.lon, self.api_key
        );

        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("API error: {}", response.status()));
        }

        response
            .json()
            .await
            .map_err(|e| format!("Parse error: {}", e))
    }

    fn process_hourly_forecast(&self, forecast: &ForecastResponse) -> Vec<HourlyForecast> {
        forecast
            .list
            .iter()
            .take(24)
            .map(|item| {
                let dt = Utc.timestamp_opt(item.dt, 0).unwrap();
                let local: DateTime<Local> = dt.into();
                let time = local.format("%l%p").to_string().trim().to_string();

                let icon = item
                    .weather
                    .first()
                    .map(|w| w.icon.clone())
                    .unwrap_or_else(|| "01d".to_string());

                HourlyForecast {
                    time,
                    temp: item.main.temp,
                    icon,
                }
            })
            .collect()
    }

    fn process_daily_forecast(&self, forecast: &ForecastResponse) -> Vec<DailyForecast> {
        let mut daily_data: HashMap<u32, Vec<&crate::models::ForecastItem>> = HashMap::new();

        for item in &forecast.list {
            let dt = Utc.timestamp_opt(item.dt, 0).unwrap();
            let local: DateTime<Local> = dt.into();
            let day_of_year = local.ordinal();
            daily_data.entry(day_of_year).or_default().push(item);
        }

        let mut days: Vec<_> = daily_data.into_iter().collect();
        days.sort_by_key(|(day, _)| *day);

        days.into_iter()
            .take(10)
            .enumerate()
            .map(|(idx, (_, items))| {
                let temps: Vec<f64> = items.iter().map(|i| i.main.temp).collect();
                let temp_min = temps.iter().cloned().fold(f64::INFINITY, f64::min);
                let temp_max = temps.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

                let icon = items
                    .iter()
                    .find(|i| i.dt_txt.contains("12:00"))
                    .or(items.first())
                    .and_then(|i| i.weather.first())
                    .map(|w| w.icon.clone())
                    .unwrap_or_else(|| "01d".to_string());

                let day = if idx == 0 {
                    "Today".to_string()
                } else {
                    let dt = Utc.timestamp_opt(items[0].dt, 0).unwrap();
                    let local: DateTime<Local> = dt.into();
                    local.format("%a").to_string()
                };

                DailyForecast {
                    day,
                    temp_min,
                    temp_max,
                    icon,
                }
            })
            .collect()
    }

    pub fn get_icon_url(icon_code: &str) -> String {
        format!("https://openweathermap.org/img/wn/{}@2x.png", icon_code)
    }
}
