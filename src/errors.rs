#[derive(Debug)]
pub enum Error {
    /// An error with the request
    RequestError(reqwest::Error),

    /// SemVer parse error
    SemVerError(semver::Error),
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