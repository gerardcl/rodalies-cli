use clap::crate_version;
use serde_json::Value;
use std::{error::Error, time::Duration};
use surf::{Client, Config, Url};

struct IndexResponse {
    name: String,
    version: String,
}

pub async fn check_rodalies_version() {
    let online_crate_state = get_last_crate_index_response().await;
    match online_crate_state {
        Ok(state) => match state.version.as_str() {
            crate_version!() => println!("✅ You are running the latest {}, version {}! yayy", state.name, state.version),
            _ => println!("🤷 You are using an outdated version of rodalies-cli ({}), please upgrade to latest ({})", crate_version!(), state.version)
        },
        _ => {
            println!("🕵️ Could not check if using latest version...dismissing check, but if you keep seeing this message please open an issue.")
        },
    };
}

/// Returns the HTML from`https://raw.githubusercontent.com/rust-lang/crates.io-index/master/ro/da/rodalies-cli`, if possible.
async fn get_last_crate_index_response() -> Result<IndexResponse, Box<dyn Error>> {
    let raw_githubcontent_base_url = "https://raw.githubusercontent.com";

    let client: Client = Config::new()
        .set_base_url(Url::parse(raw_githubcontent_base_url)?)
        .set_timeout(Some(Duration::from_secs(5)))
        .try_into()?;

    let mut response = client
        .get("/rust-lang/crates.io-index/master/ro/da/rodalies-cli")
        .header(
            "User-Agent",
            format!(
                "rodalies-cli/{} (github.com/gerardcl/rodalies-cli)",
                crate_version!()
            ),
        )
        .await?;

    let body_response = response.body_string().await?;

    let last_line = body_response.lines().last().unwrap();

    let index_response: Value = serde_json::from_str(last_line)?;

    Ok(IndexResponse {
        name: index_response["name"].to_string().replace('\"', ""),
        version: index_response["vers"].to_string().replace('\"', ""),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! run_async {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn test_get_index_returns_index_response() {
        let response = run_async!(get_last_crate_index_response());
        let index = response.unwrap();
        assert_eq!(index.name, "rodalies-cli");
        assert!(!index.version.is_empty());
    }
}
