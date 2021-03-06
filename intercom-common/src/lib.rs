#![recursion_limit = "128"]
#![allow(clippy::match_bool)]

#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate failure;

pub mod ast_converters;
pub mod attributes;
pub mod error;
pub mod guid;
pub mod idents;
pub mod methodinfo;
pub mod model;
pub mod prelude;
pub mod returnhandlers;
pub mod tyhandlers;
pub mod utils;
