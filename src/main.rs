use chrono::{Datelike, Local};
use clap::Parser;
use itertools::izip;
use prettytable::{format, Cell, Row, Table};
use scraper::{Html, Selector};
use std::error::Error;
use std::time::Duration;
use surf::{Client, Config, StatusCode, Url};

/// Rodalies CLI - timetables of Rodalies de la Generalitat de Catalunya in your terminal
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Search the ID of a given station's name pattern, to later use it on your origin or destination
    #[clap(short, long, default_value = "")]
    search: String,
    /// The origin's station ID
    #[clap(short, long, default_value = "")]
    from: String,
    /// The destinations's station ID
    #[clap(short, long, default_value = "")]
    to: String,
    /// The day value of the date to search for
    #[clap(short, long, default_value_t = Local::today().day())]
    day: u32,
    /// The month value of the date to search for
    #[clap(short, long, default_value_t = Local::today().month())]
    month: u32,
    /// The year value of the date to search for
    #[clap(short, long, default_value_t = Local::today().year())]
    year: i32,
}

#[derive(Debug)]
struct Station {
    id: String,
    name: String,
}

#[derive(Debug)]
struct Timetable {
    total: String,
    origin: String,
    destination: String,
    start_time: String,
    end_time: String,
    start_train_type: String,
    transfer_station: String,
    transfer_time: String,
    transfer_start: String,
    transfer_end: String,
    transfer_train_type: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let dt = Local::today();
    let mut results_table = Table::new();
    results_table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    println!("🚂 Rodalies CLI configuration: {:?}", args);
    println!(
        "📅 Today's date is {:02}/{:02}/{}",
        dt.day(),
        dt.month(),
        dt.year()
    );

    let rodalies_url = "https://rodalies.gencat.cat";

    let client: Client = Config::new()
        .set_base_url(Url::parse(rodalies_url)?)
        .set_timeout(Some(Duration::from_secs(5)))
        .try_into()?;

    // search
    if !args.search.is_empty() {
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

        let parsed_html = Html::parse_document(&body_response);

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
        if results_table.len() > 0 {
            results_table.printstd();
        } else {
            return Err(format!("🚨 No stations found with '{}' in it, please try searching something else and if problem persists open an issue...", args.search).into());
        }

        return Ok(());
    }

    // search timetables
    if args.from.is_empty() || args.to.is_empty() {
        return Err(format!(
            "🚨 Please, specify origin and destination station IDs (type '{} --help' for more)",
            std::env::args().nth(0).unwrap()
        )
        .into());
    }
    println!(
        "📆 Searching timetable for date {:02}/{:02}/{}",
        args.day, args.month, args.year
    );

