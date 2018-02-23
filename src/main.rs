// ------------------------------------------------------------------------------- 3rd PARTY IMPORTS
// CLI Arguments parsing dependency
#[macro_use]
extern crate clap;
// Logging dependency
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate reqwest;

use clap::{App, Arg, ArgMatches};
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use log::LevelFilter;
use reqwest::{Client, Method, Url};
use std::io::{self, Write};
use std::process;
use std::str::FromStr;
use std::time::Duration;

// --------------------------------------------------------------------------------------- CONSTANTS
const ARG_URL: &'static str = "URL";
const ARG_METHOD: &'static str = "method";
const ARG_OUTPUT: &'static str = "output";
const ARG_VERBOSE: &'static str = "verbose";
const ARG_QUIET: &'static str = "quiet";

fn main() {
  // Parse input arguments
  let arg_matches = determine_input_arguments();
  trace!("Parsed input arguments: {:#?}", arg_matches);

  // Configure logging
  let log_level_filter = determine_log_level_filter(&arg_matches);
  init_logging(log_level_filter);
  trace!("log level: {}", log_level_filter);

  // Parse input
  let url = determine_url(arg_matches.value_of(ARG_URL).unwrap());
  let method = determine_method(arg_matches.value_of(ARG_METHOD).unwrap());

  // Log configuration after parsing
  trace!("{}: {}", ARG_URL, url);
  trace!("{}: {}", ARG_METHOD, method);
  trace!("{}: {}", ARG_OUTPUT, arg_matches.value_of(ARG_OUTPUT).unwrap());

  // Build client, based on input arguments
  let client = build_client(&arg_matches);

  // Setup request
  let request = client.request(method, url).build().unwrap();
  // TODO More request setup/options here

  // Execute request and get response
  let mut response = client.execute(request).unwrap();
  trace!("{:#?}", response);

  // Write to STDOUT (TODO: write to the configured OUTPUT)
  let stdout = io::stdout();
  let mut stdout_writable = stdout.lock();
  let bytes_written = response.copy_to(&mut stdout_writable).unwrap();
  stdout_writable.flush().unwrap();
  trace!("Written {} bytes", bytes_written);
}

#[allow(unused)]
fn build_client(arg_matches: &ArgMatches) -> Client {
  reqwest::Client::builder()
    .gzip(true) // TODO make configurable via `arg_matches`
    .timeout(Duration::from_secs(30)) // TODO make configurable via `arg_matches`
    .build().unwrap()
}

// --------------------------------------------------------------------------------------- UTILITIES
fn supported_methods() -> [&'static str; 9] {
  [
    Method::Get.as_ref(),
    Method::Post.as_ref(),
    Method::Put.as_ref(),
    Method::Delete.as_ref(),
    Method::Head.as_ref(),
    Method::Options.as_ref(),
    Method::Connect.as_ref(),
    Method::Trace.as_ref(),
    Method::Patch.as_ref()
  ]
}

fn determine_input_arguments<'a>() -> ArgMatches<'a> {
  App::new(crate_name!())
    .version(crate_version!())
    .about(crate_description!())
    .author(crate_authors!("\n"))
    .arg(Arg::with_name(ARG_URL)
      .index(1)
      .required(true)
      .help("URL to send the HTTP(S) request to"))
    .arg(Arg::with_name(ARG_METHOD)
      .short(first_char(ARG_METHOD))
      .long(ARG_METHOD)
      .value_name("METHOD")
      .default_value(Method::Get.as_ref())
      .possible_values(&supported_methods())
      .help("HTTP Method"))
    .arg(Arg::with_name(ARG_OUTPUT)
      .short(first_char(ARG_OUTPUT))
      .long(ARG_OUTPUT)
      .value_name("FILE")
      .default_value("STDOUT")
      .help("Write output to FILE"))
    .arg(Arg::with_name(ARG_VERBOSE)
      .short(first_char(ARG_VERBOSE))
      .long(ARG_VERBOSE)
      .multiple(true)
      .required(false)
      .help("Verbosity level (can specify multiple times)"))
    .arg(Arg::with_name(ARG_QUIET)
      .short(first_char(ARG_QUIET))
      .long(ARG_QUIET)
      .required(false)
      .help("Don't write anything to standard output (i.e. 'quiet mode')"))
    .get_matches()
}

fn determine_method(arg_method: &str) -> Method {
  Method::from_str(arg_method).unwrap()
}

fn determine_url(url_str: &str) -> Url {
  match Url::parse(url_str) {
    Ok(url) => url,
    Err(err) => {
      error!("Malformed URL '{}': {}", url_str, err);
      process::exit(1) // TODO Define a range of error codes to pick from
    }
  }
}

fn determine_log_level_filter(arg_matches: &ArgMatches) -> LevelFilter {
  if arg_matches.occurrences_of(ARG_QUIET) == 1 {
    LevelFilter::Warn
  } else {
    match arg_matches.occurrences_of(ARG_VERBOSE) {
      0 => LevelFilter::Info,
      1 => LevelFilter::Debug,
      2 | _ => LevelFilter::Trace
    }
  }
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

fn first_char(input: &str) -> String {
  input.chars().next().unwrap().to_string()
}

