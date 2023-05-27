use ansi_term::Style;
use serde::{de, Deserialize};
use term_table::{
    row::Row,
    table_cell::{Alignment, TableCell},
    Table, TableStyle,
};

use crate::{weather::WeatherData, CirceWeather};

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

pub struct Circe;

impl Circe {
    pub async fn current_weather(location: &str, max: usize) {
        let locs = match CircePosition::get_locations(location).await {
            Ok(val) => val,
            Err(e) => {
                println!("No locations were found with the given name. ({})", e);
                return;
            }
        };
        let loc = locs.first().unwrap();

        let title_style = ansi_term::Style::new().bold();
        println!(
            "{}",
            title_style.paint(format!("== {} ==", loc.display_name))
        );

        let weather = CirceWeather::get_current_weather(loc.lat as f32, loc.lon as f32)
            .await
            .unwrap();

        print_weather(weather.hourly, max);
    }
}

fn print_weather(weather_data: WeatherData, max: usize) {
    let mut table = Table::new();
    table.max_column_width = 40;

    table.style = TableStyle::extended();

    let title_style = ansi_term::Style::new().bold().fg(ansi_term::Color::Yellow);
    let normal_style = ansi_term::Style::new();
    let _red_style = ansi_term::Style::new().fg(ansi_term::Color::Red);
    let cyan_style = ansi_term::Style::new().fg(ansi_term::Color::Cyan);

    table.add_row(Row::new(vec![
        TableCell::new_with_alignment(title_style.paint("Time"), 1, Alignment::Center),
        TableCell::new_with_alignment(title_style.paint("Temperature"), 1, Alignment::Center),
        TableCell::new_with_alignment(title_style.paint("Rain"), 1, Alignment::Center),
    ]));

    let mut i = 0;
    let mut j = 0;
    while j < max && i < weather_data.datetime.len() {
        if weather_data.datetime[i + 1] >= chrono::offset::Local::now() {
            j += 1;
        } else {
            i += 1;
            continue;
        }

        table.add_row(Row::new(vec![
            TableCell::new_with_alignment(
                weather_data.datetime[i].naive_local().format("%d/%m %H:%M"),
                1,
                Alignment::Left,
            ),
            TableCell::new_with_alignment(
                normal_style.paint(format!("{}°C", weather_data.temperature_2m[i])),
                1,
                Alignment::Right,
            ),
            TableCell::new_with_alignment(
                style_if_greater(weather_data.rain[i], 0.0, cyan_style, normal_style)
                    .paint(format!("{}mm", weather_data.rain[i])),
                1,
                Alignment::Right,
            ),
        ]));

        i += 1;
    }

    println!("{}", table.render());
}

fn style_if_greater(data: f32, than: f32, gr: Style, ls: Style) -> ansi_term::Style {
    if data > than {
        gr
    } else {
        ls
    }
}
