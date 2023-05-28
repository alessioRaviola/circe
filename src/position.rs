use serde::{de, Deserialize};

#[derive(Debug, Deserialize)]
pub struct LocationResponse(Vec<Location>);

#[derive(Debug, Deserialize)]
pub struct Location {
    pub display_name: String,
    #[serde(deserialize_with = "de_f64_string")]
    pub lat: f64,
    #[serde(deserialize_with = "de_f64_string")]
    pub lon: f64,
}

fn de_f64_string<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;
    s.parse::<f64>().map_err(de::Error::custom)
}

pub struct CircePosition;

impl CircePosition {
    const API: &str = "https://geocode.maps.co/search";

    pub async fn get_locations(query: &str) -> Result<Vec<Location>, reqwest::Error> {
        let url = format!("{}?q={}", Self::API, query);
        let response = reqwest::get(url).await?.json::<Vec<Location>>().await?;
        Ok(response)
    }
}
