use std::collections::HashMap;

use auth::Auth;
use changelog::ChangeLogEntry;
use encap::EncapEntry;
use errors::Error;
use reqwest::Response;
use semver::Version;

pub mod auth;
pub mod changelog;
pub mod errors;
pub mod encap;

/// User agent used to identify the client.
const USER_AGENT: &str =
    "ampr-api/0.1.x (An AMPRNET API client for Rust) https://github.com/ewpratten/ampr-api";

/// API base URL
const BASE_URL: &str = "https://portal.ampr.org/api/v1";

async fn make_raw_get(auth: &Auth, endpoint: &str) -> Result<Response, Error> {
    Ok(reqwest::ClientBuilder::new()
        .user_agent(USER_AGENT)
        .build()
        .unwrap()
        .post(format!("{}/{}", BASE_URL, endpoint))
        .basic_auth(auth.callsign.clone(), Some(auth.api_key.clone()))
        .send()
        .await?)
}

/// Returns the current version of the API.
pub async fn get_api_version(auth: &Auth) -> Result<Version, Error> {
    // Set up and send the request.
    let res = make_raw_get(auth, "version").await?;

    // Parse the response.
    Ok(
        serde_json::from_str::<HashMap<String, String>>(&res.text().await?)
            .unwrap()
            .get("version")
            .unwrap()
            .parse()?,
    )
}

/// Returns a complete history of changes to the API
pub async fn get_api_changelog(auth: &Auth) -> Result<Vec<ChangeLogEntry>, Error> {
    // Set up and send the request.
    let res = make_raw_get(auth, "changeLog").await?;

    // Parse the response.
    let response_map: HashMap<String, String> = serde_json::from_str(&res.text().await?).unwrap();

    // Build the log
    Ok(response_map
        .iter()
        .map(|(version, text)| ChangeLogEntry::new(version, text).unwrap())
        .collect())
}

/// Returns a list of available endpoints as key/value pairs.
///
/// The key contains the name of the endpoint, whilst the value contains the method for
/// accessing the endpoint, i.e. one of: `GET`, `POST`, `PUT`, `PATCH` or `DELETE`, followed by a
/// single space, followed by a brief description of each endpoint
pub async fn get_api_endpoints(auth: &Auth) -> Result<HashMap<String, String>, Error> {
    // Set up and send the request.
    let res = make_raw_get(auth, "endpoints").await?;

    // Parse the response.
    Ok(serde_json::from_str(&res.text().await?).unwrap())
}

/// Returns the current encap serial number
pub async fn get_encap_serial(auth: &Auth) -> Result<f32, Error> {
    // Set up and send the request.
    let res = make_raw_get(auth, "encapSerial").await?;

    // Parse the response.
    Ok(
        serde_json::from_str::<HashMap<String, String>>(&res.text().await?)
            .unwrap()
            .get("serial")
            .unwrap()
            .parse()?,
    )
}

/// Returns the current live encap data
pub async fn get_encap(auth: &Auth) -> Result<Vec<EncapEntry>, Error> {
    // Set up and send the request.
    let res = make_raw_get(auth, "endpoints").await?;

    // Parse the response.
    Ok(serde_json::from_str(&res.text().await?).unwrap())
}