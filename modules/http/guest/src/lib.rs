pub mod structs;

use assemblylift_core_iomod_guest::{call, export_iomod_guest};
use serde::export::Formatter;
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::structs::*;

export_iomod_guest!(akkoro, std, http);

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

call!(request, HttpRequest => Result<HttpResponse, Error>);
