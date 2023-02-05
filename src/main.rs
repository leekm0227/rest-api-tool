mod api;
use actix_web::{web::Data, App, HttpServer};
use mongodb::Client;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Client::with_uri_str(
        "mongodb://admin:0000@cluster0-shard-00-00.umd8v.mongodb.net:27017,cluster0-shard-00-01.umd8v.mongodb.net:27017,cluster0-shard-00-02.umd8v.mongodb.net:27017/test?replicaSet=atlas-4a7seo-shard-0&ssl=true&authSource=admin",
    )
    .await.unwrap()
    .database("FORUM");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db.clone()))
            .configure(api::config)
    })
    .bind(("0.0.0.0", 8888))?
    .run()
    .await
}
