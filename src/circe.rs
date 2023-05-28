use ansi_term::Style;
use term_table::{
    row::Row,
    table_cell::{Alignment, TableCell},
    Table, TableStyle,
};

use crate::{get_current_weather, get_locations, weather::WeatherData};

pub async fn current_weather(location: &str, max: usize, short: bool) {
    let locs = match get_locations(location).await {
        Ok(val) => val,
        Err(e) => {
            println!("No locations were found with the given name. ({})", e);
            return;
        }
    };
    let loc = locs.first().unwrap();

    let weather = get_current_weather(loc.lat as f32, loc.lon as f32)
        .await
        .unwrap();

    if short {
        print_weather_short(loc.display_name.clone(), weather.hourly, max)
    } else {
        print_weather(loc.display_name.clone(), weather.hourly, max);
    }
}

fn print_weather(location: String, weather_data: WeatherData, max: usize) {
    let mut table = Table::new();
    table.max_column_width = 40;

    table.style = TableStyle::extended();

    let title_style = ansi_term::Style::new().bold().fg(ansi_term::Color::Yellow);
    let normal_style = ansi_term::Style::new();
    let cyan_style = ansi_term::Style::new().fg(ansi_term::Color::Cyan);

    table.add_row(Row::new(vec![TableCell::new_with_alignment(
        title_style.paint(location),
        8,
        Alignment::Center,
    )]));

    table.add_row(Row::new(vec![
        TableCell::new_with_alignment("", 2, Alignment::Center),
        TableCell::new_with_alignment(title_style.paint("Temperature"), 2, Alignment::Center),
        TableCell::new_with_alignment(title_style.paint("Precipitations"), 2, Alignment::Center),
        TableCell::new_with_alignment(title_style.paint("Humidity"), 1, Alignment::Center),
        TableCell::new_with_alignment(title_style.paint("Wind"), 1, Alignment::Center),
    ]));

    table.add_row(Row::new(vec![
        TableCell::new_with_alignment(title_style.paint("Time"), 1, Alignment::Center),
        TableCell::new_with_alignment(title_style.paint("Forecast"), 1, Alignment::Center),
        TableCell::new_with_alignment(title_style.paint("At 2m"), 1, Alignment::Center),
        TableCell::new_with_alignment(title_style.paint("Perceived"), 1, Alignment::Center),
        TableCell::new_with_alignment(title_style.paint("Probability"), 1, Alignment::Center),
        TableCell::new_with_alignment(title_style.paint("Quantity"), 1, Alignment::Center),
        TableCell::new_with_alignment(title_style.paint("Relative"), 1, Alignment::Center),
        TableCell::new_with_alignment(title_style.paint("At 10m"), 1, Alignment::Center),
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
            TableCell::new_with_alignment(&weather_data.weathercode[i], 1, Alignment::Left),
            TableCell::new_with_alignment(
                normal_style.paint(format!("{:.1}°C", weather_data.temperature_2m[i])),
                1,
                Alignment::Right,
            ),
            TableCell::new_with_alignment(
                normal_style.paint(format!("{:.1}°C", weather_data.apparent_temperature[i])),
                1,
                Alignment::Right,
            ),
            TableCell::new_with_alignment(
                style_if_greater(
                    weather_data.precipitation_probability[i + 1],
                    0.0,
                    cyan_style,
                    normal_style,
                )
                .paint(format!(
                    "{:.1}%",
                    weather_data.precipitation_probability[i + 1]
                )),
                1,
                Alignment::Right,
            ),
            TableCell::new_with_alignment(
                style_if_greater(
                    weather_data.precipitation[i + 1],
                    0.0,
                    cyan_style,
                    normal_style,
                )
                .paint(format!("{:.1}mm", weather_data.precipitation[i + 1])),
                1,
                Alignment::Right,
            ),
            TableCell::new_with_alignment(
                normal_style.paint(format!("{:.1}%", weather_data.relativehumidity_2m[i])),
                1,
                Alignment::Right,
            ),
            TableCell::new_with_alignment(
                normal_style.paint(format!("{:.1}%", weather_data.windspeed_10m[i])),
                1,
                Alignment::Right,
            ),
        ]));

        i += 1;
    }

    println!("{}", table.render());
}

fn print_weather_short(location: String, weather_data: WeatherData, max: usize) {
    let mut table = Table::new();
    table.max_column_width = 40;

    table.style = TableStyle::extended();

    let title_style = ansi_term::Style::new().bold().fg(ansi_term::Color::Yellow);

    table.add_row(Row::new(vec![TableCell::new_with_alignment(
        title_style.paint(location),
        2,
        Alignment::Center,
    )]));

    table.add_row(Row::new(vec![
        TableCell::new_with_alignment(title_style.paint("Time"), 1, Alignment::Center),
        TableCell::new_with_alignment(title_style.paint("Forecast"), 1, Alignment::Center),
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
            TableCell::new_with_alignment(&weather_data.weathercode[i], 1, Alignment::Left),
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
