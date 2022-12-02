macro_rules! make_router {
    ($struct:ident, $methods:expr) => {{
        let mut scope = actix_web::web::scope(stringify!($struct).to_lowercase().as_str());
        for method in $methods {
            match method {
                "GET" => {
                    scope = scope
                        .route("/{id}", actix_web::web::get().to(handlers::get::<$struct>))
                        .route("", actix_web::web::get().to(handlers::list::<$struct>));
                }
                "POST" => {
                    scope = scope.route("", actix_web::web::post().to(handlers::post::<$struct>));
                }
                "PATCH" => {
                    scope = scope.route(
                        "/{id}",
                        actix_web::web::patch().to(handlers::patch::<$struct>),
                    );
                }
                "DELETE" => {
                    scope = scope.route(
                        "/{id}",
                        actix_web::web::delete().to(handlers::delete::<$struct>),
                    );
                }
                _ => (),
            }
        }

        scope
    }};
}

macro_rules! make_model {
    ($struct_name:ident, $($fd_name: ident: $type: ty),*) => {
        #[derive(serde::Serialize, serde::Deserialize, Debug)]
        pub struct $struct_name {
            #[serde(rename(serialize="id", deserialize="_id"), serialize_with = "utils::oid_to_str")]
            pub _id: Option<mongodb::bson::oid::ObjectId>,

            #[serde(skip_serializing_if = "Option::is_none", default="utils::get_timestamp")]
            pub ct: Option<i64>,

            #[serde(skip_serializing_if = "Option::is_none")]
            pub ut: Option<i64>,

            #[serde(skip_serializing_if = "Option::is_none")]
            $(pub $fd_name: Option<$type>),*
        }

        impl utils::Mongo<$struct_name> for $struct_name{}
    }
}
