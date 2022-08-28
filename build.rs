use serde_yaml::Mapping;
use std::{
    fs::{read_to_string, File},
    io::Write,
};

fn main() {
    // read model yaml
    let models: Mapping = serde_yaml::from_str(&read_to_string("./model.yaml").unwrap()).unwrap();

    // make body
    let mut body = "use serde::{Deserialize, Serialize};macro_rules! make_model {($fn_name:ident, $($fd_name: ident: $type: ty),*) => {#[derive(Serialize, Deserialize, Debug)]pub struct $fn_name {$(pub $fd_name: $type),* }}}".to_string();
    for model in models.iter() {
        for prop in model.1.as_mapping().unwrap().iter() {
            match prop.0.as_str() {
                Some("fields") => {
                    body.push_str(
                        format!(
                            "make_model!({}, {});",
                            to_titlecase(model.0.as_str().unwrap()),
                            parse_fields(prop.1.as_mapping().unwrap())
                        )
                        .as_str(),
                    );
                }
                _ => (),
            }
        }
    }

    // write model rs
    File::create("./src/model.rs")
        .unwrap()
        .write(body.as_bytes())
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
    }

    result.pop();
    result
}

fn parse_type(field_type: &str) -> &str {
    match field_type.to_lowercase().as_str() {
        "int" => "u32",
        "long" => "u64",
        "string" => "String",
        _ => "",
    }
}
