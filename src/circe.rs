use ansi_term::Style;
use term_table::{
    row::Row,
    table_cell::{Alignment, TableCell},
    Table, TableStyle,
};

use crate::{get_current_weather, get_locations, weather::WeatherData};

pub async fn current_weather(
    location: &str,
    max: usize,
    temperature: bool,
    preciptiations: bool,
    humidity: bool,
    wind: bool
) {
    let locs = match get_locations(location).await {
        Ok(val) => val,
        Err(_) => {
            println!("Request error.");
            return;
        }
    };
    let loc = match locs.first() {
        Some(val) => val,
        None => {
            println!("No locations were found with the given name.");
            return;
        }
    };

    let weather = match get_current_weather(loc.lat as f32, loc.lon as f32).await {
        Ok(val) => val,
        Err(e) => {
            println!("Could not find weather data. ({})", e);
            return;
        }
    };

    print_weather(
        loc.display_name.clone(),
        weather.hourly,
        max,
        temperature,
        preciptiations,
        humidity,
        wind
    )
}

fn print_weather(
    location: String, weather_data: WeatherData, max: usize,
    temperature: bool, preciptiations: bool, humidity: bool, wind: bool
) {
    let mut table = Table::new();
    table.max_column_width = 15;

    table.style = TableStyle::extended();

    let title_style = ansi_term::Style::new().bold().fg(ansi_term::Color::Yellow);
    let normal_style = ansi_term::Style::new();
    let cyan_style = ansi_term::Style::new().fg(ansi_term::Color::Cyan);

    let mut title = vec![("", 2)];
    let mut sub_title = vec![vec![
        "Time", "Forecast",
    ]];

    if temperature {
        title.push(("Temperature", 2));
        sub_title.push(vec!["At 2m", "Perceived"]);
    }
    if preciptiations {
        title.push(("Precipitations", 2));
        sub_title.push(vec!["Probability", "Quantity"])
    }
    if humidity {
        title.push(("Humidity", 1));
        sub_title.push(vec!["Relative"])
    }
    if wind {
        title.push(("Wind", 1));
        sub_title.push(vec!["At 10m"])
    }

    table.add_row(Row::new(vec![TableCell::new_with_alignment(
        title_style.paint(location),
        title.iter().map(|t| t.1).sum(),
        Alignment::Center,
    )]));

    table.add_row(Row::new(title.iter().map(|t|
        TableCell::new_with_alignment(t.0, t.1, Alignment::Center),
    )));


    let sub_title = sub_title.concat();
    table.add_row(Row::new(sub_title.into_iter().map(|t|
        TableCell::new_with_alignment(title_style.paint(t), 1, Alignment::Center),
    )));

    let mut i = 0;
    let mut j = 0;
    while j < max && i < weather_data.datetime.len() {
        if weather_data.datetime[i + 1] >= chrono::offset::Local::now() {
            j += 1;
        } else {
            i += 1;
            continue;
        }

        let mut cells = vec![
            TableCell::new_with_alignment(
                weather_data.datetime[i].naive_local().format("%d/%m %H:%M"),
                1,
                Alignment::Left,
            ),
            TableCell::new_with_alignment(
                &weather_data.weathercode[i],
                1,
                Alignment::Left
            ),
        ];
        if temperature {
            cells.push(
                TableCell::new_with_alignment(
                    normal_style.paint(format!("{:.1}°C", weather_data.temperature_2m[i])),
                    1,
                    Alignment::Right,
                ),
            );
            cells.push(
                TableCell::new_with_alignment(
                    normal_style.paint(format!("{:.1}°C", weather_data.apparent_temperature[i])),
                    1,
                    Alignment::Right,
                ),
            );
        }
        if preciptiations {
            cells.push(
                TableCell::new_with_alignment(
                    style_if_greater(
                        weather_data.precipitation_probability[i],
                        0.0,
                        cyan_style,
                        normal_style,
                    )
                    .paint(format!("{:.1}%", weather_data.precipitation_probability[i])),
                    1,
                    Alignment::Right,
                ),
            );
            cells.push(
                TableCell::new_with_alignment(
                    style_if_greater(weather_data.precipitation[i], 0.0, cyan_style, normal_style)
                        .paint(format!("{:.1}mm", weather_data.precipitation[i])),
                    1,
                    Alignment::Right,
                ),
            )
        }
        if humidity {
            cells.push(
                TableCell::new_with_alignment(
                    normal_style.paint(format!("{:.1}%", weather_data.relativehumidity_2m[i])),
                    1,
                    Alignment::Right,
                ),
            );
        }
        if wind {
            cells.push(
                TableCell::new_with_alignment(
                    normal_style.paint(format!("{:.1}km/h", weather_data.windspeed_10m[i])),
                    1,
                    Alignment::Right,
                ),
            );
        }
        
        table.add_row(Row::new(cells));
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
