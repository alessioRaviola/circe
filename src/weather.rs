use std::fmt::Display;

use chrono::{DateTime, Local, TimeZone};
use serde::{Deserialize, Deserializer};

#[derive(Deserialize)]
pub struct WeatherResponse {
    pub current_weather: Weather,
    pub hourly: WeatherData,
}

#[derive(Deserialize)]
pub struct Weather {
    pub temperature: f32,
    pub windspeed: f32,
    pub weathercode: i32,
}

impl Display for Weather {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[#] T: {} | Wind: {} | Weather Code {} ",
            self.temperature, self.windspeed, self.weathercode
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct WeatherData {
    #[serde(deserialize_with = "deserialize_weather_date", rename = "time")]
    pub datetime: Vec<DateTime<Local>>,
    pub temperature_2m: Vec<f32>,
    pub apparent_temperature: Vec<f32>,
    pub precipitation: Vec<f32>,
    pub precipitation_probability: Vec<f32>,
    pub relativehumidity_2m: Vec<f32>,
}

pub fn deserialize_weather_date<'de, D>(deserializer: D) -> Result<Vec<DateTime<Local>>, D::Error>
where
    D: Deserializer<'de>,
{
    let strings: Vec<String> = Deserialize::deserialize(deserializer)?;
    strings
        .iter()
        .map(|s| {
            Local
                .datetime_from_str(s.trim(), "%Y-%m-%dT%H:%M")
                .map_err(serde::de::Error::custom)
        })
        .collect()
}
pub struct CirceWeather;

impl CirceWeather {
    const API: &str = "https://api.open-meteo.com/v1";

    pub async fn get_current_weather(
        lat: f32,
        lon: f32,
    ) -> Result<WeatherResponse, reqwest::Error> {
        let url = format!(
            "{}/forecast?timezone=auto&latitude={}&longitude={}&current_weather=true&hourly=temperature_2m,relativehumidity_2m,precipitation,precipitation_probability,apparent_temperature",
            Self::API,
            lat,
            lon
        );
        let response = reqwest::get(url).await?.json::<WeatherResponse>().await?;
        Ok(response)
    }
}
