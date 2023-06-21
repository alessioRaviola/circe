use circe::current_weather;

use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    /// Name of the location to check the weather for
    #[clap(required = true)]
    location: String,
    /// Maximum number of data points to show the forecast for
    #[arg(short, default_value_t = 12)]
    max_data: usize,
    
    /// Show temperature data
    #[arg(short, default_value_t = false)]
    temperature: bool,

    /// Show precipitations data
    #[arg(short, default_value_t = false)]
    precipitations: bool,
    
    /// Show humidity data
    #[arg(short('u'), default_value_t = false)]
    humidity: bool,
    
    /// Show wind data
    #[arg(short, default_value_t = false)]
    wind: bool,

    /// Show full data, same as -t -p -u -w
    #[arg(long, default_value_t = false)]
    full: bool
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let args = Args::parse();

    current_weather(
        &args.location,
        args.max_data,
        args.temperature | args.full,
        args.precipitations | args.full,
        args.humidity | args.full,
        args.wind | args.full
    ).await;

    Ok(())
}
