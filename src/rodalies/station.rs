use clap::ArgMatches;
use prettytable::{Cell, Row};
use scraper::Selector;
use std::error::Error;
use surf::Client;

use crate::{config::cli::init_results_table, rodalies::client::get_search_page};

struct Station {
    id: String,
    name: String,
}

pub async fn search_station(client: Client, args: ArgMatches) -> Result<(), Box<dyn Error>> {
    let search = args.get_one::<String>("search").unwrap();
    let mut results_table = init_results_table();

    let parsed_html = get_search_page(client).await?;

    let selector = &Selector::parse(r#"#origen > option"#).unwrap();

    let station_id: Vec<&str> = parsed_html
        .select(selector)
        .flat_map(|el| el.text())
        .collect();

    let station_name: Vec<&str> = parsed_html
        .select(selector)
        .flat_map(|el| el.value().attr("value"))
        .collect();

    let stations_list: Vec<Station> = station_name
        .iter()
        .zip(station_id.iter())
        .map(|s| Station {
            id: s.0.to_string(),
            name: s.1.to_string(),
        })
        .filter(|s| !s.id.is_empty())
        .collect();

    // search IDs
    println!(
        "🔍 Listing the stations' IDs of the stations' names containing: '{}'",
        search
    );
    results_table.set_titles(Row::new(vec![
        Cell::new("Station name"),
        Cell::new("Station ID"),
    ]));

    for station in stations_list.iter() {
        if station.name.to_lowercase().contains(&search.to_lowercase()) {
            results_table.add_row(Row::new(vec![
                Cell::new(&station.name),
                Cell::new(&station.id).style_spec("c"),
            ]));
        }
    }
    if !results_table.is_empty() {
        results_table.printstd();
        Ok(())
    } else {
        return Err(format!("🚨 No stations found with '{}' in it, please try searching something else and if problem persists open an issue...", search).into());
    }
}
