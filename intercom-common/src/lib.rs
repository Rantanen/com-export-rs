#![feature(non_exhaustive)]

#[macro_use] extern crate quote;
#[macro_use] extern crate syn;
extern crate sha1;
extern crate ordermap;
extern crate toml;
#[macro_use] extern crate failure;


pub mod guid;
pub mod error;
pub mod idents;
pub mod tyhandlers;
pub mod returnhandlers;
pub mod utils;
pub mod ast_converters;
pub mod methodinfo;
pub mod model;
pub mod builtin_model;
pub mod foreign_ty;
