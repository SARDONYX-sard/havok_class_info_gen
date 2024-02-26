pub type Result<T, E = HkxError> = core::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum HkxError {
    #[error("Error: {0}")]
    String(String),

    #[error("Failed to parse {0} as {1}.")]
    ParseError(String, String),
}

impl From<String> for HkxError {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for HkxError {
    fn from(value: &str) -> Self {
        Self::String(value.to_owned())
    }
}
