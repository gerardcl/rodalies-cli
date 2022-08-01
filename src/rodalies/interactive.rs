use clap::ArgMatches;
use std::error::Error;
use std::io;
use surf::Client;

use crate::{
    config::cli::parse_date,
    rodalies::{
        station::{get_stations_list, search_station_input, Station},
        timetable::search_timetable_input,
    },
};

#[allow(unused_assignments)]
pub async fn search_interactive(client: &Client, args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let mut from = String::new();
    let mut to = String::new();
    let mut date = String::new();
    let mut input = String::new();
    let stations_list = get_stations_list(client).await?;

    println!("🎬 Which is your origin's station name?");
    let origin_station_list: Vec<Station> = match io::stdin().read_line(&mut input) {
        Ok(n) => {
            if n >= 4 {
                search_station_input(&stations_list, input.trim().to_string()).await?
            } else {
                return Err("Please, provide at least 3 characters of the station name.".into());
            }
        }
        Err(error) => return Err(format!("error: {}", error).into()),
    };

    input.clear();
    if origin_station_list.len() == 1 {
        println!(
            "🎬 Perfect match! Using '{}' as the selected origin's station",
            origin_station_list[0].name
        );
        from = origin_station_list[0].clone().id;
    } else {
        println!("🎬 Which option does match with your origin's station name? ");
        from = match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let trimmed = input.trim();
                let index = match trimmed.parse::<usize>() {
                    Ok(index) => {
                        if index <= origin_station_list.len() && index >= 1 {
                            index - 1
                        } else {
                            return Err("Please, provide a valid input number.".into());
                        }
                    }
                    Err(..) => return Err(format!("this was not an integer: {}", trimmed).into()),
                };
                println!("you have selected {}", origin_station_list[index].name);
                origin_station_list[index].clone().id
            }
            Err(error) => return Err(format!("error: {}", error).into()),
        };
    }

    input.clear();
    println!("🎬 Which is your destination's station name?");
    let destination_station_list: Vec<Station> = match io::stdin().read_line(&mut input) {
        Ok(n) => {
            if n >= 4 {
                search_station_input(&stations_list, input.trim().to_string()).await?
            } else {
                return Err("Please, provide at least 3 characters of the station name.".into());
            }
        }
        Err(error) => return Err(format!("error: {}", error).into()),
    };

    input.clear();
    if destination_station_list.len() == 1 {
        println!(
            "🎬 Perfect match! Using '{}' as the selected destination's station",
            destination_station_list[0].name
        );
        to = destination_station_list[0].clone().id;
    } else {
        println!("🎬 Which option does match with your destination's station name? ");
        input.clear();
        to = match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let trimmed = input.trim();
                let index = match trimmed.parse::<usize>() {
                    Ok(index) => {
                        if index <= destination_station_list.len() && index >= 1 {
                            index - 1
                        } else {
                            return Err("Please, provide a valid input number.".into());
                        }
                    }
                    Err(..) => return Err(format!("this was not an integer: {}", trimmed).into()),
                };
                println!("you have selected {}", destination_station_list[index].name);
                destination_station_list[index].clone().id
            }
            Err(error) => return Err(format!("error: {}", error).into()),
        };
    }

    // TODO maybe make this interactive too - options: 1) today, 2) tomorrow, 3) input day, month, year (provide default first)
    date = parse_date(args)?;

    search_timetable_input(client, from, to, date).await?;

    Ok(())
}
