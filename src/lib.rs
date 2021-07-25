use semver::Version;

pub mod errors;
pub mod auth;
use errors::Error;
use auth::Auth;

/// User agent used to identify the client.
const USER_AGENT: &str =
    "ampr-api/0.1.x (An AMPRNET API client for Rust) https://github.com/ewpratten/ampr-api";

/// API base URL
const BASE_URL: &str = "https://portal.ampr.org/api/v1";


/// Returns the current version of the API.
pub async fn get_api_version(auth: &Auth) -> Result<Version, Error> {
    // Set up and send the request.
    let res = reqwest::ClientBuilder::new()
        .user_agent(USER_AGENT)
        .build()
        .unwrap()
        .post(format!("{}/{}", BASE_URL, "version"))
        .basic_auth(auth.callsign.clone(), Some(auth.api_key.clone()))
        .send()
        .await?;

    // Parse the response.
    Ok(res.text().await?.parse()?)
}
