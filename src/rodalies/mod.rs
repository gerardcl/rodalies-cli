//! # rodalies
//!
//! The `rodalies` module contains the modules that manage the HTTP client, the stations search and the timetable results page.

/// `client` is the module responsible to handle the HTTP client conifugration and requests.
pub mod client;
/// `station` is the module responsible to handle the processing, filtering and display of stations.
pub mod station;
/// `timetable` is the module responsible to handle the processing, filtering and display of the desired trains' timetable.
pub mod timetable;
