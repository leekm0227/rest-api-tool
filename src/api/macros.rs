macro_rules! make_router {
    ($struct:ident; sub: $($sub_struct:ident),*; $methods:expr) => {{
        let scope_name = stringify!($struct).to_string().to_lowercase();
        let mut scope = actix_web::web::scope(scope_name.as_str());

        for method in $methods {
            match method {
                "GET" => {
                    scope = scope
                        .route(
                            "/{_id}",
                            actix_web::web::get().to(handlers::get::<$struct>),
                        )
                        .route(
                            "",
                            actix_web::web::get().to(handlers::list::<$struct>),
                        );
                }
                "POST" => {
                    scope = scope.route(
                        "",
                        actix_web::web::post().to(handlers::post::<$struct>),
                    );
                }
                "PATCH" => {
                    scope = scope.route(
                        "/{_id}",
                        actix_web::web::patch().to(handlers::patch::<$struct>),
                    );
                }
                "DELETE" => {
                    scope = scope.route(
                        "/{_id}",
                        actix_web::web::delete().to(handlers::delete::<$struct>),
                    );
                }
                _ => (),
            }
        }

        $(
            let sub_name = stringify!($sub_struct).to_string().to_lowercase();
            let (uri, uri_id) = (
                format!("/{{{}_id}}/{}", scope_name, sub_name),
                format!("/{{{}_id}}/{}/{{_id}}", scope_name, sub_name),
            );

            for method in $methods {
                match method {
                    "GET" => {
                        scope = scope
                            .route(
                                uri_id.as_str(),
                                actix_web::web::get().to(handlers::get::<$sub_struct>),
                            )
                            .route(
                                uri.as_str(),
                                actix_web::web::get().to(handlers::list::<$sub_struct>),
                            );
                    }
                    "POST" => {
                        scope = scope.route(
                            uri.as_str(),
                            actix_web::web::post().to(handlers::post::<$sub_struct>),
                        );
                    }
                    "PATCH" => {
                        scope = scope.route(
                            uri_id.as_str(),
                            actix_web::web::patch().to(handlers::patch::<$sub_struct>),
                        );
                    }
                    "DELETE" => {
                        scope = scope.route(
                            uri_id.as_str(),
                            actix_web::web::delete().to(handlers::delete::<$sub_struct>),
                        );
                    }
                    _ => (),
                }
            }
        )*

        scope
    }};
}

macro_rules! make_model {
    (name: $struct_name:ident; relation: $($rl_fd_name:ident),*; fields: $($fd_name: ident: $type: ty),*) => {
        #[derive(serde::Serialize, serde::Deserialize, Debug)]
        pub struct $struct_name {
            #[serde(rename(serialize="id", deserialize="_id"), serialize_with = "utils::oid_to_str")]
            pub _id: Option<mongodb::bson::oid::ObjectId>,

            $(
            #[serde(skip_serializing_if = "Option::is_none", serialize_with = "utils::oid_to_str")]
            pub $rl_fd_name: Option<mongodb::bson::oid::ObjectId>,
            )*

            #[serde(skip_serializing_if = "Option::is_none")]
            pub ct: Option<i64>,

            #[serde(skip_serializing_if = "Option::is_none")]
            pub ut: Option<i64>,

            $(
            #[serde(skip_serializing_if = "Option::is_none")]
            pub $fd_name: Option<$type>,
            )*
        }

        impl utils::Mongo<$struct_name> for $struct_name{}
    }
}
