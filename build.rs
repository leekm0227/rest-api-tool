use phf::phf_map;
use serde_yaml::Mapping;
use std::{
    collections::HashMap,
    fs::{read_to_string, File},
    io::Write,
    vec,
};

static FIELD_TYPES: phf::Map<&str, &str> = phf_map! {
    "num" => "u64",
    "str" => "String",
    "bool" => "bool",
    "date" => "u64",
};

static METHODS: phf::Map<&str, &str> = phf_map! {
    "c" => "POST",
    "r" => "GET",
    "u" => "PATCH",
    "d" => "DELETE",
};

#[derive(Debug)]
struct ModelInfo {
    struct_name: String,
    endpoints: Vec<String>,
    fields: Vec<FieldInfo>,
    methods: Vec<String>,
}

#[derive(Debug)]
struct FieldInfo {
    field_name: String,
    field_type: String,
    is_array: bool,
}

fn main() {
    let models = serde_yaml::from_str::<Mapping>(&read_to_string("./model.yaml").unwrap()).unwrap();
    let tmp = "#[macro_use]mod macros;mod utils;mod handlers;pub fn config(cfg: &mut actix_web::web::ServiceConfig){cfg.service(actix_web::web::resource(\"/\").route(actix_web::web::get().to(actix_web::HttpResponse::Ok))){routes};}{structs}".to_string();
    let mut relation_map = HashMap::<String, Vec<String>>::new();
    let mut result = vec![];

    let struct_names = models
        .clone()
        .into_iter()
        .map(|model| model.0.as_str().unwrap().to_lowercase())
        .collect::<Vec<_>>();

    for model in models.iter() {
        let mut endpoints = vec!["".to_string()];
        let mut fields = vec![];
        let mut methods = vec![];
        let struct_name = to_titlecase(model.0.as_str().unwrap());

        for prop in model.1.as_mapping().unwrap().iter() {
            match prop.0.as_str() {
                Some("fields") => {
                    for field in prop.1.as_mapping().unwrap().iter() {
                        let field_name = field.0.as_str().unwrap().to_string();
                        let mut field_type = field.1.as_str().unwrap().to_lowercase();
                        let mut is_array = false;

                        if field_type.ends_with("[]") {
                            field_type = field_type.replace("[]", "");
                            is_array = true;
                        }

                        field_type = match field_type {
                            f if FIELD_TYPES.contains_key(&f) => {
                                FIELD_TYPES.get(&f).unwrap().to_string()
                            }
                            f if struct_names.contains(&f.to_string()) => {
                                endpoints.push(to_titlecase(&f));
                                relation_map
                                    .entry(f.clone())
                                    .or_insert(vec![])
                                    .push(model.0.as_str().unwrap().to_string());
                                to_titlecase(&f)
                            }
                            _ => continue,
                        };

                        fields.push(FieldInfo {
                            field_name,
                            field_type,
                            is_array,
                        });
                    }
                }
                Some("methods") => {
                    methods = prop
                        .1
                        .as_str()
                        .unwrap()
                        .split(",")
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>();
                }
                _ => (),
            }
        }

        result.push(ModelInfo {
            struct_name,
            endpoints,
            fields,
            methods,
        });
    }

    let (structs, routes) = result
        .into_iter()
        .map(|info| {
            let mut fields = info
                .fields
                .into_iter()
                .map(|field| {
                    let field_type = match field.is_array {
                        true => format!("Vec<{}>", field.field_type),
                        false => field.field_type,
                    };

                    format!("{}: {},", field.field_name, field_type)
                })
                .reduce(|acc, e| acc + &e)
                .unwrap();
            fields.pop();

            let mut relation_fields = relation_map
                .get(&info.struct_name.to_lowercase())
                .unwrap_or(&vec![])
                .into_iter()
                .map(|x| format!("{}_id,", x))
                .reduce(|acc, e| acc + &e)
                .unwrap_or("".to_string());
            relation_fields.pop();

            let methods = info
                .methods
                .into_iter()
                .filter_map(|x| match METHODS.contains_key(&x) {
                    true => Some(METHODS.get(&x).unwrap().to_string()),
                    _ => None,
                })
                .collect::<Vec<_>>();

            let mut subs = info
                .endpoints
                .into_iter()
                .reduce(|acc, e| acc + &e + ",")
                .unwrap_or("".to_string());
            subs.pop();

            (
                format!(
                    "make_model![name: {}; relation: {}; fields: {}];",
                    info.struct_name, relation_fields, fields
                ),
                format!(
                    ".service(make_router![{}; sub: {}; {:?}.iter().copied()])",
                    info.struct_name, subs, methods
                ),
            )
        })
        .fold(("".to_string(), "".to_string()), |acc, e| {
            (acc.0 + &e.0, acc.1 + &e.1)
        });

    _ = File::create("./src/api/mod.rs").unwrap().write(
        tmp.replace("{routes}", routes.as_str())
            .replace("{structs}", structs.as_str())
            .as_bytes(),
    );
}

fn to_titlecase(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
