use clap::ArgMatches;
use prettytable::{Cell, Row};
use scraper::Selector;
use std::error::Error;
use surf::Client;

use crate::{
    config::cli::{init_results_table, parse_search},
    rodalies::client::get_search_page,
};

/// The Station information struct
#[derive(Clone)]
pub struct Station {
    /// The internal ID of the station name, that is provided by the rodalies site. It is the value used when submitting a search.
    pub id: String,
    /// The rodalies station name. It is the display name provided by the rodalies site.
    pub name: String,
}

/// It parses the main search page and returns a list with all the existing Stations.
pub async fn get_stations_list(client: &Client) -> Result<Vec<Station>, Box<dyn Error>> {
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

    Ok(stations_list)
}

/// Displays a table with station IDs and station names, from the station names that contain the `search` text.
pub async fn search_station(client: &Client, args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let search = parse_search(args)?;
    let mut results_table = init_results_table();
    let stations_list = get_stations_list(client).await?;

    // search IDs
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
        return Err(format!("🚨 No stations found with text '{}' in it, please try searching something else, and if the problem persists open an issue...", search).into());
    }
}

pub async fn search_station_input(
    stations_list: &[Station],
    search: String,
) -> Result<Vec<Station>, Box<dyn Error>> {
    let mut results_table = init_results_table();
    let mut found_station_list: Vec<Station> = Vec::new();
    let mut index = 1;
    // search IDs
    results_table.set_titles(Row::new(vec![
        Cell::new("Option"),
        Cell::new("Station name"),
    ]));

    for station in stations_list.iter() {
        if station.name.to_lowercase().contains(&search.to_lowercase()) {
            results_table.add_row(Row::new(vec![
                Cell::new(index.to_string().as_str()).style_spec("r"),
                Cell::new(&station.name),
            ]));
            found_station_list.push(station.clone());
            index += 1;
        }
    }
    if !results_table.is_empty() {
        results_table.printstd();
        Ok(found_station_list)
    } else {
        return Err(format!("🚨 No stations found with text '{}' in it, please try searching something else, and if the problem persists open an issue...", search).into());
    }
}
