use hyper::{StatusCode, Uri};

use std::fmt;

/// Result type returned by `Client`.
pub type Result<T> = std::result::Result<T, Error>;

/// Error type returned by `Client`.
#[derive(Debug)]
pub struct Error {
    kind: Kind,
    url: Option<Uri>,
}
impl Error {
    pub(crate) fn server_error(status: StatusCode, resp: String, url: Uri) -> Error {
        Error {
            kind: Kind::ServerError(status, resp),
            url: Some(url),
        }
    }

    /// Returns error kind which is the cause of this error.
    pub fn kind(&self) -> &Kind {
        &self.kind
    }

    /// Returns the url (if any) for which the error occurred.
    pub fn url(&self) -> Option<&Uri> {
        self.url.as_ref()
    }
}

/// Different kinds of error which might occur.
#[derive(Debug)]
pub enum Kind {
    Http(hyper::Error),
    HttpClient(hyper_util::client::legacy::Error),
    Uri(hyper::http::uri::InvalidUri),
    Json(serde_json::Error),
    Utf8Error(std::str::Utf8Error),
    UrlEncoding(serde_url_params::Error),
    ServerError(StatusCode, String /* response */),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref url) = self.url {
            fmt::Display::fmt(url, f)?;
            f.write_str(": ")?;
        }
        match self.kind {
            Kind::Http(ref e) => fmt::Display::fmt(e, f),
            Kind::HttpClient(ref e) => fmt::Display::fmt(e, f),
            Kind::Uri(ref e) => fmt::Display::fmt(e, f),
            Kind::Json(ref e) => fmt::Display::fmt(e, f),
            Kind::Utf8Error(ref e) => fmt::Display::fmt(e, f),
            Kind::UrlEncoding(ref e) => fmt::Display::fmt(e, f),
            Kind::ServerError(ref code, ref resp) => {
                write!(f, "Server Error: {}, Response: {}", code, resp)
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<hyper::http::uri::InvalidUri> for Error {
    fn from(err: hyper::http::uri::InvalidUri) -> Self {
        Self {
            kind: Kind::Uri(err),
            url: None,
        }
    }
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Self {
        Self {
            kind: Kind::Http(err),
            url: None,
        }
    }
}

impl From<hyper_util::client::legacy::Error> for Error {
    fn from(err: hyper_util::client::legacy::Error) -> Self {
        Self {
            kind: Kind::HttpClient(err),
            url: None,
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self {
            kind: Kind::Json(err),
            url: None,
        }
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(err: std::str::Utf8Error) -> Self {
        Self {
            kind: Kind::Utf8Error(err),
            url: None,
        }
    }
}

impl From<serde_url_params::Error> for Error {
    fn from(err: serde_url_params::Error) -> Self {
        Self {
            kind: Kind::UrlEncoding(err),
            url: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_is_sync_and_send() {
        fn is_sync_and_send<E: Sync + Send>() {}
        is_sync_and_send::<Error>();
    }
}
