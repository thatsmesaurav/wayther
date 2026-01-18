use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CurrentWeatherResponse {
    pub main: MainData,
    pub weather: Vec<WeatherCondition>,
    pub wind: WindData,
    pub visibility: Option<u32>,
    pub sys: SysData,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MainData {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub humidity: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WindData {
    pub speed: f64,
    #[serde(default)]
    pub deg: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SysData {
    pub sunrise: i64,
    pub sunset: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WeatherCondition {
    pub id: u32,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AirPollutionResponse {
    pub list: Vec<AirPollutionItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AirPollutionItem {
    pub main: AqiData,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AqiData {
    pub aqi: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UvResponse {
    pub value: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ForecastResponse {
    pub list: Vec<ForecastItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ForecastItem {
    pub dt: i64,
    pub main: MainData,
    pub weather: Vec<WeatherCondition>,
    pub dt_txt: String,
}

#[derive(Debug, Clone)]
pub struct HourlyForecast {
    pub time: String,
    pub temp: f64,
    pub icon: String,
}

#[derive(Debug, Clone)]
pub struct DailyForecast {
    pub day: String,
    pub temp_min: f64,
    pub temp_max: f64,
    pub icon: String,
}

#[derive(Debug, Clone)]
pub struct WeatherData {
    pub current: CurrentWeather,
    pub details: WeatherDetails,
    pub hourly: Vec<HourlyForecast>,
    pub daily: Vec<DailyForecast>,
}

#[derive(Debug, Clone)]
pub struct CurrentWeather {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub humidity: u32,
    pub description: String,
    pub icon: String,
}

#[derive(Debug, Clone)]
pub struct WeatherDetails {
    pub wind_speed: f64,
    pub wind_deg: Option<u32>,
    pub humidity: u32,
    pub feels_like: f64,
    pub visibility: Option<u32>,
    pub sunrise: String,
    pub sunset: String,
    pub uv_index: Option<f64>,
    pub aqi: Option<u32>,
}
