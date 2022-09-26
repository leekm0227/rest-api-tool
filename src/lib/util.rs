macro_rules! make_router {
    ($struct:ident, $methods:expr) => {{
        let mut scope = actix_web::web::scope(stringify!($struct).to_lowercase().as_str());
        for method in $methods {
            match method {
                "GET" => {
                    scope = scope
                    .route("/{id}", actix_web::web::get().to(handler::get::<$struct>))
                    .route("", actix_web::web::get().to(handler::get_list::<$struct>));
                }
                "POST" => {
                    // scope = scope.route("", actix_web::web::post().to(handler::post::<$struct>));
                }
                "PATCH" => {
                    scope =
                        scope.route("/{id}", actix_web::web::put().to(handler::patch::<$struct>));
                }
                "DELETE" => {
                    scope = scope.route(
                        "/{id}",
                        actix_web::web::delete().to(handler::delete::<$struct>),
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
            $(pub $fd_name: $type),*
        }

        impl handler::Mongo<$struct_name> for $struct_name{}
    }
}
