#![doc = include_str!("../README.md")]

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

#[macro_use]
extern crate oplfs_derive;

pub mod cli;
pub mod database;
pub mod disc;
mod schema;

#[doc(inline)]
pub use disc::Disc;
