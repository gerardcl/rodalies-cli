use std::error::Error;

use rodalies_cli::config::cli::init_cli;
use rodalies_cli::rodalies::client::init_client;
use rodalies_cli::rodalies::{station::search_station, timetable::search_timetable};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = init_cli();
    let client = init_client();

    if args.contains_id("search") {
        // search station
        search_station(client, args).await?
    } else {
        // search timetable
        search_timetable(client, args).await?
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::main;

    #[test]
    fn test_main_error_when_no_args() {
        let result = main();
        assert!(result.is_err());
    }
}
