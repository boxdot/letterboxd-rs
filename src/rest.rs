#[macro_export]
macro_rules! GET {
    ($(#[$attr:meta])* $func_name:ident, ($path:expr $(, $arg:ident:$T:ty)*), $ReqType:ty, $RespType:ty) => {
        $(#[$attr])*
        pub fn $func_name(
            &self $( , $arg: $T)*, request: &$ReqType, token: Option<&::defs::AccessToken>
        ) -> Box<dyn (::futures::Future<Item = $RespType, Error = ::error::Error>)> {

            use ::hyper::header::{self, HeaderValue};

            let uri = self.generate_signed_url(
                ::hyper::Method::GET,
                &format!($path $(, $arg)*),
                &::serde_url_params::to_string(request).unwrap(), // TODO: Error handling
                "",
            );
            let uri: ::hyper::Uri = match uri.parse() {
                Ok(uri) => uri,
                Err(err) => {
                    return Box::new(::futures::future::result(Err(Error::from(err))));
                }
            };

            let mut req = ::hyper::Request::get(uri.clone());
            req.header(header::CONTENT_TYPE, HeaderValue::from_static("application/json"))
                .header(header::CONTENT_LENGTH, HeaderValue::from_static("0"));
            if let Some(token) = token {
                req.header(
                    header::AUTHORIZATION,
                    HeaderValue::from_str(&format!("Bearer {}", token.access_token)).unwrap()
                );
            };
            let req = req.body(::hyper::Body::empty()).unwrap();

            let do_req = self.hyper_client.request(req).from_err();
            let fut_resp = do_req.and_then(move |resp| {
                let status_code = resp.status();
                let body = resp.into_body().concat2().from_err();
                body.and_then(move |chunk| if !status_code.is_success() {
                    let resp = String::from(::std::str::from_utf8(&chunk)?);
                    Err(Error::server_error(status_code, resp, uri))
                } else {
                    let json: $RespType = ::serde_json::from_slice(&chunk)?;
                    Ok(json)
                })
            });
            Box::new(fut_resp)
        }
    };
    ($(#[$attr:meta])* $func_name:ident, $path:expr, $ReqType:ty, $RespType:ty) => {
        GET!($(#[$attr])* $func_name, ($path), $ReqType, $RespType);
    };
    ($(#[$attr:meta])* $func_name:ident, ($path:expr $(, $arg:ident:$T:ty)*), $RespType:ty) => {
        $(#[$attr])*
        pub fn $func_name(
            &self $( , $arg: $T)*, token: Option<&::defs::AccessToken>
        ) -> Box<dyn (::futures::Future<Item = $RespType, Error = ::error::Error>)> {

            use ::hyper::header::{self, HeaderValue};

            let uri = self.generate_signed_url(
                ::hyper::Method::GET,
                &format!($path $(, $arg)*),
                "",
                "",
            );
            let uri: ::hyper::Uri = match uri.parse() {
                Ok(uri) => uri,
                Err(err) => {
                    return Box::new(::futures::future::result(Err(Error::from(err))));
                }
            };

            let mut req = ::hyper::Request::get(uri.clone());
            req.header(header::CONTENT_TYPE, HeaderValue::from_static("application/json"))
                .header(header::CONTENT_LENGTH, HeaderValue::from_static("0"));
            if let Some(token) = token {
                req.header(
                    header::AUTHORIZATION,
                    HeaderValue::from_str(&format!("Bearer {}", token.access_token)).unwrap()
                );
            };
            let req = req.body(::hyper::Body::empty()).unwrap();

            let do_req = self.hyper_client.request(req).from_err();
            let fut_resp = do_req.and_then(move |resp| {
                let status_code = resp.status();
                let body = resp.into_body().concat2().from_err();
                body.and_then(move |chunk| if !status_code.is_success() {
                    let resp = String::from(::std::str::from_utf8(&chunk)?);
                    Err(Error::server_error(status_code, resp, uri))
                } else {
                    let json: $RespType = ::serde_json::from_slice(&chunk)?;
                    Ok(json)
                })
            });
            Box::new(fut_resp)
        }
    };
    ($(#[$attr:meta])* $func_name:ident, $path:expr, $RespType:ty) => {
        GET!($(#[$attr])* $func_name, ($path), $RespType);
    };
}

#[macro_export]
macro_rules! POST {
    ($(#[$attr:meta])* $func_name:ident, ($path:expr $(, $arg:ident:$T:ty)*), $ReqType:ty, $RespType:ty) => {
        $(#[$attr])*
        pub fn $func_name(
            &self $( , $arg: $T)*, request: &$ReqType, token: &::defs::AccessToken
        ) -> Box<dyn (::futures::Future<Item = $RespType, Error = Error>)> {

            use ::hyper::header::{self, HeaderValue};

            let body = match ::serde_json::to_string(request) {
                Ok(body) => body,
                Err(err) => return Box::new(::futures::future::result(Err(Error::from(err)))),
            };
            println!("{:?}", body);
            let uri: hyper::Uri = match self.generate_signed_url(
                ::hyper::Method::POST,
                &format!($path $(, $arg)*),
                "",
                &body,
            ).parse() {
                Ok(uri) => uri,
                Err(err) => return Box::new(::futures::future::result(Err(Error::from(err)))),
            };

            let req = ::hyper::Request::post(uri.clone())
                .header(header::CONTENT_TYPE, HeaderValue::from_static("application/json"))
                .header(
                    header::CONTENT_LENGTH,
                    HeaderValue::from_str(&format!("{}", body.len())).unwrap())
                .header(
                    header::AUTHORIZATION,
                    HeaderValue::from_str(&format!("Bearer {}", token.access_token)).unwrap()
                )
                .body(::hyper::Body::from(body))
                .unwrap();

            let do_req = self.hyper_client.request(req).from_err();
            let fut_resp = do_req.and_then(move |resp| {
                let status_code = resp.status();
                let body = resp.into_body().concat2().from_err();
                body.and_then(move |chunk| if !status_code.is_success() {
                    let resp = String::from(::std::str::from_utf8(&chunk)?);
                    Err(Error::server_error(status_code, resp, uri))
                } else {
                    let json: $RespType = ::serde_json::from_slice(&chunk)?;
                    Ok(json)
                })
            });
            Box::new(fut_resp)
        }
    };
    ($(#[$attr:meta])* $func_name:ident, $path:expr, $ReqType:ty, $RespType:ty) => {
        POST!($(#[$attr])* $func_name, ($path), $ReqType, $RespType);
    };
}

#[macro_export]
macro_rules! PATCH {
    ($(#[$attr:meta])* $func_name:ident, ($path:expr $(, $arg:ident:$T:ty)*), $ReqType:ty, $RespType:ty ) => {
        $(#[$attr])*
        pub fn $func_name(
            &self $( , $arg: $T)*, request: &$ReqType, token: &::defs::AccessToken
        ) -> Box<dyn (::futures::Future<Item = $RespType, Error = Error>)> {

            use ::hyper::header::{self, HeaderValue};

            let body = match ::serde_json::to_string(request) {
                Ok(body) => body,
                Err(err) => return Box::new(::futures::future::result(Err(Error::from(err)))),
            };
            let uri: hyper::Uri = match self.generate_signed_url(
                ::hyper::Method::PATCH,
                &format!($path $(, $arg)*),
                "",
                &body,
            ).parse() {
                Ok(uri) => uri,
                Err(err) => return Box::new(::futures::future::result(Err(Error::from(err)))),
            };

            let req = ::hyper::Request::patch(uri.clone())
                .header(header::CONTENT_TYPE, HeaderValue::from_static("application/json"))
                .header(
                    header::CONTENT_LENGTH,
                    HeaderValue::from_str(&format!("{}", body.len())).unwrap())
                .header(
                    header::AUTHORIZATION,
                    HeaderValue::from_str(&format!("Bearer {}", token.access_token)).unwrap()
                )
                .body(::hyper::Body::from(body))
                .unwrap();

            let do_req = self.hyper_client.request(req).from_err();
            let fut_resp = do_req.and_then(move |resp| {
                let status_code = resp.status();
                let body = resp.into_body().concat2().from_err();
                body.and_then(move |chunk| if !status_code.is_success() {
                    let resp = String::from(::std::str::from_utf8(&chunk)?);
                    Err(Error::server_error(status_code, resp, uri))
                } else {
                    let json: $RespType = ::serde_json::from_slice(&chunk)?;
                    Ok(json)
                })
            });
            Box::new(fut_resp)
        }
    };
    ($(#[$attr:meta])* $func_name:ident, $path:expr, $ReqType:ty, $RespType:ty) => {
        PATCH!($(#[$attr])* $func_name, ($path), $ReqType, $RespType);
    };
}

#[macro_export]
macro_rules! DELETE {
    ($(#[$attr:meta])* $func_name:ident, ($path:expr $(, $arg:ident:$T:ty)+)) => {
        $(#[$attr])*
        pub fn $func_name(
            &self $( , $arg: $T)*, token: &::defs::AccessToken
        ) -> Box<dyn (::futures::Future<Item = ::hyper::StatusCode, Error = ::error::Error>)> {

            use ::hyper::header::{self, HeaderValue};

            let uri = self.generate_signed_url(
                ::hyper::Method::DELETE,
                &format!($path $(, $arg)*),
                "",
                "",
            );
            let uri: ::hyper::Uri = match uri.parse() {
                Ok(uri) => uri,
                Err(err) => {
                    return Box::new(::futures::future::result(Err(Error::from(err))));
                }
            };

            let req = ::hyper::Request::delete(uri.clone())
                .header(header::CONTENT_TYPE, HeaderValue::from_static("application/json"))
                .header(header::CONTENT_LENGTH, HeaderValue::from_static("0"))
                .header(
                    header::AUTHORIZATION,
                    HeaderValue::from_str(&format!("Bearer {}", token.access_token)).unwrap()
                )
                .body(::hyper::Body::empty())
                .unwrap();

            let do_req = self.hyper_client.request(req).from_err();
            let fut_resp = do_req.and_then(move |resp| {
                let status_code = resp.status();
                if !status_code.is_success() {
                    Err(Error::server_error(status_code, String::new(), uri))
                } else {
                    Ok(status_code)
                }
            });
            Box::new(fut_resp)
        }
    };
}
