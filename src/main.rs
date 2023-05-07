use std::error::Error;

use chrono::{Datelike, Local};
use rodalies_cli::config::cli::{init_cli, interactive_mode};
use rodalies_cli::rodalies::client::init_client;
use rodalies_cli::rodalies::interactive::search_interactive;
use rodalies_cli::rodalies::{station::search_station, timetable::search_timetable};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = init_cli();
    let client = init_client();
    let dt = Local::now();

    println!(
        "🚂 Rodalies CLI 📅 Today's date is {:02}/{:02}/{}",
        dt.day(),
        dt.month(),
        dt.year()
    );

    if !interactive_mode(&args).unwrap() {
        if args.contains_id("search") {
            // search station
            search_station(&client, &args).await?
        } else {
            // search timetable
            search_timetable(&client, &args).await?
        }
    } else {
        search_interactive(&client, &args).await?
    }

    Ok(())
}
