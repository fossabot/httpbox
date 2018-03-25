#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate reqwest;

pub mod core;
pub mod cli;

#[cfg(test)]
mod cli_test;
