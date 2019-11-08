use std::str;

use futures::{future, Future, Stream};
use hyper::{
    self,
    header::{self, HeaderValue},
    Body,
};
use hyper_tls;
use serde_json;

use uuid;

use defs;
use error::Error;
use helper;

pub struct Client {
    url: String,
    key: String,
    shared_secret: String,
    // state
    hyper_client: hyper::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>,
}

impl Client {
    pub fn new(api_key: String, api_shared_secret: String) -> Client {
        let https = hyper_tls::HttpsConnector::new(4).unwrap();
        let hyper_client = hyper::Client::builder().build::<_, hyper::Body>(https);
        Client {
            url: String::from("https://api.letterboxd.com/api/v0/"),
            key: api_key,
            shared_secret: api_shared_secret,
            hyper_client,
        }
    }

    // auth

    //     /auth/forgotten-password-request

    /// Use a member’s credentials to sign in and receive an authentication
    /// token.
    ///
    /// Use this function to generate or refresh an auth token. See
    /// Authentication for more details.
    pub fn auth(
        &self,
        username: &str,
        password: &str,
    ) -> Box<dyn Future<Item = defs::AccessToken, Error = Error>> {
        let body = format!(
            "grant_type=password&username={}&password={}",
            username, password
        );
        let uri: hyper::Uri = match self
            .generate_signed_url(hyper::Method::POST, "auth/token", "", &body)
            .parse()
        {
            Ok(uri) => uri,
            Err(err) => return Box::new(future::result(Err(Error::from(err)))),
        };

        let req = hyper::Request::post(uri.clone())
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_static("application/x-www-form-urlencoded"),
            )
            .header(header::ACCEPT, HeaderValue::from_static("application/json"))
            .body(Body::from(body))
            .unwrap();

        let req = self.hyper_client.request(req).from_err();
        let fut_resp = req.and_then(move |resp| {
            let status_code = resp.status();
            let body = resp.into_body().concat2().from_err();
            body.and_then(move |chunk| {
                if status_code != hyper::StatusCode::OK {
                    let resp = String::from(str::from_utf8(&chunk)?);
                    Err(Error::server_error(status_code, resp, uri))
                } else {
                    let json: defs::AccessToken = serde_json::from_slice(&chunk)?;
                    Ok(json)
                }
            })
        });
        Box::new(fut_resp)
    }

    //     /auth/username-check

    // comment

    //     /comment/{id}
    //     /comment/{id}/report

    // contributor

    //     /contributor/{id}
    //     /contributor/{id}/contributions

    // film

    GET!(
        /// A cursored window over the list of films.
        // Use the ‘next’ cursor to move through the list. The response will include the film
        // relationships for the signed-in member and the member indicated by the member LID if
        // specified.
        films,
        "films",
        defs::FilmsRequest,
        defs::FilmsResponse
    );

    GET!(
        /// Get a list of services supported by the /films endpoint.
        // Services are returned in alphabetical order. Some services are only available to paying
        // members, so results will vary based on the authenticated member’s status.
        film_services,
        "films/film-services",
        defs::FilmServicesResponse
    );

    GET!(
        /// Get a list of genres supported by the `films` function.
        // Genres are returned in alphabetical order.
        film_genres,
        "films/genres",
        defs::GenresResponse
    );

    GET!(
        /// Get details about a film by ID.
        film,
        ("film/{}", id: &str),
        defs::Film
    );

    GET!(
        /// Get availability data about a film by ID.
        film_availability,
        ("film/{}/availability", id: &str),
        defs::FilmAvailabilityResponse
    );

    GET!(
        /// Get details of the authenticated member’s relationship with a film by ID.
        film_relationship,
        ("film/{}/me", id: &str),
        defs::FilmRelationship
    );

    PATCH!(
        /// Update the authenticated member’s relationship with a film by ID.
        update_film_relationship,
        ("film/{}/me", id: &str),
        defs::FilmRelationshipUpdateRequest,
        defs::FilmRelationshipUpdateResponse
    );

    GET!(
        /// Get details of the authenticated member’s relationship with a film by ID.
        film_relationship_members,
        ("film/{}/members", id: &str),
        defs::MemberFilmRelationshipsRequest,
        defs::MemberFilmRelationshipsResponse
    );

    //     /film/{id}/report

    GET!(
        /// Get statistical data about a film by ID.
        film_statistics,
        ("film/{}/statistics", id: &str),
        defs::FilmStatistics
    );

    // list

    GET!(
        /// A cursored window over a list of lists.
        // Use the ‘next’ cursor to move through the list.
        lists,
        "lists",
        defs::ListsRequest,
        defs::ListsResponse
    );

    POST!(
        /// Create a list.
        post_list,
        "lists",
        defs::ListCreationRequest,
        defs::ListCreateResponse
    );

    GET!(
        /// Get details of a list by ID.
        list,
        ("list/{}", id: &str),
        defs::List
    );

    PATCH!(
        /// Update a list by ID.
        patch_list,
        ("list/{}", id: &str),
        defs::ListUpdateRequest,
        defs::ListUpdateResponse
    );

    DELETE!(
        /// Delete a list by ID.
        delete_list,
        ("list/{}", id: &str)
    );

    //     /list/{id}/comments

    //     /list/{id}/entries

    GET!(
        /// Get entries for a list by ID.
        list_entries,
        ("list/{}/entries", id: &str),
        defs::ListEntriesRequest,
        defs::ListEntriesResponse
    );

    //     /list/{id}/me

    //     /list/{id}/report
    //     /list/{id}/statistics

    // log-entry

    //     /log-entries
    //     /log-entry/{id}
    //     /log-entry/{id}/comments
    //     /log-entry/{id}/me
    //     /log-entry/{id}/report
    //     /log-entry/{id}/statistics

    // me

    //     /me
    //     /me/validation-request

    // member

    //     /members
    //     /members/pronouns
    //     /members/register
    //     /member/{id}
    //     /member/{id}/activity
    //     /member/{id}/list-tags
    //     /member/{id}/list-tags-2
    //     /member/{id}/log-entry-tags
    //     /member/{id}/me
    //     /member/{id}/report
    //     /member/{id}/review-tags
    //     /member/{id}/review-tags-2
    //     /member/{id}/statistics
    //     /member/{id}/watchlist

    // search

    GET!(search, "search", defs::SearchRequest, defs::SearchResponse);

    // helper methods

    fn generate_signed_url(
        &self,
        method: hyper::Method,
        endpoint: &str,
        parameters: &str,
        body: &str,
    ) -> String {
        self.url_with_nonce_and_timestamp(
            method,
            endpoint,
            parameters,
            body,
            helper::nonce(),
            helper::now(),
        )
    }

    fn url_with_nonce_and_timestamp(
        &self,
        method: hyper::Method,
        endpoint: &str,
        parameters: &str,
        body: &str,
        nonce: uuid::Uuid,
        timestamp: u64,
    ) -> String {
        let url = format!(
            "{}{}?apikey={}&nonce={}&timestamp={}{}{}",
            self.url,
            endpoint,
            self.key,
            nonce,
            timestamp,
            if !parameters.is_empty() { "&" } else { "" },
            parameters
        );

        let salted_msg = format!("{}\0{}\0{}", method, url, body);
        format!(
            "{}&signature={}",
            url,
            helper::hmac_sha256(&self.shared_secret, &salted_msg)
        )
    }
}
