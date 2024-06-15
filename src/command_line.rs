#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::str::FromStr;

const SHORT_OPTION: &str = "-";
const LONG_OPTION: &str = "--";
const SHORT_HELP: &str = "-h";
const LONG_HELP: &str = "--help";
const TRUE: &str = "true";
const FALSE: &str = "false";

/// Defines the valid options for this program
struct OptionDef {
  /// The aliases for this option. e.g. -f --filename
  aliases:Vec<&'static str>,
  /// The name for the value associated with the option. e.g. -f path. If None,
  /// this option will be treated as a flag and the default value will be false.
  value_name:Option<&'static str>,
  /// An Option containing the value to use if one is not supplied. If `None`,
  /// then this option will be considered required and will panic if the program tries to use it.
  default_value:Option<&'static str>,
  /// The description of this option. e.g. The file to be read.
  description:&'static str,
}

impl OptionDef {
  /// Creates a new OptionDef
  ///
  /// # Arguments
  ///
  /// * `aliases` - The aliases for this option. e.g. -f --filename
  /// * `value_name` - The name for the value associated with the option. e.g. -f path. If None,
  /// this option will be treated as a flag and the default value will be false.
  /// * `default_value` - An Option<T> containing the value to use if one is not supplied. If `None`,
  /// then this option will be considered required and will panic if the program tries to use it.
  /// * `description` - The description of this option. e.g. The file to be read.
  ///
  /// # Panics
  ///
  /// * Panics if the alias does not start with '-' or '--'.
  /// * Panics if the alias starts with '--' and the length is less than 4
  /// * Panics if the alias starts with '-' and the length is not equal to 2
  ///
  #[inline]
  fn new(aliases:Vec<&'static str>, value_name:Option<&'static str>, default_value:Option<&'static str>, description:&'static str) -> Self {
    Self::validate_aliases(&aliases);
    OptionDef {
      description,
      aliases,
      value_name,
      default_value
    }
  }

  fn validate_aliases(aliases:&Vec<&'static str>) {
    for alias in aliases {
      let option_len = alias.trim_start_matches(SHORT_OPTION).len();
      if alias.starts_with(LONG_OPTION) {
        if option_len < 2 || alias.len()-option_len>2 {
          panic!("Invalid option name '{alias}'. Long option names must start with '--' and be greater than 1 character. e.g. --lo");
        }
      } else if alias.starts_with(SHORT_OPTION) {
        if option_len==0 || option_len>1
        {
          panic!("Invalid option name '{alias}'. Short option names must start with '-' and be 1 character. e.g. -f");
        }
      } else {
        panic!("Invalid option name '{alias}'. Options must start with '-' or '--'");
      };
    }
  }
}

/// Defines the valid commandline options and arguments for this program
pub struct CommandLineDef {
  /// The list of option_defs added
  option_defs: Vec<OptionDef>,
  /// Maps the individual aliases of the OptionDef to the OptionDef.
  option_def_map:HashMap<&'static str, usize>,
  /// Descriptive names for each of the arguments. e.g. file_path
  argument_names:Vec<&'static str>,
}

impl CommandLineDef {
  /// Creates a new CommandLineDef
  ///
  /// # Examples
  ///
  /// ```
  /// use cl_parse::command_line::CommandLineDef;
  /// let cl_def = CommandLineDef::new();
  /// ```
  #[inline]
  pub fn new() -> Self {
    CommandLineDef {
      option_defs:Vec::default(),
      option_def_map:HashMap::default(),
      argument_names:Vec::default(),
    }
  }

  /// A convenience function for adding flag options.
  ///
  /// # Arguments
  ///
  /// * `aliases` - The aliases for this option. e.g. `"-n","--negative"`
  /// * `description` - The description of this option. e.g. `A negative number`.
  ///
  /// # Panics
  ///
  /// * Panics if the alias does not start with '-' or '--'.
  /// * Panics if the alias starts with '--' and the length is less than 4
  /// * Panics if the alias starts with '-' and the length is not equal to 2
  ///
  /// # Examples
  ///
  /// ```
  /// use cl_parse::command_line::CommandLineDef;
  /// let args=vec![
  ///   "program".to_string(),
  ///   "-f".to_string(),
  /// ];
  /// let cl = CommandLineDef::new()
  ///   .add_flag(vec!["-f","--flag"], "A flag")
  ///   .add_option(
  ///     vec!["-b","--bool"],
  ///     None,
  ///     Some("true"),
  ///     "An ignored bool default which will be false if not specified"
  ///   )
  ///   .parse(args.into_iter());
  ///
  /// assert_eq!(cl.program_name(), "program");
  ///
  /// let f:bool = cl.option("-f");
  /// assert_eq!(f, true);
  ///
  /// let flag = cl.option("--flag");
  /// assert_eq!(flag, true);
  ///
  /// let b:bool = cl.option("-b");
  /// assert_eq!(b, false);
  ///
  /// let boolean:bool = cl.option("--bool");
  /// assert_eq!(boolean, false);
  /// ```
  #[inline]
  pub fn add_flag(&mut self, aliases:Vec<&'static str>, description:&'static str) -> &mut Self {
    self.add_option(aliases, None, None, description)
  }

