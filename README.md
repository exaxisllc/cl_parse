# Command Line Parse

[![CI](https://github.com/exaxisllc/cl_parse/actions/workflows/test.yml/badge.svg)](https://github.com/exaxisllc/cl_parse/actions/workflows/test.yml)

`cl_parse` is a library that allows you to define commandline options and arguments and then
parse the commandline arguments based on that definition.

# Motivation

`cl_parse` was developed to allow the most common commandline options that are used in
modern commandline utilities. It was also designed for ease of use. The following are the features
implemented in cl_parse.

 - option aliases. e.g. -f, --file
 - options with negative values. e.g. --increment -1
 - flag concatenation. i.e. -xvgf  is equivalent to -x -v -g -f
 - Auto usage message generation
 - Auto help message generation
 - -h, --help output provided by default
 - missing value detection for options
 - ability to define required options
 - option and argument validation. i.e. only defined options and arguments can be used
 - option value validation. e.g. --level vec!["low","med","high"]
 - unordered options and arguments
 - retrieving the option or argument in the target type. e.g. i32, String, etc.
 - arguments that start with - or -- can be enclosed in single or double quotes. e.g. '-1' or "-5"

# Example

```
use std::env;
use cl_parse::CommandLineDef;

let cl = CommandLineDef::new()
.add_flag(vec!["-b","--boolean"], "A boolean value")
.add_flag(vec!["-f","--faux"], "Another boolean value")
.add_option(vec!["-n","--num"], Some("num"), None, "A required numeric value")
.add_option_with_values(vec!["--level"], Some("level"), Some("med"), "Operating Speed", vec!["low", "med", "high"])
.add_argument("arg1_name") // define the argument names
.add_argument("arg2_name")
.add_argument("arg3_name")
.parse(env::args());

// The asserts assume the following commandline
// program arg1_value --boolean arg2_value -n -1 arg3_value --level low
assert_eq!(cl.program_name(), "program");

// aliases are updated
let b:bool = cl.option("-b");
assert_eq!(b, true);

let boolean:bool = cl.option("--boolean");
assert_eq!(boolean, true);

// flags default to false
let f:bool = cl.option("-f");
assert_eq!(f, false);

// all aliased are updated
let faux:bool = cl.option("--faux");
assert_eq!(faux,false);

let n:i16 = cl.option("-n");
assert_eq!(n, -1);

let level:String = cl.option("--level");
assert_eq!(level, "low");

assert_eq!(cl.arguments(), 3);

let arg1:String = cl.argument("arg1_name"); // retrieve the argument value by the argument name
assert_eq!(arg1, "arg1_value");

let arg2:String = cl.argument("arg2_name");
assert_eq!(arg2, "arg2_value");

let arg3:String = cl.argument("arg3_name");
assert_eq!(arg3, "arg3_value");
```