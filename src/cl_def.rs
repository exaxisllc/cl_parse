use std::collections::HashMap;
use crate::text::T;
use super::option_def::OptionDef;
use super::{FALSE, format_usage, LONG_HELP, LONG_OPTION, panic_msg, SHORT_HELP, SHORT_OPTION, TRUE};
use super::command_line::CommandLine;

/// Defines the valid commandline options and arguments for this program
pub struct CommandLineDef {
  /// The list of option_defs added
  pub(crate) option_defs: Vec<OptionDef>,
  /// Maps the individual aliases of the OptionDef to the OptionDef.
  pub(crate) option_def_map:HashMap<&'static str, usize>,
  /// Descriptive names for each of the arguments. e.g. file_path
  pub(crate) argument_names:Vec<&'static str>,
}

impl CommandLineDef {
  /// Creates a new CommandLineDef
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
  /// use cl_parse::CommandLineDef;
  /// // Simulate env::args()
  /// let env_args=vec![
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
  ///   .parse(env_args.into_iter());
  ///
  /// assert_eq!(cl.program_name(), "program");
  ///
  /// let f:bool = cl.option("-f");
  /// assert_eq!(f, true);
  ///
  /// let flag:bool = cl.option("--flag");
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
  /// the commandline. If `value_name`==`None`, `default_value` will be ignored.
  /// * `description` - The description of this option. e.g. `A negative number`.
  ///
  /// # Panics
  ///
  /// * Panics if the alias does not start with '-' or '--'.
  /// * Panics if the alias starts with '--' and the length is less than 4
  /// * Panics if the alias starts with '-' and the length is not equal to 2
  /// * Panics if an alias is defined more than once
  ///
  /// # Examples
  ///
  /// ```
  /// use cl_parse::CommandLineDef;
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
  pub fn add_option(&mut self, mut aliases:Vec<&'static str>, value_name:Option<&'static str>, default_value:Option<&'static str>, description:&'static str) -> &mut Self {
    let default = if value_name.is_some() { default_value } else { Some(FALSE) };
    aliases.sort_by(|a,b| a.trim_start_matches(SHORT_OPTION).cmp(b.trim_start_matches(SHORT_OPTION)));
    self.option_defs.push(OptionDef::new(aliases, value_name, default, description));
    let od_idx = self.option_defs.len()-1;
    for alias in &self.option_defs[od_idx].aliases {
      if self.option_def_map.insert(alias, od_idx).is_some() {
        panic_msg(T.option_redefined(alias));
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
  /// use cl_parse::CommandLineDef;
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
  /// .add_argument("arg-0")
  /// .add_argument("arg-1")
  /// .add_argument("arg-2")
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
  ///
  /// let arg0:String = cl.argument(0);
  /// assert_eq!(arg0, "arg1");
  ///
  /// let arg1:String = cl.argument(1);
  /// assert_eq!(arg1, "arg2");
  ///
  /// let arg2:String = cl.argument(2);
  /// assert_eq!(arg2, "arg3");
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
  /// * Panics if an option is specified and its value is missing
  /// * Panics if an undefined option is present on the commandline
  /// * Panics if a required option is not present on the commandline
  /// * Panics if number of arguments is incorrect
  ///
  /// # Examples
  ///
  /// ```
  ///  use std::collections::VecDeque;
  ///  use std::env;
  ///  // Simulate env::args()
  ///  let env_args=vec![String::from("program"), String::from("-f"), String::from("/file/path")];
  ///  use cl_parse::{CommandLine, CommandLineDef};
  ///  let cl = CommandLineDef::new().add_option(vec!["-f","--filename"], Some("filepath"),
  ///      None, "The file to be parsed").parse(env_args.into_iter());
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
        panic!("{}", help);
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
    // make sure we got the defined number of arguments
    if arguments.len() != self.argument_names.len() {
      panic_msg(format_usage(
        &T.argument_defined_ne_found(self.argument_names.len(), arguments.len()),
        &usage));
    }
    self.add_default_options(&mut options, &usage);
    CommandLine::new(program_name, options, arguments)
  }

  #[inline]
  fn usage(&self, program_name:&str) -> String {
    let mut flags: Vec<char> = Vec::default();
    let mut options: Vec<String> = Vec::default();

    for od in &self.option_defs {
      if od.value_name.is_some() {
        options.push(format!("{} <{}>",od.aliases[0],od.value_name.unwrap()))
      } else if od.aliases[0].starts_with(LONG_OPTION) {
        options.push(format!("{}",od.aliases[0]))
      } else {
        flags.push(od.aliases[0].chars().last().unwrap())
      }
    }

    let mut usage = T.usage(program_name);

    if !flags.is_empty() {
      flags.sort();
      usage.push_str(&format!(" -{}", flags.iter().fold(String::default(),|acc, c |{acc + &c.to_string()})));
    }

    if !options.is_empty() {
      options.sort_by(|a,b| a.trim_start_matches(SHORT_OPTION).cmp(b.trim_start_matches(SHORT_OPTION)));
      usage.push_str(&format!(" {}", options.join(" ").to_string()));
    }

    if !self.argument_names.is_empty() {
      usage.push_str(&format!(" <{}>", self.argument_names.join("> <").to_string()));
    }

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
  fn find_option_def(&self, option:&str) -> Option<&OptionDef> {
    let od_idx = self.option_def_map.get(option)?;
    Some(&self.option_defs[*od_idx])
  }

  #[inline]
  fn add_default_options(&self, options: &mut HashMap<String, String>, usage: &str, ){
    for option in self.option_def_map.keys() {
      if !options.contains_key(*option) {
        if let Some(od) = self.find_option_def(&option) {
          let default = od.default_value.expect(&format_usage(&T.option_required(option), usage));
          options.insert(option.to_string(), default.to_string());
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
          panic_msg(format_usage(&T.option_value_required(&option), usage));
        }
        skip=true;
        value.unwrap()
      };
      for alias in &option_def.aliases {
        if options.insert(alias.to_string(), val.to_string()).is_some() {
          panic_msg(format_usage(&T.option_multiple_found(alias), usage));
        }
      }
    } else if !option.starts_with(LONG_OPTION) && option.starts_with(SHORT_OPTION){
      let flags = option.trim_start_matches(SHORT_OPTION);
      for f in flags.chars() {
        let flag = format!("-{f}");
        let flag_def = self.find_option_def(&flag).expect(&format_usage(&T.option_not_defined(&flag), usage));
          if flag_def.value_name.is_none() {
            if options.insert(flag, TRUE.to_string()).is_some() {
              panic_msg(format_usage(&T.option_multiple_flags(f),usage));
            }
          } else {
            panic_msg(format_usage(&T.option_invalid_flag(&flag),usage));
          }
      }
    } else {
      panic_msg(format_usage(&T.option_not_defined(&option), usage));
    }
    skip
  }
}

