use prettytable::{Cell, Row};
use scraper::{Html, Selector};
use std::error::Error;
use surf::{Client, StatusCode};

use crate::config::cli::{init_results_table, Args};

struct TimetableData {
    departures_train_type: Vec<String>,
    departures_station: Vec<String>,
    departures_time: Vec<String>,
    transfers_time: Vec<String>,
    transfers_duration: Vec<String>,
    arrivals_time: Vec<String>,
    arrivals_station: Vec<String>,
}

pub async fn search_timetable(client: Client, args: Args) -> Result<(), Box<dyn Error>> {
    let mut results_table = init_results_table();

    if args.from.is_empty() || args.to.is_empty() {
        return Err(format!(
            "🚨 Please, specify origin and destination station IDs (type '{} --help' for more)",
            std::env::args().next().unwrap()
        )
        .into());
    }
    println!(
        "📆 Searching timetable for date {:02}/{:02}/{}",
        args.day, args.month, args.year
    );

    let date = format!("{:02}/{:02}/{}", args.day, args.month, args.year);
    let mut response = client
        .post("/en/horaris")
        .content_type("application/x-www-form-urlencoded")
        .body_string(format!(
            "origen={}&desti={}&dataViatge={}&horaIni=00&lang=en&cercaRodalies=true&tornada=false",
            args.from, args.to, date
        ))
        .await?;

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
        return Err(format!(
            "🚨 Please, make sure you provided right flags and values (type '{} --help' for more)",
            std::env::args().next().unwrap()
        )
        .into());
    }

    // check how much station transfer
    let transfers = &Selector::parse(r#"#acordio_resultats > div.panel.panel-default > div.panel-collapse.collapse > div.intinerari > div.timeline-wrap.connection"#).unwrap();
    let totals = &Selector::parse(r#"#acordio_resultats > div.panel.panel-default > div.panel-collapse.collapse > div.intinerari > div.timeline-wrap"#).unwrap();
    let transfers_count = parsed_html.select(transfers).count();
    let total_count = parsed_html.select(totals).count();
    let total_transfers_count = match transfers_count {
        count if count > 0 => transfers_count / (total_count - transfers_count),
        _ => 0,
    };

    println!(
        "🔍 Listing timetable with {} transfers",
        total_transfers_count
    );

    // Create timetable's first row
    let mut title_cells: Vec<Cell> = vec![
        Cell::new("Duration"),
        Cell::new("Train"),
        Cell::new("Station"),
        Cell::new("Start"),
    ];
    for _ in 0..total_transfers_count {
        title_cells.push(Cell::new("Stop"));
        title_cells.push(Cell::new("Transfer"));
        title_cells.push(Cell::new("Wait"));
        title_cells.push(Cell::new("Train"));
        title_cells.push(Cell::new("Start"));
    }
    title_cells.push(Cell::new("End"));
    title_cells.push(Cell::new("Station"));
    results_table.set_titles(Row::new(title_cells.clone()));

    let selector_duration = &Selector::parse(r#"#acordio_resultats > div.panel.panel-default > div.panel-heading > a > div.resultats-fila > div.durada"#).unwrap();
    let durations: Vec<&str> = parsed_html
        .select(selector_duration)
        .flat_map(|el| el.text())
        .collect();

    // departures
    let selector_departure_train_type = &Selector::parse(r#"#acordio_resultats > div.panel.panel-default > div.panel-collapse.collapse > div.intinerari > div.timeline-wrap > ul.timeline > li.sortida > div.timeline-badge > img"#).unwrap();
    let departures_train_type: Vec<&str> = parsed_html
        .select(selector_departure_train_type)
        .flat_map(|el| el.value().attr("alt"))
        .collect();
    let selector_departure_station = &Selector::parse(r#"#acordio_resultats > div.panel.panel-default > div.panel-collapse.collapse > div.intinerari > div.timeline-wrap > ul.timeline > li.sortida > div.estacio > h3"#).unwrap();
    let departures_station: Vec<&str> = parsed_html
        .select(selector_departure_station)
        .flat_map(|el| el.text())
        .collect();
    let selector_departure_time = &Selector::parse(r#"#acordio_resultats > div.panel.panel-default > div.panel-collapse.collapse > div.intinerari > div.timeline-wrap > ul.timeline > li.sortida > div.horari > div.hora"#).unwrap();
    let departures_time: Vec<&str> = parsed_html
        .select(selector_departure_time)
        .flat_map(|el| el.text())
        .collect();

    if departures_time.is_empty()
        || departures_station.is_empty()
        || departures_train_type.is_empty()
    {
        return Err(format!("Something went wrong, try again. Please, if problem presists do report an issue with the following information: departures_time == {}, departures_station == {}, departures_train_type == {}", departures_time.len(),departures_station.len(),departures_train_type.len() ).into());
    }

    // transfers
    let selector_transfer_time = &Selector::parse(r#"#acordio_resultats > div.panel.panel-default > div.panel-collapse.collapse > div.intinerari > div.timeline-wrap > ul.timeline > li.transbord > div.horari > div.hora"#).unwrap();
    let transfers_time: Vec<&str> = parsed_html
        .select(selector_transfer_time)
        .flat_map(|el| el.text())
        .collect();
    let selector_transfer_duration = &Selector::parse(r#"#acordio_resultats > div.panel.panel-default > div.panel-collapse.collapse > div.intinerari > div.timeline-wrap > ul.timeline > li.transbord > div.horari > div.temps > span"#).unwrap();
    let transfers_duration: Vec<&str> = parsed_html
        .select(selector_transfer_duration)
        .flat_map(|el| el.text())
        .collect();

    // arrivals
    let selector_arrival_time = &Selector::parse(r#"#acordio_resultats > div.panel.panel-default > div.panel-collapse.collapse > div.intinerari > div.timeline-wrap > ul.timeline > li.arribada > div.mask > div.horari > div.hora"#).unwrap();
    let arrivals_time: Vec<&str> = parsed_html
        .select(selector_arrival_time)
        .flat_map(|el| el.text())
        .collect();
    let selector_arrival_station = &Selector::parse(r#"#acordio_resultats > div.panel.panel-default > div.panel-collapse.collapse > div.intinerari > div.timeline-wrap > ul.timeline > li.arribada > div.mask > div.estacio > h3"#).unwrap();
    let arrivals_station: Vec<&str> = parsed_html
        .select(selector_arrival_station)
        .flat_map(|el| el.text())
        .collect();

    let timetable_data: TimetableData = TimetableData {
        departures_train_type: departures_train_type
            .iter()
            .map(|x| x.replace('\n', "").replace('\t', ""))
            .collect(),
        departures_station: departures_station
            .iter()
            .map(|x| x.replace('\n', "").replace('\t', ""))
            .collect(),
        departures_time: departures_time
            .iter()
            .map(|x| x.replace('\n', "").replace('\t', ""))
            .collect(),
        transfers_time: transfers_time
            .iter()
            .map(|x| x.replace('\n', "").replace('\t', ""))
            .collect(),
        transfers_duration: transfers_duration
            .iter()
            .map(|x| x.replace('\n', "").replace('\t', ""))
            .collect(),
        arrivals_time: arrivals_time
            .iter()
            .map(|x| x.replace('\n', "").replace('\t', ""))
            .collect(),
        arrivals_station: arrivals_station
            .iter()
            .map(|x| x.replace('\n', "").replace('\t', ""))
            .collect(),
    };

    for (i, duration) in durations.iter().enumerate() {
        let mut row_cells: Vec<Cell> = vec![
            Cell::new(duration),
            Cell::new(
                timetable_data.departures_train_type[i * (total_transfers_count + 1)].as_str(),
            ),
            Cell::new(timetable_data.departures_station[i * (total_transfers_count + 1)].as_str()),
            Cell::new(timetable_data.departures_time[i * (total_transfers_count + 1)].as_str()),
        ];
        for j in 0..total_transfers_count {
            row_cells.push(Cell::new(
                timetable_data.transfers_time[i * total_transfers_count + j].as_str(),
            ));
            row_cells.push(Cell::new(
                timetable_data.departures_station[(i * (total_transfers_count + 1)) + j + 1]
                    .as_str(),
            ));
            row_cells.push(Cell::new(
                timetable_data.transfers_duration[i * total_transfers_count + j].as_str(),
            ));
            row_cells.push(Cell::new(
                timetable_data.departures_train_type[(i * (total_transfers_count + 1)) + j + 1]
                    .as_str(),
            ));
            row_cells.push(Cell::new(
                timetable_data.departures_time[(i * (total_transfers_count + 1)) + j + 1].as_str(),
            ));
        }
        row_cells.push(Cell::new(timetable_data.arrivals_time[i].as_str()));
        row_cells.push(Cell::new(timetable_data.arrivals_station[i].as_str()));
        results_table.add_row(Row::new(row_cells.clone()));
    }

    results_table.printstd();

    Ok(())
}
