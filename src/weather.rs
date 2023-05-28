use std::{fmt::Display, str::FromStr, string::ParseError};

use chrono::{DateTime, Local, TimeZone};
use serde::{de::Error, Deserialize, Deserializer};

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
    pub windspeed_10m: Vec<f32>,
    #[serde(deserialize_with = "deserialize_weathercode")]
    pub weathercode: Vec<WeatherCode>,
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

#[derive(Debug, Deserialize)]
pub enum WeatherCode {
    ClearSky,
    MainlyClear,
    PartlyCloudy,
    Overcast,
    Fog,
    DepositingRimeFog,
    DrizzleLight,
    DrizzleModerate,
    DrizzleDense,
    FreezingDrizzleLight,
    FreezingDrizzleDense,
    RainSlight,
    RainModerate,
    RainHeavy,
    FreezingRainLight,
    FreezingRainHeavy,
    SnowSlight,
    SnowModerate,
    SnowHeavy,
    SnowGrains,
    RainShowersSlight,
    RainShowersModerate,
    RainShowersViolent,
    SnowShowersSlight,
    SnowShowersHeavy,
    Thunderstorm,
    ThunderstormHailSlight,
    ThunderstormHailHeavy,
}

pub fn deserialize_weathercode<'de, D>(deserializer: D) -> Result<Vec<WeatherCode>, D::Error>
where
    D: Deserializer<'de>,
{
    let strings: Vec<i32> = Deserialize::deserialize(deserializer)?;
    strings
        .iter()
        .map(|s| WeatherCode::from_code(*s).map_err(serde::de::Error::custom))
        .collect()
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseWeatherCodeError;

impl Display for ParseWeatherCodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalide weather code.")
    }
}

impl WeatherCode {
    fn from_code(s: i32) -> Result<Self, ParseWeatherCodeError> {
        let r = match s {
            0 => Self::ClearSky,
            1 => Self::MainlyClear,
            2 => Self::PartlyCloudy,
            3 => Self::Overcast,
            45 => Self::Fog,
            48 => Self::DepositingRimeFog,
            51 => Self::DrizzleLight,
            53 => Self::DrizzleModerate,
            55 => Self::DrizzleDense,
            56 => Self::FreezingDrizzleLight,
            57 => Self::FreezingDrizzleDense,
            61 => Self::RainSlight,
            62 => Self::RainModerate,
            63 => Self::RainHeavy,
            66 => Self::FreezingRainLight,
            67 => Self::FreezingRainHeavy,
            71 => Self::SnowSlight,
            73 => Self::SnowModerate,
            75 => Self::SnowHeavy,
            77 => Self::SnowGrains,
            80 => Self::RainShowersSlight,
            81 => Self::RainShowersModerate,
            82 => Self::RainShowersViolent,
            85 => Self::SnowShowersSlight,
            86 => Self::SnowShowersHeavy,
            95 => Self::Thunderstorm,
            96 => Self::ThunderstormHailSlight,
            99 => Self::ThunderstormHailHeavy,
            _ => return Err(ParseWeatherCodeError),
        };
        Ok(r)
    }
}

impl Display for WeatherCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: &str = match self {
            WeatherCode::ClearSky => "Clear Sky",
            WeatherCode::MainlyClear => "Mainly Clear Sky",
            WeatherCode::PartlyCloudy => "Cloudy",
            WeatherCode::Overcast => "Overcast",
            WeatherCode::Fog => "Fog",
            WeatherCode::DepositingRimeFog => "Depositing Rime Fog",
            WeatherCode::DrizzleLight => "Light Drizzle",
            WeatherCode::DrizzleModerate => "Moderate Drizzle",
            WeatherCode::DrizzleDense => "Dense Drizzle",
            WeatherCode::FreezingDrizzleLight => "Light Freezing Drizzle",
            WeatherCode::FreezingDrizzleDense => "Dense Freezing Drizzle",
            WeatherCode::RainSlight => "Slight Rain",
            WeatherCode::RainModerate => "Moderate Rain",
            WeatherCode::RainHeavy => "Heavy Rain",
            WeatherCode::FreezingRainLight => "Light Freezing Rain",
            WeatherCode::FreezingRainHeavy => "Heavy Freezing Rain",
            WeatherCode::SnowSlight => "Slight Snow Fall",
            WeatherCode::SnowModerate => "Moderate Snow Fall",
            WeatherCode::SnowHeavy => "Heavy Snow Fall",
            WeatherCode::SnowGrains => "Snow Grains",
            WeatherCode::RainShowersSlight => "Slight Rain Showers",
            WeatherCode::RainShowersModerate => "Moderate Rain Showers",
            WeatherCode::RainShowersViolent => "Violent Rain Showers",
            WeatherCode::SnowShowersSlight => "Slight Snow Showers",
            WeatherCode::SnowShowersHeavy => "Heavy Snow Showers",
            WeatherCode::Thunderstorm => "Thunderstorm",
            WeatherCode::ThunderstormHailSlight => "Slight Hail Thunderstorm",
            WeatherCode::ThunderstormHailHeavy => "Heavy Hail Thunderstorm",
        };
        write!(f, "{}", s)
    }
}

const API: &str = "https://api.open-meteo.com/v1";

pub async fn get_current_weather(lat: f32, lon: f32) -> Result<WeatherResponse, reqwest::Error> {
    let url = format!(
        "{}/forecast?timezone=auto&latitude={}&longitude={}&current_weather=true&hourly=temperature_2m,weathercode,windspeed_10m,relativehumidity_2m,precipitation,precipitation_probability,apparent_temperature",
        API,
        lat,
        lon
    );
    let response = reqwest::get(url).await?.json::<WeatherResponse>().await?;
    Ok(response)
}
