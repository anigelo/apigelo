use serde::{Deserialize, Serialize};
use std::future::{Ready,ready};
use actix_web::{Responder, HttpResponse, Error, HttpRequest};

#[derive(Deserialize, Serialize, Debug)]
pub struct MediaCollection {
    pub data: Option<Vec<String>>
}

macro_rules! impl_responder {
    ($($t:ty),+) => {
        $(impl Responder for $t {
            type Error = Error;
            type Future = Ready<Result<HttpResponse, Error>>;

            fn respond_to(self, _req: &HttpRequest) -> Self::Future {
                let body = serde_json::to_string(&self).unwrap();

                // Create response and set content type
                ready(Ok(HttpResponse::Ok()
                    .content_type("application/json")
                    .body(body)))
            }
        })+
    }
}

impl_responder!(MediaCollection);