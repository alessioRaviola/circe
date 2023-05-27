use circe::Circe;

use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short)]
    location: String,
    #[arg(short, default_value_t = 12)]
    max_data: usize,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let args = Args::parse();

    Circe::current_weather(&args.location, args.max_data).await;

    Ok(())
}
