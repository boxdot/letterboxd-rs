#[macro_export]
macro_rules! GET {
    ($func_name:ident, ($path:expr $(, $arg:ident:$T:ty)*), $ReqType:ty, $RespType:ty) => {
        pub fn $func_name(
            &self $( , $arg: $T)*, request: &$ReqType, token: Option<&::defs::AccessToken>
        ) -> Box<::futures::Future<Item = $RespType, Error = ::error::Error>> {

            let uri = self.generate_signed_url(
                ::hyper::Method::Get,
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

            let mut req = ::hyper::Request::new(hyper::Method::Get, uri.clone());
            req.headers_mut().set(::hyper::header::ContentType::json());
            req.headers_mut().set(::hyper::header::ContentLength(0));
            if let Some(token) = token {
                req.headers_mut().set(::hyper::header::Authorization(
                    ::hyper::header::Bearer { token: token.access_token.clone() },
                ));
            };

            let do_req = self.hyper_client.request(req).from_err();
            let fut_resp = do_req.and_then(move |resp| {
                let status_code = resp.status();
                let body = resp.body().concat2().from_err();
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
    ($func_name:ident, $path:expr, $ReqType:ty, $RespType:ty) => {
        GET!($func_name, ($path), $ReqType, $RespType);
    };
    ($func_name:ident, ($path:expr $(, $arg:ident:$T:ty)*), $RespType:ty) => {
        pub fn $func_name(
            &self $( , $arg: $T)*, token: Option<&::defs::AccessToken>
        ) -> Box<::futures::Future<Item = $RespType, Error = ::error::Error>> {

            let uri = self.generate_signed_url(
                ::hyper::Method::Get,
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

            let mut req = ::hyper::Request::new(hyper::Method::Get, uri.clone());
            req.headers_mut().set(::hyper::header::ContentType::json());
            req.headers_mut().set(::hyper::header::ContentLength(0));
            if let Some(token) = token {
                req.headers_mut().set(::hyper::header::Authorization(
                    ::hyper::header::Bearer { token: token.access_token.clone() },
                ));
            };

            let do_req = self.hyper_client.request(req).from_err();
            let fut_resp = do_req.and_then(move |resp| {
                let status_code = resp.status();
                let body = resp.body().concat2().from_err();
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
    ($func_name:ident, $path:expr, $RespType:ty) => {
        GET!($func_name, ($path), $RespType);
    };
}

#[macro_export]
macro_rules! POST {
    ($func_name:ident, ($path:expr $(, $arg:ident:$T:ty)*), $ReqType:ty, $RespType:ty) => {
        pub fn $func_name(
            &self $( , $arg: $T)*, request: &$ReqType, token: &::defs::AccessToken
        ) -> Box<::futures::Future<Item = $RespType, Error = Error>> {

            let body = match ::serde_json::to_string(request) {
                Ok(body) => body,
                Err(err) => return Box::new(::futures::future::result(Err(Error::from(err)))),
            };
            println!("{:?}", body);
            let uri: hyper::Uri = match self.generate_signed_url(
                ::hyper::Method::Post,
                &format!($path $(, $arg)*),
                "",
                &body,
            ).parse() {
                Ok(uri) => uri,
                Err(err) => return Box::new(::futures::future::result(Err(Error::from(err)))),
            };

            let mut req = hyper::Request::new(::hyper::Method::Post, uri.clone());
            req.headers_mut().set(::hyper::header::ContentType::json());
            req.headers_mut().set(::hyper::header::ContentLength(
                body.len() as u64,
            ));
            req.headers_mut().set(::hyper::header::Authorization(
                ::hyper::header::Bearer { token: token.access_token.clone() },
            ));
            req.set_body(body);

            let do_req = self.hyper_client.request(req).from_err();
            let fut_resp = do_req.and_then(move |resp| {
                let status_code = resp.status();
                let body = resp.body().concat2().from_err();
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
    ($func_name:ident, $path:expr, $ReqType:ty, $RespType:ty) => {
        POST!($func_name, ($path), $ReqType, $RespType);
    };
}


#[macro_export]
macro_rules! PATCH {
    ($func_name:ident, ($path:expr $(, $arg:ident:$T:ty)*), $ReqType:ty, $RespType:ty ) => {
        pub fn $func_name(
            &self $( , $arg: $T)*, request: &$ReqType, token: &::defs::AccessToken
        ) -> Box<::futures::Future<Item = $RespType, Error = Error>> {

            let body = match ::serde_json::to_string(request) {
                Ok(body) => body,
                Err(err) => return Box::new(::futures::future::result(Err(Error::from(err)))),
            };
            let uri: hyper::Uri = match self.generate_signed_url(
                ::hyper::Method::Patch,
                &format!($path $(, $arg)*),
                "",
                &body,
            ).parse() {
                Ok(uri) => uri,
                Err(err) => return Box::new(::futures::future::result(Err(Error::from(err)))),
            };

            let mut req = hyper::Request::new(::hyper::Method::Patch, uri.clone());
            req.headers_mut().set(::hyper::header::ContentType::json());
            req.headers_mut().set(::hyper::header::ContentLength(
                body.len() as u64,
            ));
            req.headers_mut().set(::hyper::header::Authorization(
                ::hyper::header::Bearer { token: token.access_token.clone() },
            ));
            req.set_body(body);

            let do_req = self.hyper_client.request(req).from_err();
            let fut_resp = do_req.and_then(move |resp| {
                let status_code = resp.status();
                let body = resp.body().concat2().from_err();
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
    ($func_name:ident, $path:expr, $ReqType:ty, $RespType:ty) => {
        PATCH!($func_name, ($path), $ReqType, $RespType);
    };
}

#[macro_export]
macro_rules! DELETE {
    ($func_name:ident, ($path:expr $(, $arg:ident:$T:ty)+)) => {
        pub fn $func_name(
            &self $( , $arg: $T)*, token: &::defs::AccessToken
        ) -> Box<::futures::Future<Item = ::hyper::StatusCode, Error = ::error::Error>> {

            let uri = self.generate_signed_url(
                ::hyper::Method::Delete,
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

            let mut req = ::hyper::Request::new(hyper::Method::Delete, uri.clone());
            req.headers_mut().set(::hyper::header::ContentType::json());
            req.headers_mut().set(::hyper::header::ContentLength(0));
            req.headers_mut().set(::hyper::header::Authorization(
                ::hyper::header::Bearer { token: token.access_token.clone() },
            ));

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
