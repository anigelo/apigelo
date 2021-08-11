use serde::{Deserialize,Serialize};
use std::future::{Ready,ready};
use actix_web::{Responder, HttpResponse, Error, HttpRequest};

#[derive(Deserialize, Serialize, Debug)]
pub struct KitsuMediaCollection {
    pub data: Option<Vec<KitsuMedia>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KitsuMedia {
    pub id: Option<String>,
    pub attributes: KitsuMediaAttrs
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct KitsuMediaAttrs {
    pub canonicalTitle: Option<String>,
    pub startDate: Option<String>,
    pub endDate: Option<String>,
    pub posterImage: Option<KitsuMediaAttrsImages>,
    pub coverImage: Option<KitsuMediaAttrsImages>,
    pub synopsis: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KitsuMediaAttrsImages {
    pub tiny: Option<String>,
    pub small: Option<String>,
    pub medium: Option<String>,
    pub large: Option<String>,
    pub original: Option<String>
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

impl_responder!(KitsuMediaCollection, KitsuMedia, KitsuMediaAttrsImages, KitsuMediaAttrs);