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
    /// Only show time and forecast, no details
    #[arg(short, default_value_t = false)]
    short: bool,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let args = Args::parse();

    current_weather(&args.location, args.max_data, args.short).await;

    Ok(())
}
