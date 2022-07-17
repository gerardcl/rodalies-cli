use prettytable::{Cell, Row};
use scraper::{Html, Selector};
use std::error::Error;
use surf::{Client, StatusCode};

use crate::config::cli::{init_results_table, Args};

struct Station {
    id: String,
    name: String,
}

pub async fn search_station(client: Client, args: Args) -> Result<(), Box<dyn Error>> {
    let mut results_table = init_results_table();

    let mut response = client.get("/en/horaris").await?;

    let error = match response.status() {
        StatusCode::Ok => false,
        _ => {
            println!(
                "⛔ Rodalies server failed with HTTP Status: {}",
                response.status()
            );
            true
        }
    };

    if error {
        return Err(
            ("🚨 Please, try again later or open an issue if the error persists...").into(),
        );
    }

    let body_response = &response.body_string().await?;

    let parsed_html = Html::parse_document(body_response);

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
        args.search
    );
    results_table.set_titles(Row::new(vec![
        Cell::new("Station name"),
        Cell::new("Station ID"),
    ]));

    for station in stations_list.iter() {
        if station
            .name
            .to_lowercase()
            .contains(&args.search.to_lowercase())
        {
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
        return Err(format!("🚨 No stations found with '{}' in it, please try searching something else and if problem persists open an issue...", args.search).into());
    }
}
