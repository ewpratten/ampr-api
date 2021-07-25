use std::num::ParseFloatError;

#[derive(Debug)]
pub enum Error {
    /// An error with the request
    RequestError(reqwest::Error),

    /// SemVer parse error
    SemVerError(semver::Error),

    /// An error with float parsing
    ParseFloatError(ParseFloatError),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::RequestError(e)
    }
}

impl From<semver::Error> for Error {
    fn from(e: semver::Error) -> Self {
        Self::SemVerError(e)
    }
}

impl From<ParseFloatError> for Error {
    fn from(e: ParseFloatError) -> Self {
        Self::ParseFloatError(e)
    }
}
