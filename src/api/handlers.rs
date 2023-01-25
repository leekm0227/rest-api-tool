use super::utils::{get_query, get_timestamp, Mongo, str_to_oid};
use actix_web::{
    web::{Data, Json},
    HttpRequest, HttpResponse,
};

use mongodb::bson::{self, doc};
use serde::{de::DeserializeOwned, Serialize};
use std::fmt::Debug;

pub async fn list<T>(req: HttpRequest, db: Data<mongodb::sync::Database>) -> HttpResponse
where
    T: DeserializeOwned + Unpin + Send + Sync + Serialize + Mongo<T> + Debug,
{
    let query = get_query(req);
    HttpResponse::Ok().json(T::find_all(db, query))
}

pub async fn get<T>(req: HttpRequest, db: Data<mongodb::sync::Database>) -> HttpResponse
where
    T: DeserializeOwned + Unpin + Send + Sync + Serialize + Mongo<T> + Debug,
{
    let query = get_query(req);
    HttpResponse::Ok().json(T::find_one(db, query))
}

pub async fn post<T>(
    req: HttpRequest,
    db: Data<mongodb::sync::Database>,
    body: Json<T>,
) -> HttpResponse
where
    T: DeserializeOwned + Unpin + Send + Sync + Serialize + Mongo<T> + Debug,
{

    let mut body = bson::to_document(&body.into_inner()).unwrap();
    body.insert("ct", get_timestamp());

    for (key, val) in req.match_info().iter() {
        body.insert(key, str_to_oid(val));
    }

    HttpResponse::Ok().json(T::insert_one(db, body))
}

pub async fn patch<T>(
    req: HttpRequest,
    db: Data<mongodb::sync::Database>,
    body: Json<T>,
) -> HttpResponse
where
    T: DeserializeOwned + Unpin + Send + Sync + Serialize + Mongo<T> + Debug,
{
    let mut body = bson::to_document(&body.into_inner()).unwrap();
    body.remove("_id");
    body.remove("ct");
    body.insert("ut", get_timestamp());

    let update = doc! {"$set":body};
    let query = get_query(req);
    HttpResponse::Ok().json(T::update_one(db, query, update))
}

pub async fn delete<T>(req: HttpRequest, db: Data<mongodb::sync::Database>) -> HttpResponse
where
    T: DeserializeOwned + Unpin + Send + Sync + Serialize + Mongo<T> + Debug,
{
    let query = get_query(req);
    HttpResponse::Ok().json(T::delete_one(db, query))
}
