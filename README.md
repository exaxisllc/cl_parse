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
 - unordered options and arguments
 - retrieving the option or argument in the target type. e.g. i32, String, etc.

# Example

```
use std::env;
use cl_parse::{CommandLine, CommandLineDef};

let cl = CommandLineDef::new()
.add_flag(vec!["-b","--boolean"], "A boolean value")
.add_flag(vec!["-f","--faux"], "Another boolean value")
.add_option(vec!["-n","--num"], Some("num"), None, "A required numeric value")
.add_argument("arg-0")
.add_argument("arg-1")
.add_argument("arg-2")
.parse(env::args());

// program arg1 --boolean arg2 -n -1 arg3
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

assert_eq!(cl.arguments(), 3);

let arg0:String = cl.argument(0);
assert_eq!(arg0, "arg1");

let arg1:String = cl.argument(1);
assert_eq!(arg1, "arg2");

let arg2:String = cl.argument(2);
assert_eq!(arg2, "arg3");
```