use std::{fmt, io};

#[derive(Debug)]
pub enum Error {
    /// HTTP status error: summary and response body.
    Http {
        message: String,
        body: String,
    },
    Other(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Http { message, .. } => f.write_str(message),
            Error::Other(m) => f.write_str(m),
        }
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Other(e.to_string())
    }
}

impl From<ureq::Error> for Error {
    fn from(e: ureq::Error) -> Self {
        match e {
            ureq::Error::Status(code, resp) => Error::Http {
                message: format!("{code} {} for url: {}", resp.status_text(), resp.get_url()),
                body: resp.into_string().unwrap_or_default(),
            },
            e => Error::Other(e.to_string()),
        }
    }
}
