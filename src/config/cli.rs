use chrono::{Datelike, Local};
use clap::Parser;
use prettytable::{format, Table};

/// Rodalies CLI - timetables of Rodalies de la Generalitat de Catalunya in your terminal
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Search the ID of a given station's name pattern, to later use it on your origin or destination
    #[clap(short, long, default_value = "")]
    pub search: String,
    /// The origin's station ID
    #[clap(short, long, default_value = "")]
    pub from: String,
    /// The destinations's station ID
    #[clap(short, long, default_value = "")]
    pub to: String,
    /// The day value of the date to search for
    #[clap(short, long, default_value_t = Local::today().day())]
    pub day: u32,
    /// The month value of the date to search for
    #[clap(short, long, default_value_t = Local::today().month())]
    pub month: u32,
    /// The year value of the date to search for
    #[clap(short, long, default_value_t = Local::today().year())]
    pub year: i32,
}

pub fn init_cli() -> Args {
    let dt = Local::today();
    println!(
        "🚂 Rodalies CLI 📅 Today's date is {:02}/{:02}/{}",
        dt.day(),
        dt.month(),
        dt.year()
    );
    Args::parse()
}

pub fn init_results_table() -> Table {
    let mut results_table = Table::new();
    results_table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    results_table
}
