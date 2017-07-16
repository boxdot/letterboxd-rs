use std::error::Error as StdError;
use std::fmt;

use hyper::StatusCode;
use hyper::Uri;

#[derive(Debug)]
pub struct Error {
    kind: Kind,
    url: Option<Uri>,
}

impl Error {
    pub fn server_error(status: StatusCode, resp: String, url: Uri) -> Error {
        Error {
            kind: Kind::ServerError(status, resp),
            url: Some(url),
        }
    }

    pub fn url(&self) -> Option<&Uri> {
        self.url.as_ref()
    }
}

#[derive(Debug)]
pub enum Kind {
    Http(::hyper::Error),
    Uri(::hyper::error::UriError),
    Json(::serde_json::Error),
    Utf8Error(::std::str::Utf8Error),
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
            Kind::Uri(ref e) => fmt::Display::fmt(e, f),
            Kind::Json(ref e) => fmt::Display::fmt(e, f),
            Kind::Utf8Error(ref e) => fmt::Display::fmt(e, f),
            Kind::ServerError(ref code, ref resp) => {
                write!(f, "Server Error: {}, Response: {}", code, resp)
            }
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match self.kind {
            Kind::Http(ref e) => e.description(),
            Kind::Uri(ref e) => e.description(),
            Kind::Json(ref e) => e.description(),
            Kind::Utf8Error(ref e) => e.description(),
            Kind::ServerError(_, _) => "Server Error",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match self.kind {
            Kind::Http(ref e) => e.cause(),
            Kind::Uri(ref e) => e.cause(),
            Kind::Json(ref e) => e.cause(),
            Kind::Utf8Error(ref e) => e.cause(),
            Kind::ServerError(_, _) => None,
        }
    }
}

impl From<::hyper::Error> for Kind {
    fn from(err: ::hyper::Error) -> Self {
        Kind::Http(err)
    }
}

impl From<::hyper::error::UriError> for Kind {
    fn from(err: ::hyper::error::UriError) -> Self {
        Kind::Uri(err)
    }
}

impl From<::serde_json::Error> for Kind {
    fn from(err: ::serde_json::Error) -> Self {
        Kind::Json(err)
    }
}

impl From<::std::str::Utf8Error> for Kind {
    fn from(err: ::std::str::Utf8Error) -> Self {
        Kind::Utf8Error(err)
    }
}

impl<E> From<E> for Error
where
    Kind: From<E>,
{
    fn from(err: E) -> Self {
        Error {
            kind: Kind::from(err),
            url: None,
        }
    }
}
