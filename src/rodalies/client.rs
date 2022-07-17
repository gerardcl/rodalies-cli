use std::time::Duration;
use surf::{Client, Config, Url};

pub fn init_client() -> Client {
    let rodalies_url = "https://rodalies.gencat.cat";

    Config::new()
        .set_base_url(Url::parse(rodalies_url).unwrap())
        .set_timeout(Some(Duration::from_secs(5)))
        .try_into()
        .unwrap()
}
