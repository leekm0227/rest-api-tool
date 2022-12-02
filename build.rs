use serde_yaml::Mapping;
use std::{
    fs::{read_to_string, File},
    io::Write,
};

fn main() {
    init();
}

fn init() {
    let models: Mapping = serde_yaml::from_str(&read_to_string("./model.yaml").unwrap()).unwrap();
    let tmp = "#[macro_use]mod macros;mod utils;mod handlers;pub fn config(cfg: &mut actix_web::web::ServiceConfig){cfg.service(actix_web::web::resource(\"/\").route(actix_web::web::get().to(|| actix_web::HttpResponse::Ok()))){routes};}{structs}".to_owned();
    let mut routes = "".to_owned();
    let mut structs = "".to_owned();

    for model in models.iter() {
        let struct_name = to_titlecase(model.0.as_str().unwrap());

        for prop in model.1.as_mapping().unwrap().iter() {
            match prop.0.as_str() {
                Some("fields") => {
                    structs.push_str(
                        format!(
                            "make_model![{}, {}];",
                            struct_name,
                            parse_fields(prop.1.as_mapping().unwrap())
                        )
                        .as_str(),
                    );
                }
                Some("methods") => {
                    let methods = prop
                        .1
                        .as_sequence()
                        .unwrap()
                        .iter()
                        .map(|x| serde_yaml::from_value::<String>(x.to_owned()).unwrap())
                        .collect::<Vec<String>>();
                    routes.push_str(
                        format!(
                            ".service(make_router![{}, {:?}.to_vec()])",
                            struct_name, methods
                        )
                        .as_str(),
                    );
                }
                _ => (),
            }
        }
    }

    File::create("./src/lib/mod.rs")
        .unwrap()
        .write(
            tmp.replace("{routes}", &routes)
                .replace("{structs}", &structs)
                .as_bytes(),
        )
        .unwrap();
}

fn to_titlecase(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn parse_fields(fields: &Mapping) -> String {
    let mut result = String::new();

    for field in fields.iter() {
        result.push_str(
            format!(
                "{}: {},",
                field.0.as_str().unwrap(),
                parse_type(field.1.as_str().unwrap())
            )
            .as_str(),
        );

        println!("{:?}", result);
    }

    result.pop();
    result
}

fn parse_type(field_type: &str) -> String {
    eprintln!("field: {}", field_type);
    match field_type.to_lowercase().as_str() {
        "int" => "u32".to_owned(),
        "long" => "u64".to_owned(),
        "string" => "String".to_owned(),
        "bool" => "bool".to_owned(),
        f if f.ends_with("[]") => format!("Vec<{}>", to_titlecase(f.strip_suffix("[]").unwrap())),
        _ => "".to_owned(),
    }
}
