use reqwest::Client;
use rocket::State;


pub fn remove_suffix<'a>(s: &'a str, suffix: &str) -> &'a str {
    match s.strip_suffix(suffix) {
        Some(s) => s,
        None => s,
    }
}

pub async fn text_request(client: &State<Client>, url: &str) -> Result<String, reqwest::Error> {
    client.get(url).send().await?.text().await
}
