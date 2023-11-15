use self::server_error::ServerError;

pub mod server_error;
pub mod code;

#[derive(Debug)]
pub enum Error {
    FromServerError(ServerError),
    RequestError(reqwest::Error),
    JsonParsingError(serde_json::Error),
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::RequestError(value)
    }
}

impl From<ServerError> for Error {
    fn from(value: ServerError) -> Self {
        Error::FromServerError(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::JsonParsingError(value)
    }
}