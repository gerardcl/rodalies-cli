use chrono::{Datelike, Local};
use clap::{
    crate_authors, crate_description, crate_name, crate_version, value_parser, Arg, ArgAction,
    ArgMatches, Command
};
use prettytable::{format, Table};
use std::error::Error;

/// Configures the CLI behaviour, reads the arguments and returns and returns a container of matches.
pub fn init_cli() -> ArgMatches {
    let cli = Command::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::new("interactive")
                .required(false)
                .short('i')
                .long("interactive")
                .action(ArgAction::SetTrue)
                .value_parser(value_parser!(bool))
                .default_missing_value("true")
                .help("Enable interactive train timetable search. If no value then it defaults to 'true'.")
        )
        .arg(
            Arg::new("search")
                .required(false)
                .short('s')
                .long("search")
                .env("RODALIES_CLI_SEARCH")
                .action(ArgAction::Set)
                .help("Search the ID of a given station's name pattern, to later use it on your origin or destination.")
        )
        .arg(
            Arg::new("from")
                .required(false)
                .short('f')
                .long("from")
                .env("RODALIES_CLI_FROM")
                .action(ArgAction::Set)
                .help("The origin's station ID.")
        )
        .arg(
            Arg::new("to")
                .required(false)
                .short('t')
                .long("to")
                .env("RODALIES_CLI_TO")
                .action(ArgAction::Set)
                .help("The destinations's station ID.")
        )
        .arg(
            Arg::new("day")
                .required(false)
                .short('d')
                .long("day")
                .action(ArgAction::Set)
                .help("The day value of the date to search for (default = today's day).")
        )
        .arg(
            Arg::new("month")
                .required(false)
                .short('m')
                .long("month")
                .action(ArgAction::Set)
                .help("The month value of the date to search for (default = today's month).")
        )
        .arg(
            Arg::new("year")
                .required(false)
                .short('y')
                .long("year")
                .action(ArgAction::Set)
                .help("The year value of the date to search for (default = today's year).")
        );

    cli.get_matches()
}

/// Configures and returns the Table to print results from.
pub fn init_results_table() -> Table {
    let mut results_table = Table::new();
    results_table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    results_table
}

/// Given a container of CLI args, it processes the `interactive`, `from`, `to` and `search` arguments.
pub fn interactive_mode(args: &ArgMatches) -> Result<bool, Box<dyn Error>> {
    let interactive = args.get_one::<bool>("interactive").unwrap_or(&true);
    let from = args.contains_id("from");
    let to = args.contains_id("to");
    let search = args.contains_id("search");

    let is_interactive = *interactive && !from && !to && !search;
    println!("✨ Interactive mode enabled: '{}'", is_interactive);
    Ok(is_interactive)
}

/// Given a container of CLI args, it processes the `search` argument.
pub fn parse_search(args: &ArgMatches) -> Result<String, Box<dyn Error>> {
    let search = args.get_one::<String>("search").unwrap();
    println!("🔍 Searching stations that contain the text: '{}'", search);
    Ok(search.to_string())
}

/// Given a container of CLI args, it processes the `from` and `to` arguments.
pub fn parse_trip(args: &ArgMatches) -> Result<(String, String), Box<dyn Error>> {
    let from = args.get_one::<String>("from");
    let to = args.get_one::<String>("to");

    if from.is_none() || to.is_none() {
        return Err("🚨 Please, specify origin and destination station IDs".into());
    }

    Ok((from.unwrap().to_string(), to.unwrap().to_string()))
}

/// Given a container of CLI args, it processes the `day`, `month` and `year` arguments.
pub fn parse_date(args: &ArgMatches) -> Result<String, Box<dyn Error>> {
    let dt = Local::now();
    let day = match args.get_one::<String>("day") {
        Some(day) => match day.parse::<u32>() {
            Ok(day) => day,
            _ => return Err("🚨 Please, specify right value for day".into()),
        },
        None => dt.day(),
    };
    let month = match args.get_one::<String>("month") {
        Some(month) => match month.parse::<u32>() {
            Ok(month) => month,
            _ => return Err("🚨 Please, specify right value for month".into()),
        },
        None => dt.month(),
    };
    let year = match args.get_one::<String>("year") {
        Some(year) => match year.parse::<i32>() {
            Ok(year) => year,
            _ => return Err("🚨 Please, specify right value for year".into()),
        },
        None => dt.year(),
    };

    println!(
        "🔍 Searching timetable for date {:02}/{:02}/{}",
        day, month, year
    );

    Ok(format!("{:02}/{:02}/{}", day, month, year))
}

#[cfg(test)]
mod tests {
    use super::{init_cli, init_results_table};

    #[test]
    fn test_init_results_table_is_empty() {
        let results_table = init_results_table();
        assert!(results_table.is_empty());
    }

    #[test]
    fn test_init_cli_with_defaults() {
        let args = init_cli();
        assert!(args.args_present());
    }
}
