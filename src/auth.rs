/// An Auth token used for all requests to the AMPR API.
pub struct Auth {
    /// User's callsign
    pub(crate) callsign: String,

    /// User's API key
    pub(crate) api_key: String,
}

impl Auth {
    /// Construct a new Auth object.
    pub fn new(callsign: String, api_key: String) -> Self {
        Self { callsign, api_key }
    }

    /// Check if an API key follows AMPR's key format rules
    pub fn is_key_valid(&self) -> bool {
        self.api_key.len() == 32
            && self
                .api_key
                .chars()
                .into_iter()
                .all(|c| c.is_numeric() || c.is_ascii_uppercase())
    }
}
