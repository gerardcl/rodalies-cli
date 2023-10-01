use clap::ArgMatches;
use prettytable::{Cell, Row};
use scraper::{ElementRef, Html, Selector};
use std::error::Error;
use surf::Client;

use crate::{
    config::cli::{init_results_table, parse_date, parse_trip},
    rodalies::client::get_timetable_page,
};

/// Displays a table with the found train timetable.
pub async fn search_timetable(client: &Client, args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let (from, to) = parse_trip(args)?;
    let date = parse_date(args)?;
    search_timetable_input(client, from, to, date).await
}

// Convenience function to avoid unwrap()ing all the time
fn make_selector(selector: &str) -> Selector {
    Selector::parse(selector).unwrap()
}

trait VecParser {
    fn texts_parser(&self, selector: Selector) -> Vec<String>;
    fn alts_parser(&self, selector: Selector) -> Vec<String>;
}
impl VecParser for Html {
    fn texts_parser(&self, selector: Selector) -> Vec<String> {
        self.select(&selector)
            .flat_map(|el| el.text())
            .map(|t| t.to_string())
            .map(|x| x.trim().to_string())
            .filter(|x| !x.is_empty())
            .collect()
    }
    fn alts_parser(&self, selector: Selector) -> Vec<String> {
        self.select(&selector)
            .flat_map(|el| el.value().attr("alt"))
            .map(|t| t.to_string())
            .map(|x| x.trim().to_string())
            .filter(|x| !x.is_empty())
            .collect()
    }
}
impl VecParser for ElementRef<'_> {
    fn texts_parser(&self, selector: Selector) -> Vec<String> {
        self.select(&selector)
            .flat_map(|el| el.text())
            .map(|t| t.to_string())
            .map(|x| x.trim().to_string())
            .filter(|x| !x.is_empty())
            .collect()
    }
    fn alts_parser(&self, selector: Selector) -> Vec<String> {
        self.select(&selector)
            .flat_map(|el| el.value().attr("alt"))
            .map(|t| t.to_string())
            .map(|x| x.trim().to_string())
            .filter(|x| !x.is_empty())
            .collect()
    }
}

pub async fn search_timetable_input(
    client: &Client,
    from: String,
    to: String,
    date: String,
) -> Result<(), Box<dyn Error>> {
    let mut results_table = init_results_table();

    let parsed_html = get_timetable_page(client, from, to, date).await?;

    // check, show and fail if displayed errors
    let selector_errors = &Selector::parse(r#".error_contingut > p"#).unwrap();
    let errors: Vec<&str> = parsed_html
        .select(selector_errors)
        .flat_map(|el| el.text())
        .collect();
    if !errors.is_empty() {
        println!("⛔ Errors found and reported from Rodalies site:");
        for (pos, e) in errors.iter().enumerate() {
            println!("💩 {}: {:?}", pos + 1, e);
        }
        return Err("🚨 Please, make sure you provided right flags and values".into());
    }

    let resum_selector = make_selector(r#"div.resum > div.col-sm-12 > div.taula.d60 > div.cel"#);
    let total_stations = parsed_html.select(&resum_selector);
    // println!("#estacions: {:?}", &total_stations.count());
    let min_temp_selector = make_selector(
        r#"div.resum > div.col-sm-12 > div.taula.d40 > div.cel > div.info > span.t-min"#,
    );
    let min_temp = parsed_html.texts_parser(min_temp_selector);

    let max_temp_selector = make_selector(
        r#"div.resum > div.col-sm-12 > div.taula.d40 > div.cel > div.info > span.t-max"#,
    );
    let max_temp = parsed_html.texts_parser(max_temp_selector);

    // Create timetable's first row
    let mut title_cells: Vec<Cell> = vec![
        Cell::new("Duration"),
        Cell::new("Train"),
        Cell::new("Station"),
        Cell::new("Start"),
    ];
    let total = total_stations.count();

    println!("📆 Listing timetable with {} transfers", total - 2);

    for _ in 2..total {
        title_cells.push(Cell::new("Stop"));
        title_cells.push(Cell::new("Transfer"));
        title_cells.push(Cell::new("Wait"));
        title_cells.push(Cell::new("Train"));
        title_cells.push(Cell::new("Start"));
    }
    title_cells.push(Cell::new("End"));
    title_cells.push(Cell::new("Station"));
    results_table.set_titles(Row::new(title_cells.clone()));

    let mut different_lengths = false;

    let rows_selector = make_selector(r#"#acordio_resultats > div.panel.panel-default"#);
    let rows = parsed_html.select(&rows_selector);
    for row in rows {
        let durada_selector: Selector = make_selector(r#"div.resultats-fila > div.durada"#);
        let durada = row.texts_parser(durada_selector);

        let hora_sortides_selector: Selector =
            make_selector(r#"li.sortida > div.horari > div.hora"#);
        let hora_sortides = row.texts_parser(hora_sortides_selector);

        let hora_transbords_selector: Selector =
            make_selector(r#"li.transbord > div.horari > div.hora"#);
        let hora_transbords = row.texts_parser(hora_transbords_selector);

        let hora_arribades_selector: Selector =
            make_selector(r#"li.arribada > div.mask > div.horari > div.hora"#);
        let hora_arribades = row.texts_parser(hora_arribades_selector);

        let durada_transbords_selector: Selector =
            make_selector(r#"li.transbord > div.horari > div.temps > span"#);
        let durada_transbords = row.texts_parser(durada_transbords_selector);

        let tren_sortides_selector: Selector = make_selector(r#"div.timeline-badge > img"#);
        let tren_sortides = row.alts_parser(tren_sortides_selector);

        let estacions_selector: Selector = make_selector(r#"div.estacio > h3.timeline-title"#);
        let estacions = row.texts_parser(estacions_selector);

        let mut row_cells: Vec<Cell> = vec![
            Cell::new(durada[0].as_str()),
            Cell::new(tren_sortides[0].as_str()),
            Cell::new(estacions[0].as_str()),
            Cell::new(hora_sortides[0].as_str()),
        ];
        for tx in 0..hora_transbords.len() {
            row_cells.push(Cell::new(hora_transbords[tx].as_str()));
            row_cells.push(Cell::new(estacions[tx + 1].as_str()));
            row_cells.push(Cell::new(durada_transbords[tx].as_str()));
            row_cells.push(Cell::new(tren_sortides[tx + 1].as_str()));
            row_cells.push(Cell::new(hora_sortides[tx + 1].as_str()));
        }
        if hora_transbords.len() < total - 2 {
            different_lengths = true;
            for _ in 0..(total - 2 - hora_transbords.len()) {
                row_cells.push(Cell::new(""));
                row_cells.push(Cell::new(""));
                row_cells.push(Cell::new(""));
                row_cells.push(Cell::new(""));
                row_cells.push(Cell::new(""));
            }
        }
        row_cells.push(Cell::new(hora_arribades[0].as_str()));
        row_cells.push(Cell::new(estacions[estacions.len() - 1].as_str()));
        results_table.add_row(Row::new(row_cells.clone()));
    }

    if different_lengths {
        println!("📢 Some trips have extra transfers. You might consider getting shorter trips.");
    }

    println!(
        "🌡 Expected temperatures at destination between {}C and {}C",
        min_temp[0], max_temp[0]
    );
    results_table.printstd();

    Ok(())
}
