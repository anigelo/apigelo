use std::future::Future;
use mongodb::bson::{doc, Document, oid::ObjectId};
use mongodb::{Client, Collection};
use mongodb::options::{ClientOptions};
use futures::stream::TryStreamExt;

use crate::config;

type MongoResult<T>= Result<T, mongodb::error::Error>;

pub async fn get_all_media() -> Vec<Document> {
    let result = get_all(vec![
        doc! {
            "$project": {
                "id": { "$toString": "$_id" },
                "_id": 0, "title": 1, "backdrop": 1, "poster": 1, "description": 1
            }
        }
    ]).await;

    match result {
        Ok(media) => media,
        Err(error) => {
            eprintln!("MongoDB error for 'get_all_media': {:#?}", error);
            vec![]
        }
    }
}

pub async fn get_media(id: String) -> Option<Document> {
    let result = query_with_object_id(id, |media_id| async move {
        get_one(vec![
            doc! { "$match": { "_id": media_id }},
            doc! { "$limit": 1 },
            doc! { "$addFields": { "seasons": { "$size": "$seasons" } } },
            doc! { "$project": { "_id": 0, "id": 0, "path": 0, "folder_title": 0 } }
        ]).await
    }, None).await;

    match result {
        Ok(media) => media,
        Err(error) => {
            eprintln!("MongoDB error for 'get_media': {:#?}", error);
            None
        }
    }
}

pub async fn get_media_seasons(id: String) -> Vec<Document> {
    let result = query_with_object_id(id, |media_id| async move {
        get_all(vec![
            doc! { "$match": { "_id": media_id }},
            doc! { "$limit": 1 },
            doc! { "$unwind": "$seasons" },
            doc! { "$replaceRoot": { "newRoot": "$seasons" } },
            doc! { "$project": { "episodes": { "$size": "$episodes" }, "number": 1, "poster": 1 } }
        ]).await
    }, vec![]).await;

    match result {
        Ok(media) => media,
        Err(error) => {
            eprintln!("MongoDB error for 'get_all_media': {:#?}", error);
            vec![]
        }
    }
}

pub async fn get_season(media_id: String, season_id: u32) -> Option<Document> {
    let result = query_with_object_id(media_id, |media_id| async move {
        get_one(vec![
            doc! { "$match": { "_id": media_id }},
            doc! { "$limit": 1 },
            doc! { "$unwind": "$seasons" },
            doc! { "$replaceRoot": { "newRoot": "$seasons" } },
            doc! { "$match": { "number": season_id } },
            doc! { "$limit": 1 },
            doc! { "$project": {"path":0,"episodes.path":0}}
        ]).await
    }, None).await;

    match result {
        Ok(media) => media,
        Err(error) => {
            eprintln!("MongoDB error for 'get_season': {:#?}", error);
            None
        }
    }
}

async fn query_with_object_id<T, F, Fut>(id: String, query: F, default_result: T) -> MongoResult<T>
    where
        F: FnOnce(ObjectId) -> Fut,
        Fut: Future<Output= MongoResult<T>>
{
    match ObjectId::parse_str(id) {
        Ok(id) => query(id).await,
        Err(error) => {
            eprintln!("Can't parse ObjectId: {:#?}", error);
            Ok(default_result)
        }
    }
}

const APP_NAME: &str = "Apigelo";
const DB: &str = "anigelo";
const COLLECTION: &str = "anime";

async fn init_connection() -> Result<Collection<Document>, mongodb::error::Error> {
    let mut options = ClientOptions::parse(config::db_connection_string()).await?;
    options.app_name = Some(APP_NAME.to_string());

    let collection = Client::with_options(options)?
        .database(DB)
        .collection::<Document>(COLLECTION);

    Ok(collection)
}

async fn get_all(query: Vec<Document>) -> MongoResult<Vec<Document>> {
    let collection = init_connection().await?;

    let result = collection.aggregate(query, None).await?
        .try_collect::<Vec<Document>>().await?;

    Ok(result)
}

async fn get_one(query: Vec<Document>) -> MongoResult<Option<Document>> {
    let collection = init_connection().await?;

    let mut result = collection.aggregate(query, None).await?;
    Ok(result.try_next().await?)
}