  /// Adds a new option definition to this commandline definition
  ///
  /// # Arguments
  ///
  /// * `aliases` - The aliases for this option. e.g. `"-n","--negative"`
  /// * `value_name` - The `Option<&'static str>` name for the value associated with the option.
  /// If set to `None`, this option will be treated as a flag, and its value will default to "false".
  /// * `default_value` - An `Option<T>` containing the value to use if one is not supplied. If `None`,
  /// then this option will be considered required and will panic if this option is not specified on
  /// the commandline. If `value_name`==`None`, this parameter will be ignored.
  /// * `description` - The description of this option. e.g. `A negative number`.
  ///
  /// # Panics
  ///
  /// * Panics if the alias does not start with '-' or '--'.
  /// * Panics if the alias starts with '--' and the length is less than 4
  /// * Panics if the alias starts with '-' and the length is not equal to 2
  ///
  /// # Examples
  ///
  /// ```
  /// use cl_parse::command_line::CommandLineDef;
  /// let args=vec![
  ///   "program".to_string(),
  ///   "-n".to_string(), "-1".to_string(),
  ///   "-p".to_string(), "1".to_string(),
  /// ];
  /// let cl = CommandLineDef::new()
  ///   .add_option(vec!["-n","--negative"], Some("neg"), None, "A negative value")
  ///   .add_option(vec!["-p","--positive"], Some("pos"), None, "A positive value")
  ///   .parse(args.into_iter());
  ///
  /// assert_eq!(cl.program_name(), "program");
  ///
  /// let n:i16 = cl.option("-n");
  /// assert_eq!(n, -1);
  ///
  /// let neg:i16 = cl.option("--negative");
  /// assert_eq!(neg, n);
  ///
  /// let p:i16 = cl.option("-p");
  /// assert_eq!(p, 1);
  ///
  /// let pos:i16 = cl.option("--positive");
  /// assert_eq!(pos, p);
  /// ```
  #[inline]
  pub fn add_option(&mut self, aliases:Vec<&'static str>, value_name:Option<&'static str>, default_value:Option<&'static str>, description:&'static str) -> &mut Self {
    let default = if let Some(_) = value_name { default_value } else { Some(FALSE) };
    self.option_defs.push(OptionDef::new(aliases, value_name, default, description));
    let od_idx = self.option_defs.len()-1;
    for alias in &self.option_defs[od_idx].aliases {
      if self.option_def_map.insert(alias, od_idx).is_some() {
        panic!("Option {alias} cannot be redefined")
      }
    }
    self
  }

  /// Add a new argument definition to the commandline definition
  ///
  /// # Arguments
  ///
  /// * `argument_name` - The name of this argument. To be used in the usage message.
  ///
  /// # Examples
  ///
  /// ```
  /// use cl_parse::command_line::CommandLineDef;
  /// let args=vec![
  ///   "program".to_string(),
  ///   "arg1".to_string(),
  ///   "--bool".to_string(),
  ///   "arg2".to_string(),
  ///   "-n".to_string(), "-1".to_string(),
  ///   "arg3".to_string(),
  /// ];
  /// let cl = CommandLineDef::new()
  /// .add_option(vec!["-b","--bool"], None, Some("false"), "A boolean value")
  /// .add_option(vec!["-n","--num"], Some("num"), None, "A numeric value")
  /// .parse(args.into_iter());
  /// assert_eq!(cl.program_name(), "program");
  ///
  /// let b:bool = cl.option("-b");
  /// assert_eq!(b, true);
  ///
  /// let n:i16 = cl.option("-n");
  /// assert_eq!(n, -1);
  ///
  /// assert_eq!(cl.arguments(), 3);
  /// assert_eq!(cl.argument(0), "arg1");
  /// assert_eq!(cl.argument(1), "arg2");
  /// assert_eq!(cl.argument(2), "arg3");
  /// ```
  #[inline]
  pub fn add_argument(&mut self, argument_name:&'static str) -> &mut Self {
    self.argument_names.push(argument_name);
    self
  }

