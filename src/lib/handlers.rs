use std::fmt::Debug;

use super::utils::Mongo;
use actix_web::{
    web::{Data, Json, Path},
    HttpResponse,
};

use serde::{de::DeserializeOwned, Serialize};

pub async fn list<T>(db: Data<mongodb::sync::Database>) -> HttpResponse
where
    T: DeserializeOwned + Unpin + Send + Sync + Serialize + Mongo<T> + Debug,
{
    HttpResponse::Ok().json(T::find_all(db))
}

pub async fn get<T>(db: Data<mongodb::sync::Database>, id: Path<String>) -> HttpResponse
where
    T: DeserializeOwned + Unpin + Send + Sync + Serialize + Mongo<T> + Debug,
{
    HttpResponse::Ok().json(T::find_one(db, id.into_inner()))
}

pub async fn post<T>(db: Data<mongodb::sync::Database>, body: Json<T>) -> HttpResponse
where
    T: DeserializeOwned + Unpin + Send + Sync + Serialize + Mongo<T> + Debug,
{
    HttpResponse::Ok().json(T::insert_one(db, body.into_inner()))
}

pub async fn patch<T>(
    db: Data<mongodb::sync::Database>,
    id: Path<String>,
    body: Json<T>,
) -> HttpResponse
where
    T: DeserializeOwned + Unpin + Send + Sync + Serialize + Mongo<T> + Debug,
{
    HttpResponse::Ok().json(T::update_one(db, id.into_inner(), body.to_doc()))
}

pub async fn delete<T>(db: Data<mongodb::sync::Database>, id: Path<String>) -> HttpResponse
where
    T: DeserializeOwned + Unpin + Send + Sync + Serialize + Mongo<T> + Debug,
{
    HttpResponse::Ok().json(T::delete_one(db, id.into_inner()))
}
