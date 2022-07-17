use std::error::Error;

use rodalies_cli::config::cli::{init_cli, Args};
use rodalies_cli::rodalies::client::init_client;
use rodalies_cli::rodalies::{station::search_station, timetable::search_timetable};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Args = init_cli();
    let client = init_client();

    if !args.search.is_empty() {
        // search station
        search_station(client, args).await?
    } else {
        // search timetable
        search_timetable(client, args).await?
    }

    Ok(())
}
