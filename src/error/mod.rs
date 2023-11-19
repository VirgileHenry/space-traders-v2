use self::server_error::ServerError;

pub mod server_error;
pub mod code;

// todo : better error
// I think error for this api should be Result<Result<T, GameError>, ActuallyBadError> ?
// feels weird that error "json parsing error" and "not enough fuel in ship" are at the same level.

#[derive(Debug)]
pub enum Error {
    ServerErrorResponse {
        status: u16,
        error: ServerError
    },
    ErrorSendingRequest(reqwest::Error),
    JsonParsingError(serde_json::Error),
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::ErrorSendingRequest(value)
    }
}

impl From<(u16, ServerError)> for Error {
    fn from((status, error): (u16, ServerError)) -> Self {
        Error::ServerErrorResponse {
            status,
            error,
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::JsonParsingError(value)
    }
}