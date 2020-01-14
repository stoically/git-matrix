use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Error {
    pub message: String,
}

impl From<git2::Error> for Error {
    fn from(error: git2::Error) -> Error {
        Error {
            message: format!("{}", error),
        }
    }
}

impl From<ruma_client::Error> for Error {
    fn from(error: ruma_client::Error) -> Error {
        Error {
            message: format!("{}", error),
        }
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(error: serde_json::error::Error) -> Error {
        Error {
            message: format!("{}", error),
        }
    }
}

impl From<std::env::VarError> for Error {
    fn from(error: std::env::VarError) -> Error {
        Error {
            message: format!("{}", error),
        }
    }
}

impl From<url::ParseError> for Error {
    fn from(error: url::ParseError) -> Error {
        Error {
            message: format!("{}", error),
        }
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(error: std::str::Utf8Error) -> Error {
        Error {
            message: format!("{}", error),
        }
    }
}

impl From<ruma_client::identifiers::Error> for Error {
    fn from(error: ruma_client::identifiers::Error) -> Error {
        Error {
            message: format!("{}", error),
        }
    }
}
