extern crate clap;
extern crate futures;
extern crate tokio;
extern crate tokio_process;

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate failure;

mod config;
mod core;

pub use config::Config;
pub use core::{listen, Clipper};
