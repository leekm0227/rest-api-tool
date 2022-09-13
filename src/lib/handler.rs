use actix_web::{
    web::{Data, Json, Path},
    HttpResponse,
};
use mongodb::bson::doc;
use serde::{de::DeserializeOwned, Serialize};
use std::any::type_name;

pub async fn get_list<T>(db: Data<mongodb::sync::Database>) -> HttpResponse
where
    T: DeserializeOwned + Unpin + Send + Sync + Serialize,
{
    let data = db
        .collection::<T>(
            type_name::<T>()
                .to_uppercase()
                .as_str()
                .split("::")
                .last()
                .unwrap(),
        )
        .find(None, None)
        .unwrap()
        .map(|doc| doc.unwrap())
        .collect::<Vec<_>>();
    HttpResponse::Ok().json(data)
}

pub async fn get<T>(db: Data<mongodb::sync::Database>, id: Path<u32>) -> HttpResponse
where
    T: DeserializeOwned + Unpin + Send + Sync + Serialize,
{
    let data = db
        .collection::<T>(
            type_name::<T>()
                .to_uppercase()
                .as_str()
                .split("::")
                .last()
                .unwrap(),
        )
        .find_one(doc! {"idx":id.into_inner()}, None)
        .unwrap();
    HttpResponse::Ok().json(data)
}

pub async fn post<T>(db: Data<mongodb::sync::Database>, document: Json<T>) -> HttpResponse
where
    T: DeserializeOwned + Unpin + Send + Sync + Serialize,
{
    let data = db
        .collection::<T>(
            type_name::<T>()
                .to_uppercase()
                .as_str()
                .split("::")
                .last()
                .unwrap(),
        )
        .insert_one(document.into_inner(), None)
        .unwrap()
        .inserted_id;
    HttpResponse::Ok().json(data)
}

pub async fn patch<T>(db: Data<mongodb::sync::Database>, id: Path<u32>) -> HttpResponse
where
    T: DeserializeOwned + Unpin + Send + Sync + Serialize,
{
    let data = db
        .collection::<T>(
            type_name::<T>()
                .to_uppercase()
                .as_str()
                .split("::")
                .last()
                .unwrap(),
        )
        .find_one(doc! {"idx":id.into_inner()}, None)
        .unwrap();
    HttpResponse::Ok().json(data)
}

pub async fn delete<T>(db: Data<mongodb::sync::Database>, id: Path<u32>) -> HttpResponse
where
    T: DeserializeOwned + Unpin + Send + Sync + Serialize,
{
    let data = db
        .collection::<T>(
            type_name::<T>()
                .to_uppercase()
                .as_str()
                .split("::")
                .last()
                .unwrap(),
        )
        .find_one(doc! {"idx":id.into_inner()}, None)
        .unwrap();
    HttpResponse::Ok().json(data)
}