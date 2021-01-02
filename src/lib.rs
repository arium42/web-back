#![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

pub mod api;
