//! an `extern crate` loading macros must be at the crate root
#[macro_use(log, debug, info)]
extern crate log;
#[macro_use]
extern crate serde_derive;

pub mod ch01;
pub mod ch02;
pub mod ch03;