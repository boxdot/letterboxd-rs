#[macro_export]
macro_rules! GET {
    ($func_name:ident, ($path:expr $(, $arg:ident:$T:ty)*), $RequestType:ty, $ResponseType:ty) => {
        pub fn $func_name(
            &self $( , $arg: $T)*, request: &$RequestType, access_token: Option<&::defs::AccessToken>
        ) -> Box<Future<Item = $ResponseType, Error = ::error::Error>> {

            let uri = self.generate_signed_url(
                ::hyper::Method::Get,
                &format!($path $(, $arg)*),
                &::serde_url_params::to_string(request).unwrap(), // TODO: Error handling
                "",
            );
            let uri: ::hyper::Uri = match uri.parse() {
                Ok(uri) => uri,
                Err(err) => {
                    return Box::new(future::result(Err(Error::from(err))));
                }
            };

            let mut req = ::hyper::Request::new(hyper::Method::Get, uri.clone());
            req.headers_mut().set(::hyper::header::ContentType::json());
            req.headers_mut().set(::hyper::header::ContentLength(0));
            if let Some(token) = access_token {
                req.headers_mut().set(::hyper::header::Authorization(
                    ::hyper::header::Bearer { token: token.access_token.clone() },
                ));
            };

            let get = self.hyper_client.request(req).from_err();
            let fut_resp = get.and_then(move |resp| {
                let status_code = resp.status();
                let body = resp.body().concat2().from_err();
                body.and_then(move |chunk| if status_code != hyper::StatusCode::Ok {
                    let resp = String::from(::std::str::from_utf8(&chunk)?);
                    Err(Error::server_error(status_code, resp, uri))
                } else {
                    let json: $ResponseType = serde_json::from_slice(&chunk)?;
                    Ok(json)
                })
            });
            Box::new(fut_resp)
        }
    };
    ($func_name:ident, $path:expr, $ReqT:ty, $RespT:ty) => {
        GET!($func_name, ($path), $ReqT, $RespT);
    };
}

#[macro_export]
macro_rules! PATCH {
    ($func_name:ident, ($path:expr $(, $arg:ident:$T:ty)*), $RequestType:ty, $ResponseType:ty) => {
        pub fn $func_name(
            &self $( , $arg: $T)*, request: &$RequestType, access_token: &::defs::AccessToken
        ) -> Box<Future<Item = $ResponseType, Error = Error>> {

            let body = match serde_json::to_string(request) {
                Ok(body) => body,
                Err(err) => return Box::new(future::result(Err(Error::from(err)))),
            };
            let uri: hyper::Uri = match self.generate_signed_url(
                hyper::Method::Patch,
                &format!($path $(, $arg)*),
                "",
                &body,
            ).parse() {
                Ok(uri) => uri,
                Err(err) => return Box::new(future::result(Err(Error::from(err)))),
            };
            println!("{:?}", uri);

            let mut req = hyper::Request::new(hyper::Method::Patch, uri.clone());
            req.headers_mut().set(hyper::header::ContentType::json());
            req.headers_mut().set(hyper::header::ContentLength(
                body.len() as u64,
            ));
            req.headers_mut().set(hyper::header::Authorization(
                hyper::header::Bearer { token: access_token.access_token.clone() },
            ));
            req.set_body(body);

            let patch = self.hyper_client.request(req).from_err();
            let fut_resp = patch.and_then(move |resp| {
                let status_code = resp.status();
                let body = resp.body().concat2().from_err();
                body.and_then(move |chunk| if status_code != hyper::StatusCode::Ok {
                    let resp = String::from(::std::str::from_utf8(&chunk)?);
                    Err(Error::server_error(status_code, resp, uri))
                } else {
                    let json: $ResponseType = serde_json::from_slice(&chunk)?;
                    Ok(json)
                })
            });
            Box::new(fut_resp)
        }
    };
    ($func_name:ident, $path:expr, $ReqT:ty, $RespT:ty) => {
        GET!($func_name, ($path), $ReqT, $RespT);
    };
}
