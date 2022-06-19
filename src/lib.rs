//! # Open PS2 Loader FUSE Filesystem (OPLFS)
//!
//! There are many distinct tasks that we have to take care of:
//!
//! * We have to initialize a SQLite database
//! * We have to index all of the discs and put them into the SQLite database
//! * We have to inspect the ISOs to determine their title IDs, file sizes, and media types
//! * We have to grab artwork from archive.org for the discs we find
//! * We have to grab configuration from GitHub for the discs we find
//! * We have to parse that configuration
//! * We have to expose an OPL-compatible FUSE filesystem whose directory entries come from SQLite
//! * We have to proxy reads from CD/ and DVD/ to the real locations on disk
//! * We have to build files in CFG/ from SQLite when read
//! * We have to do something with writes to CFG/, VMC/, and others (TBD)
//! * We have to return blobs from SQLite for reads from ART/
//!
//! In the future, we should:
//!
//! * index .cue files
//! * transparently read .bin/.cue files and present them as .VCD files for POPStarter
//! * have some way of providing the necessary binaries for POPStarter (via indexer?)
//! * have some way of adding/indexing apps
//! * VMC/cheats/PS1 BIOS support for POPStarter

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
