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
use log4rs::config::{Appender, Config, Root};
use log::LevelFilter;
use reqwest::Method;
use std::str::FromStr;

// --------------------------------------------------------------------------------------- CONSTANTS
const ARG_URL: &'static str = "url";
const ARG_METHOD: &'static str = "method";
const ARG_OUTPUT: &'static str = "output";
const ARG_VERBOSE: &'static str = "verbose";
const ARG_QUIET: &'static str = "quiet";

fn main() {
  // Parse input arguments
  let arg_matches = App::new(crate_name!())
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
    .get_matches();

  // Determine the Log Level filtering
  let log_level_filter = determine_log_level_filter(&arg_matches);
  let http_method = determine_http_method(arg_matches.value_of(ARG_METHOD).unwrap());

  // Configure Logging
  init_logging(log_level_filter);

  // Log the given input, as seen by us
  debug!("* {}: {}", ARG_URL, arg_matches.value_of(ARG_URL).unwrap());
  debug!("* {}: {}", ARG_OUTPUT, arg_matches.value_of(ARG_OUTPUT).unwrap());
  debug!("* {}: {}", ARG_METHOD, http_method);
  trace!("* {}: {}", ARG_VERBOSE, arg_matches.occurrences_of(ARG_VERBOSE));
  trace!("* {}: {}", ARG_QUIET, arg_matches.occurrences_of(ARG_QUIET));
  debug!("* log level: {}", log_level_filter);

  // TODO Actually do some http already!

  info!("READY!");
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

fn determine_http_method(arg_method: &str) -> Method {
  Method::from_str(arg_method).unwrap()
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
  let log_config = Config::builder()
    .appender(Appender::builder().build("stdout", Box::new(ConsoleAppender::builder().build())))
    .build(Root::builder().appender("stdout").build(log_level_filter))
    .unwrap();

  log4rs::init_config(log_config).unwrap();
}

fn first_char(input: &str) -> String {
  input.chars().next().unwrap().to_string()
}

