use scraper::Html;
use std::{error::Error, time::Duration};
use surf::{Client, Config, Response, StatusCode, Url};

/// Configures and returns the HTTP client that will interact with the `rodalies.gencat.cat` site.
pub fn init_client() -> Client {
    let rodalies_url = "https://rodalies.gencat.cat";

    Config::new()
        .set_base_url(Url::parse(rodalies_url).unwrap())
        .set_timeout(Some(Duration::from_secs(5)))
        .try_into()
        .unwrap()
}

/// Returns the HTML body parsed of the main search page.
pub async fn get_search_page(client: &Client) -> Result<Html, Box<dyn Error>> {
    let mut response = client.get("/en/horaris").await?;

    let body_response = get_page_body(&mut response).await?;

    Ok(Html::parse_document(&body_response))
}

/// Returns the HTML body parsed of the timetable searched result page.
pub async fn get_timetable_page(
    client: &Client,
    from: String,
    to: String,
    date: String,
) -> Result<Html, Box<dyn Error>> {
    let mut response = client
        .post("/en/horaris")
        .content_type("application/x-www-form-urlencoded")
        .body_string(format!(
            "origen={}&desti={}&dataViatge={}&horaIni=00&lang=en&cercaRodalies=true&tornada=false",
            from, to, date
        ))
        .await?;

    let body_response = get_page_body(&mut response).await?;

    Ok(Html::parse_document(&body_response))
}

/// Returns the raw body of the provided HTTP response.
async fn get_page_body(response: &mut Response) -> Result<String, Box<dyn Error>> {
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

    Ok(response.body_string().await?)
}

#[cfg(test)]
mod tests {
    use super::init_client;
    use surf::Url;

    #[test]
    fn test_init_client_with_rodalies_web() {
        let client = init_client();
        let expected_url = "https://rodalies.gencat.cat";
        assert!(client
            .config()
            .base_url
            .eq(&Some(Url::parse(expected_url).unwrap())));
    }
}