    let mut response = client
        .post("/en/horaris")
        .content_type("application/x-www-form-urlencoded")
        .body_string(format!(
            "origen={}&desti={}&dataViatge={}&horaIni=00&lang=en&cercaRodalies=true&tornada=false",
            args.from,
            args.to,
            format!("{:02}/{:02}/{}", args.day, args.month, args.year)
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

    let parsed_html = Html::parse_document(&body_response);

    let selector_total_times = &Selector::parse(r#"#acordio_resultats > div.panel.panel-default > div.panel-heading > a > div.resultats-fila > div.durada"#).unwrap();
    let total_times: Vec<&str> = parsed_html
        .select(selector_total_times)
        .flat_map(|el| el.text())
        .collect();

    // departures
    let selector_departure_time = &Selector::parse(r#"#acordio_resultats > div.panel.panel-default > div.panel-collapse.collapse > div.intinerari > div.timeline-wrap > ul.timeline > li.sortida > div.horari > div.hora"#).unwrap();
    let sortides_hora: Vec<&str> = parsed_html
        .select(selector_departure_time)
        .flat_map(|el| el.text())
        .collect();
    let selector_departure_train = &Selector::parse(r#"#acordio_resultats > div.panel.panel-default > div.panel-collapse.collapse > div.intinerari > div.timeline-wrap > ul.timeline > li.sortida > div.timeline-badge > img"#).unwrap();
    let departures_train: Vec<&str> = parsed_html
        .select(selector_departure_train)
        .flat_map(|el| el.value().attr("alt"))
        .collect();
    let selector_departure_station = &Selector::parse(r#"#acordio_resultats > div.panel.panel-default > div.panel-collapse.collapse > div.intinerari > div.timeline-wrap > ul.timeline > li.sortida > div.estacio > h3"#).unwrap();
    let departures_station: Vec<&str> = parsed_html
        .select(selector_departure_station)
        .flat_map(|el| el.text())
        .collect();
    // transfers
    let selector_transfer_start = &Selector::parse(r#"#acordio_resultats > div.panel.panel-default > div.panel-collapse.collapse > div.intinerari > div.timeline-wrap > ul.timeline > li.transbord > div.horari > div.hora"#).unwrap();
    let transfer_start: Vec<&str> = parsed_html
        .select(selector_transfer_start)
        .flat_map(|el| el.text())
        .collect();
    let selector_transfer_duration = &Selector::parse(r#"#acordio_resultats > div.panel.panel-default > div.panel-collapse.collapse > div.intinerari > div.timeline-wrap > ul.timeline > li.transbord > div.horari > div.temps > span"#).unwrap();
    let transfer_duration: Vec<&str> = parsed_html
        .select(selector_transfer_duration)
        .flat_map(|el| el.text())
        .collect();
    // arrivals
    let selector_arrival_time = &Selector::parse(r#"#acordio_resultats > div.panel.panel-default > div.panel-collapse.collapse > div.intinerari > div.timeline-wrap > ul.timeline > li.arribada > div.mask > div.horari > div.hora"#).unwrap();
    let arrival_time: Vec<&str> = parsed_html
        .select(selector_arrival_time)
        .flat_map(|el| el.text())
        .collect();
    let selector_arrival_station = &Selector::parse(r#"#acordio_resultats > div.panel.panel-default > div.panel-collapse.collapse > div.intinerari > div.timeline-wrap > ul.timeline > li.arribada > div.mask > div.estacio > h3"#).unwrap();
    let arrival_station: Vec<&str> = parsed_html
        .select(selector_arrival_station)
        .flat_map(|el| el.text())
        .collect();

    // check, show and fail if errors after query timetable
    let selector_errors = &Selector::parse(r#".error_contingut > p"#).unwrap();
    let errors: Vec<&str> = parsed_html
        .select(selector_errors)
        .flat_map(|el| el.text())
        .collect();
    if errors.len() > 0 {
        println!("⛔ Errors found and reported from Rodalies site:");
        for (pos, e) in errors.iter().enumerate() {
            println!("💩 {}: {:?}", pos + 1, e);
        }
        return Err(format!(
            "🚨 Please, make sure you provided right flags and values (type '{} --help' for more)",
            std::env::args().nth(0).unwrap()
        )
        .into());
    }

    // check if multiple transbords
    let transfers = &Selector::parse(r#"#acordio_resultats > div.panel.panel-default > div.panel-collapse.collapse > div.intinerari > div.timeline-wrap.connection"#).unwrap();
    let totals = &Selector::parse(r#"#acordio_resultats > div.panel.panel-default > div.panel-collapse.collapse > div.intinerari > div.timeline-wrap"#).unwrap();
    let transfers_count = parsed_html.select(transfers).count();
    let total_count = parsed_html.select(totals).count();

    if total_count > 0 {
        println!(
            "📖 Timetable with {} transfers found:",
            if transfers_count > 0 {
                total_count / transfers_count - 1
            } else {
                0
            }
        );

        if transfers_count > 0 {
            let mut departures_times: Vec<&str> = Vec::new();
            let mut transfers_ends: Vec<&str> = Vec::new();
            for (pos, sortida) in sortides_hora.iter().enumerate() {
                if pos % 2 == 0 {
                    departures_times.push(&sortida)
                } else {
                    transfers_ends.push(&sortida)
                }
            }

            let mut departures_trains: Vec<&str> = Vec::new();
            let mut transfers_trains: Vec<&str> = Vec::new();
            for (pos, tren) in departures_train.iter().enumerate() {
                if pos % 2 == 0 {
                    departures_trains.push(&tren)
                } else {
                    transfers_trains.push(&tren)
                }
            }

            let mut departures_stations: Vec<&str> = Vec::new();
            let mut transfers_stations: Vec<&str> = Vec::new();
            for (pos, estacio) in departures_station.iter().enumerate() {
                if pos % 2 == 0 {
                    departures_stations.push(&estacio)
                } else {
                    transfers_stations.push(&estacio)
                }
            }

            let timetables: Vec<Timetable> = izip!(
                departures_trains.iter(),
                departures_stations.iter(),
                arrival_station.iter(),
                departures_times.iter(),
                arrival_time.iter(),
                total_times.iter(),
                transfers_stations.iter(),
                transfers_trains.iter(),
                transfer_duration.iter(),
                transfer_start.iter(),
                transfers_ends.iter(),
            )
            .map(|s| Timetable {
                start_train_type: s.0.to_string(),
                origin: s.1.to_string(),
                destination: s.2.to_string().replace("\n", "").replace("\t", ""),
                start_time: s.3.to_string(),
                end_time: s.4.to_string().replace("\n", "").replace("\t", ""),
                total: s.5.to_string(),
                transfer_station: s.6.to_string(),
                transfer_train_type: s.7.to_string(),
                transfer_time: s.8.to_string(),
                transfer_start: s.9.to_string().replace("\n", "").replace("\t", ""),
                transfer_end: s.10.to_string(),
            })
            .collect();

            results_table.set_titles(Row::new(vec![
                Cell::new("Duration"),
                Cell::new("Train"),
                Cell::new("Station"),
                Cell::new("Start"),
                Cell::new("Stop"),
                Cell::new("Transfer"),
                Cell::new("Wait"),
                Cell::new("Train"),
                Cell::new("Start"),
                Cell::new("End"),
                Cell::new("Station"),
            ]));
            for tt in timetables.iter() {
                results_table.add_row(Row::new(vec![
                    Cell::new(&tt.total).style_spec("c"),
                    Cell::new(&tt.start_train_type).style_spec("c"),
                    Cell::new(&tt.origin).style_spec("c"),
                    Cell::new(&tt.start_time).style_spec("c"),
                    Cell::new(&tt.transfer_start).style_spec("c"),
                    Cell::new(&tt.transfer_station).style_spec("c"),
                    Cell::new(&tt.transfer_time).style_spec("c"),
                    Cell::new(&tt.transfer_train_type).style_spec("c"),
                    Cell::new(&tt.transfer_end).style_spec("c"),
                    Cell::new(&tt.end_time).style_spec("c"),
                    Cell::new(&tt.destination).style_spec("c"),
                ]));
            }
            results_table.printstd();
        // no transfers
        } else {
            let timetables: Vec<Timetable> = izip!(
                departures_train.iter(),
                departures_station.iter(),
                arrival_station.iter(),
                sortides_hora.iter(),
                arrival_time.iter(),
                total_times.iter()
            )
            .map(|s| Timetable {
                start_train_type: s.0.to_string(),
                origin: s.1.to_string(),
                destination: s.2.to_string(),
                start_time: s.3.to_string(),
                end_time: s.4.to_string(),
                total: s.5.to_string(),
                transfer_station: String::from("no"),
                transfer_train_type: String::from("no"),
                transfer_time: String::from("no"),
                transfer_start: String::from("no"),
                transfer_end: String::from("no"),
            })
            .collect();

            results_table.set_titles(Row::new(vec![
                Cell::new("Duration"),
                Cell::new("Train"),
                Cell::new("Station"),
                Cell::new("Start"),
                Cell::new("End"),
                Cell::new("Station"),
            ]));
            for tt in timetables.iter() {
                results_table.add_row(Row::new(vec![
                    Cell::new(&tt.total).style_spec("c"),
                    Cell::new(&tt.start_train_type).style_spec("c"),
                    Cell::new(&tt.origin).style_spec("c"),
                    Cell::new(&tt.start_time).style_spec("c"),
                    Cell::new(&tt.end_time).style_spec("c"),
                    Cell::new(&tt.destination).style_spec("c"),
                ]));
            }
            results_table.printstd();
        }
    // unexpected
    } else {
        return Err(("🚨 No timetables found, try again later and if problem perists open an issue").into());
    }

    Ok(())
}
