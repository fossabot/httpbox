// ------------------------------------------------------------------------------- 3rd PARTY IMPORTS
// CLI Arguments parsing dependency
#[macro_use]
extern crate clap;
// Logging dependency
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate reqwest;

use clap::ArgMatches;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use log::LevelFilter;
use reqwest::Client;
use std::io::Write;
use std::time::Duration;
use std::process;

mod cli;

// --------------------------------------------------------------------------------------- CONSTANTS
fn main() {
  // Parse input arguments
  let arg_matches = cli::build_arg_matches();
  trace!("Parsed input arguments: {:#?}", arg_matches);

  // Configure logging
  let log_level_filter = cli::get_log_level_filter(&arg_matches);
  init_logging(log_level_filter);
  trace!("log level: {}", log_level_filter);

  // Parse input
  let url = cli::get_url(&arg_matches);
  let method = cli::get_method(&arg_matches);
  let mut output_writer = match cli::get_output_writer(&arg_matches, false) {
    Ok(boxed_writer) => boxed_writer,
    Err(err) => {
      error!("Unable to open output: {}", err);
      process::exit(1) //< TODO Define a range of error codes to pick from
    }
  };

  // Log configuration after parsing
  trace!("{}: {}", cli::ARG_URL, url);
  trace!("{}: {}", cli::ARG_METHOD, method);

  // Build client, based on input arguments
  let client = build_client(&arg_matches);

  // Setup request
  let request = client.request(method, url).build().unwrap(); //< TODO More request setup/options here

  // Execute request and get response
  let mut response = client.execute(request).unwrap();
  trace!("{:#?}", response);

  // Write
  let bytes_written = response.copy_to(output_writer.as_mut()).unwrap();
  output_writer.flush().unwrap();
  trace!("Written {} bytes", bytes_written);
}

#[allow(unused)]
fn build_client(arg_matches: &ArgMatches) -> Client {
  reqwest::Client::builder()
    .gzip(true) // TODO make configurable via `arg_matches`
    .timeout(Duration::from_secs(30)) // TODO make configurable via `arg_matches`
    .build().unwrap()
}

fn init_logging(log_level_filter: LevelFilter) {
  // TODO Make the following list of packages to filter out a bit more flexible
  let log_config = Config::builder()
    .appender(Appender::builder().build("stdout", Box::new(ConsoleAppender::builder().build())))
    .logger(Logger::builder().build("mio", LevelFilter::Off))
    .logger(Logger::builder().build("tokio_core", LevelFilter::Off))
    .logger(Logger::builder().build("hyper", LevelFilter::Off))
    .build(Root::builder().appender("stdout").build(log_level_filter))
    .unwrap();

  log4rs::init_config(log_config).unwrap();
}

