use std::env;

use reqwest::header;
use reqwest::header::HeaderValue;

use crate::github::GithubUser;

struct GithubHeader {}

impl GithubHeader {
    fn new_header_map() -> header::HeaderMap<HeaderValue> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::ACCEPT,
            header::HeaderValue::from_static("application/vnd.github+json"),
        );
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_static("Github Profile App"),
        );
        headers
    }
}

pub async fn fetch_user(name: String) -> Result<GithubUser, Box<dyn std::error::Error>> {
    let token = env::var("GITHUB_ACCESS_TOKEN").expect("GITHUB_ACCESS_TOKEN must be set");
    let client = reqwest::Client::builder()
        .default_headers(GithubHeader::new_header_map())
        .build()?;

    let res = client
        .get(format!("https://api.github.com/users/{}", name))
        .header(header::AUTHORIZATION, format!("token {}", token))
        .send()
        .await?;

    match res.error_for_status_ref() {
        Ok(_) => {}
        Err(err) => {
            return Err(Box::new(err));
        }
    };

    let user: GithubUser = res.json().await?;
    Ok(user)
}
