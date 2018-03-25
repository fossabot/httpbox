// ------------------------------------------------------------------------------- 3rd PARTY IMPORTS
#[macro_use] extern crate log;
extern crate log4rs;
extern crate reqwest;
extern crate httpbox;

use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use log::LevelFilter;
use reqwest::Client;
use std::io::Write;
use std::time::Duration;

use httpbox::cli::CLI;
use httpbox::core::RequestParameters;

// --------------------------------------------------------------------------------------- CONSTANTS
fn main() {
  let cli = CLI::new();
  let mut req_params = cli.get_request_parameters();

  // Configure logging
  let log_level_filter = cli.get_log_level_filter();
  init_logging(log_level_filter);

  // Build client, based on input arguments
  let client = build_client(&req_params);

  // Setup request
  let request = client.request(req_params.method.clone(), req_params.url.clone()).build().unwrap(); //< TODO More request setup/options here

  // Execute request and get response
  let mut response = client.execute(request).unwrap();
  trace!("{:#?}", response);

  // Write
  let output_writer = req_params.borrow_mut_output_writer();
  let bytes_written = response.copy_to(output_writer).unwrap();
  output_writer.flush().unwrap();
  trace!("Written {} bytes", bytes_written);
}

#[allow(unused)]
fn build_client(req_params: &RequestParameters) -> Client {
  reqwest::Client::builder()
    .gzip(true) // TODO make configurable via `req_params`
    .timeout(Duration::from_secs(30)) // TODO make configurable via `req_params`
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

