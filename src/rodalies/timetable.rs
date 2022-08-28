use clap::ArgMatches;
use prettytable::{Cell, Row};
use scraper::Selector;
use std::{collections::VecDeque, error::Error};
use surf::Client;

use crate::{
    config::cli::{init_results_table, parse_date, parse_trip},
    rodalies::client::get_timetable_page,
};

/// The Timetable data collected information struct
struct TimetableData {
    /// A list of the train types (also transfer departures, if any) of each trip
    departures_train_type: Vec<String>,
    /// A list of the station name of the departures (also transfer departures, if any) of each trip
    departures_station: Vec<String>,
    /// A list of the start time of the departures (also transfer departures, if any) of each trip
    departures_time: Vec<String>,
    /// A list of the stop time (also the start time of a transfer) of each trip (if any)
    transfers_time: Vec<String>,
    /// A list of the transfer duration time of each trip (if any)
    transfers_duration: Vec<String>,
    /// A list of the arrival time of each trip
    arrivals_time: Vec<String>,
    /// A list of the arrival station of each trip
    arrivals_station: Vec<String>,
}

/// Displays a table with the found train timetable.
pub async fn search_timetable(client: &Client, args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let (from, to) = parse_trip(args)?;
    let date = parse_date(args)?;
    search_timetable_input(client, from, to, date).await
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

    // check how much station transfer and if we need to skip transfers that are useless
    let transfers = &Selector::parse(r#"#acordio_resultats > div.panel.panel-default > div.panel-collapse.collapse > div.intinerari > div.timeline-wrap.connection"#).unwrap();
    let totals = &Selector::parse(r#"#acordio_resultats > div.panel.panel-default > div.panel-collapse.collapse > div.intinerari > div.timeline-wrap"#).unwrap();
    let arrivals = &Selector::parse(r#"#acordio_resultats > div.panel.panel-default > div.panel-collapse.collapse > div.intinerari > div.timeline-wrap > ul.timeline > li.arribada"#).unwrap();
    let transfers_count = parsed_html.select(transfers).count();
    let total_count = parsed_html.select(totals).count();
    let arrivals_count = parsed_html.select(arrivals).count();
    let total_transfers_count = match transfers_count {
        count if count > 0 => transfers_count / (total_count - transfers_count),
        _ => 0,
    };
    let skip_transfers = arrivals_count == transfers_count && total_transfers_count == 0;

    println!(
        "📆 Listing timetable with {} transfers",
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
    let mut arrivals_time: VecDeque<&str> = parsed_html
        .select(selector_arrival_time)
        .flat_map(|el| el.text())
        .collect();
    let selector_arrival_station = &Selector::parse(r#"#acordio_resultats > div.panel.panel-default > div.panel-collapse.collapse > div.intinerari > div.timeline-wrap > ul.timeline > li.arribada > div.mask > div.estacio > h3"#).unwrap();
    let mut arrivals_station: VecDeque<&str> = parsed_html
        .select(selector_arrival_station)
        .flat_map(|el| el.text())
        .collect();

    // when different number of transfers in a timetable:
    // 1- expected arrival div is now called arrivals (the ones we want to keep)
    // 2- arrival div is the arrivals of the different longer transfers
    // 3- we need to collect both and inject an arrival in the arrivals list just after an arrival is equal to that arrivals value
    // 4- in the iterator we would just skip the arrival that contains transfer when arrivals == arrivals+1
    // we need to do all this complex handling because rodalies web decided that although you arrive at the same time you might want to catch a train earlier and wait for a transfer somewhere just to get the next train you would end up getting anyway...
    if skip_transfers {
        // arrivals diff (if different transfers then this is the ones we want to keep)
        let selector_arrival_diff_time = &Selector::parse(r#"#acordio_resultats > div.panel.panel-default > div.panel-collapse.collapse > div.intinerari > div.timeline-wrap > ul.timeline > li.arribadas > div.mask > div.horari > div.hora"#).unwrap();
        let arrivals_diff_time: VecDeque<&str> = parsed_html
            .select(selector_arrival_diff_time)
            .flat_map(|el| el.text())
            .collect();
        let selector_arrival_diff_station = &Selector::parse(r#"#acordio_resultats > div.panel.panel-default > div.panel-collapse.collapse > div.intinerari > div.timeline-wrap > ul.timeline > li.arribadas > div.mask > div.estacio > h3"#).unwrap();
        let arrivals_diff_station: VecDeque<&str> = parsed_html
            .select(selector_arrival_diff_station)
            .flat_map(|el| el.text())
            .collect();

        let clean_arrivals_time: Vec<String> = arrivals_time
            .iter()
            .map(|x| x.replace('\n', "").replace('\t', ""))
            .collect();

        if clean_arrivals_time.is_empty() {
            return Err(format!("Something went wrong, try again. Please, if problem presists do report an issue with the following information: arrivals_diff_time == {}, clean_arrivals_time == {}", arrivals_diff_time.len(),clean_arrivals_time.len()).into());
        }

        arrivals_time = arrivals_diff_time.clone();
        arrivals_station = arrivals_diff_station.clone();
        let mut found = 0;
        for (i, arrival) in arrivals_diff_time.iter().enumerate() {
            if clean_arrivals_time.contains(&arrival.to_string()) {
                arrivals_time.insert(i + found, arrival);
                arrivals_station.insert(i + found, arrivals_diff_station[i]);
                found += 1;
            }
        }
    }

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

    let mut skip = 0;
    for (i, duration) in durations.iter().enumerate() {
        if skip_transfers
            && i < (durations.len() - 1)
            && timetable_data.arrivals_time[i] == timetable_data.arrivals_time[i + 1]
        {
            skip += 1;
            continue;
        }
        let mut row_cells: Vec<Cell> = vec![
            Cell::new(duration),
            Cell::new(
                timetable_data.departures_train_type[(i + skip) * (total_transfers_count + 1)]
                    .as_str(),
            ),
            Cell::new(
                timetable_data.departures_station[(i + skip) * (total_transfers_count + 1)]
                    .as_str(),
            ),
            Cell::new(
                timetable_data.departures_time[(i + skip) * (total_transfers_count + 1)].as_str(),
            ),
        ];
        for j in 0..total_transfers_count {
            row_cells.push(Cell::new(
                timetable_data.transfers_time[(i + skip) * total_transfers_count + j].as_str(),
            ));
            row_cells.push(Cell::new(
                timetable_data.departures_station
                    [((i + skip) * (total_transfers_count + 1)) + j + 1]
                    .as_str(),
            ));
            row_cells.push(Cell::new(
                timetable_data.transfers_duration[(i + skip) * total_transfers_count + j].as_str(),
            ));
            row_cells.push(Cell::new(
                timetable_data.departures_train_type
                    [((i + skip) * (total_transfers_count + 1)) + j + 1]
                    .as_str(),
            ));
            row_cells.push(Cell::new(
                timetable_data.departures_time[((i + skip) * (total_transfers_count + 1)) + j + 1]
                    .as_str(),
            ));
        }
        row_cells.push(Cell::new(timetable_data.arrivals_time[i].as_str()));
        row_cells.push(Cell::new(timetable_data.arrivals_station[i].as_str()));
        results_table.add_row(Row::new(row_cells.clone()));
    }

    results_table.printstd();

    Ok(())
}