  /// Creates a new CommandLine from this CommandLineDef and the args
  ///
  /// # Arguments
  ///
  /// * `args` - A string iterator that holds the commandline arguments to be parsed
  ///
  /// # Examples
  ///
  /// ```
  ///  use std::collections::VecDeque;
  ///  use std::env;
  ///  use cl_parse::command_line::{CommandLine, CommandLineDef};
  ///  let cl = CommandLineDef::new().add_option(vec!["-f","--filename"], Some("filepath"),
  ///      None, "The file to be parsed").parse(env::args());
  ///
  ///   // Test Program Name
  ///   assert_eq!(false, cl.program_name().is_empty());
  /// ```
  pub fn parse(&self, args: impl Iterator<Item=String>) -> CommandLine {
    let mut options:HashMap<String, String> = HashMap::default();
    let mut arguments:Vec<String> = Vec::default();

    // make the iterator peekable so we can see the next one
    let mut peekable_args = args.peekable();

    let program_name = peekable_args.next().unwrap_or_else(String::default);
    let usage = self.usage(&program_name);
    let help = self.help();
    let mut skip_next = false;

    while let Some(arg) = peekable_args.next() {
      if arg == SHORT_HELP || arg == LONG_HELP {
        print!("{}", help);
        std::process::exit(0);
      }
      if !skip_next {
        skip_next = if arg.starts_with(SHORT_OPTION) {
          self.parse_option(arg, peekable_args.peek(), &usage, &mut options)
        } else {
            arguments.push(arg);
            false
        }
      } else {
        skip_next = false;
      }
    }
    self.add_default_options(&mut options);
    CommandLine::new(program_name, options, arguments)
  }

  #[inline]
  fn usage(&self, program_name:&str) -> String {
    let mut usage = self.option_defs.iter().map(|od| {
      let mut option = od.aliases[0].to_string();
      if od.value_name.is_some(){
        option.push(' ');
        option.push_str(od.value_name.unwrap());
      }
      option
    }).fold(program_name.to_string(),|accum, alias|  accum+" "+&alias);

    let arguments = self.argument_names.iter().fold(String::default(),|accum, s|  accum+" "+s);
    usage.push(' ');
    usage.push_str(&arguments);
    usage
  }

  #[inline]
  fn help(&self) -> String {
    let mut usage:String = String::default();
    usage.push(' ');

    let options = self.option_defs.iter().map(|od| {
      let mut option = od.aliases[0].to_string();
      if od.value_name.is_some(){
        option.push(' ');
        option.push_str(od.value_name.unwrap());
      }
      option
    }).fold(String::default(),|accum, alias|  accum+" "+&alias);
    usage.push_str(&options);
    usage.push(' ');

    let arguments = self.argument_names.iter().fold(String::default(),|accum, s|  accum+" "+s);
    usage.push_str(&arguments);
    usage
  }

  #[inline]
  fn invalid_option(option: &str) {
    panic!("Option {option} not defined");
  }

  #[inline]
  fn required_option(option: &str) {
    panic!("Option {option} is required");
  }

  #[inline]
  fn invalid_value(option: &str) {
    panic!("A value is required for option {option}");
  }

  #[inline]
  fn find_option_def(&self, option:&str) -> Option<&OptionDef> {
    match self.option_def_map.get(option) {
      None => None,
      Some(od_idx) => Some(&self.option_defs[*od_idx])
    }
  }

  #[inline]
  fn add_default_options(&self, options: &mut HashMap<String, String>){
    for option in self.option_def_map.keys() {
      if !options.contains_key(*option) {
        if let Some(od) = self.find_option_def(&option) {
          if let Some(default) = od.default_value {
            options.insert(option.to_string(), default.to_string());
          } else {
            Self::required_option(option);
          }
        }
      }
    }
  }

  #[inline]
  fn parse_option(&self, option: String, value: Option<&String>, usage: &str, options: &mut HashMap<String, String>) -> bool {
    let mut skip = false;

    if let Some(option_def) = self.find_option_def(&option) {
      let val = if option_def.value_name.is_none() {
        TRUE
      } else {
        if value.is_none() {
          Self::invalid_value(&option);
        }
        skip=true;
        value.unwrap()
      };
      for alias in &option_def.aliases {
        options.insert(alias.to_string(), val.to_string());
      }
    } else {
      Self::invalid_option(&option);
    }
    skip
  }

}

