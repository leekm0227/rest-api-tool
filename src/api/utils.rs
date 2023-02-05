use actix_web::{web::Data, HttpRequest};
use async_trait::async_trait;
use chrono::Local;
use data_encoding::BASE64URL_NOPAD;
use futures::stream::StreamExt;
use mongodb::{
    bson::{oid::ObjectId, Document},
    options::{FindOneAndUpdateOptions, ReturnDocument},
};
use serde::{de::DeserializeOwned, Serialize};
use std::{any::type_name, fmt::Debug};

#[async_trait]
pub trait Mongo<T>
where
    T: DeserializeOwned + Unpin + Send + Sync + Serialize,
    Self: Debug + Serialize,
{
    fn get_coll_name() -> String {
        type_name::<T>().split("::").last().unwrap().to_uppercase()
    }

    async fn find_all(db: Data<mongodb::Database>, query: Document) -> Vec<T> {
        db.collection::<T>(Self::get_coll_name().as_str())
            .find(query, None)
            .await
            .unwrap()
            .map(|x| x.unwrap())
            .collect::<Vec<_>>()
            .await
    }

    async fn find_one(db: Data<mongodb::Database>, query: Document) -> Option<T> {
        db.collection::<T>(Self::get_coll_name().as_str())
            .find_one(query, None)
            .await
            .unwrap()
    }

    async fn insert_one(db: Data<mongodb::Database>, doc: Document) -> String {
        let bson = db
            .collection(Self::get_coll_name().as_str())
            .insert_one(doc, None)
            .await
            .unwrap()
            .inserted_id;

        BASE64URL_NOPAD.encode(&bson.as_object_id().unwrap().bytes())
    }

    async fn update_one(db: Data<mongodb::Database>, query: Document, doc: Document) -> Option<T> {
        let options = FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .build();

        db.collection::<T>(Self::get_coll_name().as_str())
            .find_one_and_update(query, doc, options)
            .await
            .unwrap()
    }

    async fn delete_one(db: Data<mongodb::Database>, query: Document) -> bool {
        let del_count = db
            .collection::<T>(Self::get_coll_name().as_str())
            .delete_one(query, None)
            .await
            .unwrap()
            .deleted_count;

        del_count > 0
    }
}

pub fn oid_to_str<S>(oid: &Option<mongodb::bson::oid::ObjectId>, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match oid.as_ref().map(|x| x.bytes()) {
        Some(v) => s.serialize_some::<String>(&BASE64URL_NOPAD.encode(&v)),
        None => s.serialize_none(),
    }
}

pub fn get_timestamp() -> Option<i64> {
    Some(Local::now().timestamp_millis())
}

pub fn get_query(req: HttpRequest) -> Document {
    let mut query = Document::new();

    for (key, val) in req.match_info().iter() {
        query.insert(key, str_to_oid(val));
    }

    query
}

pub fn str_to_oid(id: &str) -> ObjectId {
    ObjectId::from_bytes(
        BASE64URL_NOPAD.decode(id.as_bytes()).unwrap().as_slice()[0..12]
            .try_into()
            .unwrap(),
    )
}
