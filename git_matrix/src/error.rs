use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Error {
    pub message: String,
}

impl From<git2::Error> for Error {
    fn from(error: git2::Error) -> Error {
        Error {
            message: error.message().to_owned(),
        }
    }
}

impl From<ruma_client::Error> for Error {
    fn from(_error: ruma_client::Error) -> Error {
        Error {
            message: "Error while talking with the matrix API".to_owned(),
        }
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(_error: serde_json::error::Error) -> Error {
        Error {
            message: "Some serde_json error occured".to_owned(),
        }
    }
}

impl From<std::env::VarError> for Error {
    fn from(_error: std::env::VarError) -> Error {
        Error {
            message: "Some env var error".to_owned(),
        }
    }
}

impl From<url::ParseError> for Error {
    fn from(_error: url::ParseError) -> Error {
        Error {
            message: "Some url parse error".to_owned(),
        }
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(_error: std::str::Utf8Error) -> Error {
        Error {
            message: "Could not convert git oid to utf8 string".to_owned(),
        }
    }
}

impl From<ruma_client::identifiers::Error> for Error {
    fn from(_error: ruma_client::identifiers::Error) -> Error {
        Error {
            message: "Some ruma identifiers error".to_owned(),
        }
    }
}
