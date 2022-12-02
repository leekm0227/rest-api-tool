use std::{any::type_name, fmt::Debug};

use actix_web::web::Data;
use chrono::{Local};
use data_encoding::BASE64URL_NOPAD;
use mongodb::{bson::{self, doc, oid::ObjectId, Document}, options::{FindOneAndUpdateOptions, ReturnDocument}};
use serde::{de::DeserializeOwned, Serialize};

pub trait Mongo<T>
where
    T: DeserializeOwned + Unpin + Send + Sync + Serialize,
    Self: Debug + Serialize,
{
    fn get_coll_name() -> String {
        type_name::<T>().split("::").last().unwrap().to_uppercase()
    }

    fn string_to_oid(id: String) -> ObjectId {
        ObjectId::from_bytes(
            BASE64URL_NOPAD.decode(id.as_bytes()).unwrap().as_slice()[0..12]
                .try_into()
                .unwrap(),
        )
    }

    fn to_doc(&self) -> Document {
        println!("self: {:?}", self);
        // println!("doc: {:?}", doc);
        let mut update = bson::to_document(self).unwrap();
        
        // remove default keys
        update.remove("_id");
        update.remove("ct");
        update.remove("ut");
        update.insert("ut", get_timestamp());

        doc! {"$set":update}
    }

    fn find_all(db: Data<mongodb::sync::Database>) -> Vec<T> {
        db.collection::<T>(Self::get_coll_name().as_str())
            .find(None, None)
            .unwrap()
            .map(|doc| doc.unwrap())
            .collect::<Vec<T>>()
    }

    fn find_one(db: Data<mongodb::sync::Database>, id: String) -> Option<T> {
        let query = doc! {"_id":Self::string_to_oid(id)};
        db.collection::<T>(Self::get_coll_name().as_str())
            .find_one(query, None)
            .unwrap()
    }

    fn insert_one(db: Data<mongodb::sync::Database>, doc: T) -> String {
        let bson = db
            .collection::<T>(Self::get_coll_name().as_str())
            .insert_one(doc, None)
            .unwrap()
            .inserted_id;

        BASE64URL_NOPAD.encode(&bson.as_object_id().unwrap().bytes())
    }

    fn update_one(db: Data<mongodb::sync::Database>, id: String, doc: Document) -> Option<T> {
        let query = doc! {"_id":Self::string_to_oid(id)};
        let options = FindOneAndUpdateOptions::builder().return_document(ReturnDocument::After).build();

        db.collection::<T>(Self::get_coll_name().as_str())
            .find_one_and_update(query, doc, options)
            .unwrap()
    }

    fn delete_one(db: Data<mongodb::sync::Database>, id: String) -> bool {
        let query = doc! {"_id":Self::string_to_oid(id)};
        let del_count = db
            .collection::<T>(Self::get_coll_name().as_str())
            .delete_one(query, None)
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

pub fn get_timestamp() -> Option<i64>{
    Some(Local::now().timestamp_millis())
}

pub fn str_to_oid<S>(id: String, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    s.serialize_some::<ObjectId>(&ObjectId::from_bytes(
        BASE64URL_NOPAD.decode(id.as_bytes()).unwrap().as_slice()[0..12]
            .try_into()
            .unwrap(),
    ))
}
