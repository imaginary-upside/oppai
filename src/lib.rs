// Hides compiler warning from diesel
// https://github.com/diesel-rs/diesel/issues/1785
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;

pub mod backend;
pub mod schema;
pub mod server;
pub mod config;
