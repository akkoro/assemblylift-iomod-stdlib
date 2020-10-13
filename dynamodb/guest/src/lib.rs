use std::fmt;

use assemblylift_core_iomod_guest::{call, export_iomod_guest};
use serde::export::Formatter;
use serde::{Deserialize, Serialize};

use crate::structs::{
    DeleteItemInput, DeleteItemOutput, GetItemInput, GetItemOutput, ListTablesInput,
    ListTablesOutput, PutItemInput, PutItemOutput, UpdateItemInput, UpdateItemOutput,
};

export_iomod_guest!(akkoro, aws, dynamodb);

mod serialization;
pub mod structs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub why: String,
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.why)
    }
}
impl std::error::Error for Error {}

call!(list_tables, ListTablesInput => Result<ListTablesOutput, Error>);
call!(put_item,    PutItemInput    => Result<PutItemOutput, Error>);
call!(get_item,    GetItemInput    => Result<GetItemOutput, Error>);
call!(delete_item, DeleteItemInput => Result<DeleteItemOutput, Error>);
call!(update_item, UpdateItemInput => Result<UpdateItemOutput, Error>);

#[macro_export]
macro_rules! val {
    (B => $val:expr) => {{
        let mut attr = AttributeValue::default();
        attr.b = Some($val);
        attr
    }};
    (S => $val:expr) => {{
        let mut attr = AttributeValue::default();
        attr.s = Some($val.to_string());
        attr
    }};
    (N => $val:expr) => {{
        let mut attr = AttributeValue::default();
        attr.n = Some($val.to_string());
        attr
    }};
}
