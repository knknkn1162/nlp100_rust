//! an `extern crate` loading macros must be at the crate root
#[macro_use(log, debug, info)]
extern crate log;


pub mod ch01;
pub mod ch02;