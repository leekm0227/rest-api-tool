use serde::{Deserialize, Serialize};macro_rules! make_model {($fn_name:ident, $($fd_name: ident: $type: ty),*) => {#[derive(Serialize, Deserialize, Debug)]pub struct $fn_name {$(pub $fd_name: $type),* }}}make_model!(Article, idx: u32,title: String,content: String,ct: u64,ut: u64,user_idx: u32,user_name: String);make_model!(Comment, idx: u32,content: String,ct: u64,ut: u64,user_idx: u32,user_name: String);