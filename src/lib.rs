//! # Command Line Parse
//!
//! `cl_parse` is a library that allows you to define commandline options and arguments and then
//! parse the commandline arguments based on that definition.
//!
//! # Motivation
//!
//! `cl_parse` was developed to allow the most common commandline options that are used in
//! modern commandline utilities. It was also designed for ease of use. The following are the features
//! implemented in cl_parse.
//!
//!  - option aliases. e.g. -f, --file
//!  - options with negative values. e.g. --increment -1
//!  - flag concatenation. i.e. -xvgf  is equivalent to -x -v -g -f
//!  - Auto usage message generation
//!  - Auto help message generation
//!  - -h, --help output provided by default
//!  - missing value detection for options
//!  - ability to define required options
//!  - option and argument validation. i.e. only defined options and arguments can be used
//!  - unordered options and arguments
//!  - retrieving the option or argument in the target type. e.g. i32, String, etc.
//!
//! # Examples
//!
#![deny(missing_docs)]

const SHORT_OPTION: &'static str = "-";
const LONG_OPTION: &'static str = "--";
const SHORT_HELP: &'static str = "-h";
const LONG_HELP: &'static str = "--help";
const TRUE: &'static str = "true";
const FALSE: &'static str = "false";

/// # Option Def
///
/// `option_def` is used by cl_def to define options
mod option_def;

/// # Command Line Def
///
/// `cl_def` is used to define and parse commandline options and arguments
mod cl_def;

/// # Command Line
///
/// `command_line` is a collection of utilities for processing commandline arguments
mod command_line;

pub use cl_def::CommandLineDef;
pub use command_line::CommandLine;

mod text;

fn format_usage(msg: &str, usage: &str) -> String {
  format!("{}\n{}", msg, usage)
}

fn panic_msg(msg: String) {
  panic!("{}",msg)
}

