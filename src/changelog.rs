use chrono::{DateTime, NaiveDateTime, Utc};
use semver::Version;
use serde::{Deserialize, Serialize};

use crate::errors::Error;

/// Describes an entry in the API changelog
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ChangeLogEntry {
    /// Version this change is attached to
    pub version: Version,
    /// Time of change
    pub time: DateTime<Utc>,
    /// Change message
    pub message: String,
}

impl ChangeLogEntry {
    /// Construct a new ChangeLogEntry
    pub fn new(version: &String, text: &String) -> Result<Self, Error> {
        // Split the text into the message and the time
        //
        // The incoming text will be formatted as follows:
        // `201411192022 First production release. 'GET encap' is only endpoint.`
        let mut split_text = text.split_ascii_whitespace();
        let timestamp: i64 = split_text.next().unwrap().parse().unwrap();
        let message = split_text.collect::<Vec<&str>>().join(" ");

        Ok(Self {
            version: version.parse()?,
            time: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(timestamp, 0), Utc),
            message,
        })
    }
}