/// Stores the parsed command line
pub struct CommandLine {
  /// Commandline argument 0 the program name
  program_name: String,
  /// The options and values parsed from the command line
  options: HashMap<String, String>,
  /// The remaining non-option arguments
  arguments: Vec<String>,
}

impl CommandLine {
  /// Creates a new CommandLine from the args and the OptionDefs
  ///
  /// # Arguments
  ///
  /// * `program_name` - The program name used on the commandline
  /// * `option` - A hashmap of options specified on the commandline
  /// * `args` - A vector of arguments specified on the commandline
  ///
  /// # Examples
  ///
  /// ```
  ///  use std::collections::VecDeque;
  ///  use std::env;
  ///  use cl_parse::command_line::{CommandLine, CommandLineDef};
  ///  let cl = CommandLineDef::new().add_option(vec!["-f","--filename"], Some("filepath"),
  ///      None, "The file to be parsed").parse(env::args());
  ///   // Test Program Name
  ///   assert_eq!(false, cl.program_name().is_empty());
  /// ```
  #[inline]
  fn new(program_name: String, options: HashMap<String, String>, arguments: Vec<String>) -> Self {
    CommandLine {
      program_name,
      options,
      arguments
    }
  }

  /// Returns the number of options parsed
  ///
  /// # Examples
  ///
  /// ```
  ///  use std::collections::VecDeque;
  ///  use std::env;
  ///  use cl_parse::command_line::{CommandLine, CommandLineDef};
  ///  let cl = CommandLineDef::new().add_option(vec!["-f","--filename"], Some("filepath"),
  ///      None, "The file to be parsed").parse(env::args());
  ///
  ///   // Test Program Name
  ///   assert_eq!(cl.options(), 2);
  /// ```
  #[inline]
  pub fn options(self) -> usize {
    self.options.len()
  }

  /// Returns the option for the option key specified
  ///
  /// # Arguments
  ///
  /// * `name` - A string slice that holds the name of the option
  ///
  /// # Examples
  ///
  /// ```
  ///  use std::collections::VecDeque;
  ///  use std::env;
  ///  use cl_parse::command_line::{CommandLine, CommandLineDef};
  ///  let cl = CommandLineDef::new().add_option(vec!["-f","--filename"], Some("filepath"),
  ///      None, "The file to be parsed").parse(env::args());
  ///
  ///   // Test Program Name
  ///   assert_eq!(cl.option("-f"), "/users/dberry/testfile");
  /// ```

  #[inline]
  pub fn option<T>(&self, name:&str) -> T
  where T: FromStr {
    match self.options.get(name) {
      Some(option) => {
        match T::from_str(option) {
          Ok(t) => t,
          Err(_) => panic!("Cannot convert '{name}' from '{option}'")
        }
      }
      None => panic!("Option '{name}' not found")
    }
  }

  /// Returns the number of arguments parsed
  ///
  /// # Examples
  ///
  /// ```
  ///  use std::collections::VecDeque;
  ///  use std::env;
  ///  use cl_parse::command_line::{CommandLine, CommandLineDef};
  ///  let cl = CommandLineDef::new().add_option(vec!["-f","--filename"], Some("filepath"),
  ///      None, "The file to be parsed").parse(env::args());
  ///
  ///   // Test Program Name
  ///   assert_eq!(cl.arguments(), 0);
  /// ```
  #[inline]
  pub fn arguments(&self) -> usize {
    self.arguments.len()
  }

  /// Returns the argument based by index
  ///
  /// # Examples
  ///
  /// ```
  ///  use std::collections::VecDeque;
  ///  use std::env;
  ///  use cl_parse::command_line::{CommandLine, CommandLineDef};
  ///  let cl = CommandLineDef::new().add_option(vec!["-f","--filename"], Some("filepath"),
  ///      None, "The file to be parsed").parse(env::args());
  ///
  ///   // Test Program Name
  ///   assert_eq!(cl.argument(0), "arg1");
  /// ```
  #[inline]
  pub fn argument(&self, index:usize) -> &str {
    &self.arguments[index]
  }

  /// Returns the program name specified on the command line
  ///
  /// # Examples
  ///
  /// ```
  ///  use std::collections::VecDeque;
  ///  use std::env;
  ///  use cl_parse::command_line::{CommandLine, CommandLineDef};
  ///  let cl = CommandLineDef::new().add_option(vec!["-f","--filename"], Some("filepath"),
  ///      None, "The file to be parsed").parse(env::args());
  ///
  ///   // Test Program Name
  ///   assert_eq!(false, cl.program_name().is_empty());
  /// ```
  #[inline]
  pub fn program_name(&self) -> &str {
    &self.program_name
  }
}
