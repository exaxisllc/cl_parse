use crate::text::T;
use std::collections::HashMap;
use std::str::FromStr;

/// Stores the parsed command line
pub struct CommandLine {
    /// Commandline argument 0 the program name
    program_name: String,
    /// The options and values parsed from the command line
    options: HashMap<String, String>,
    /// The remaining non-option arguments
    argument_map: HashMap<String, String>,
}

impl CommandLine {
    /// Creates a new CommandLine from the args and the OptionDefs
    ///
    /// # Arguments
    ///
    /// * `program_name` - The program name used on the commandline
    /// * `options` - A hashmap of options and their values specified on the commandline
    /// * `argument_map` - A HashMap of arguments and their values specified on the commandline
    ///
    /// # Examples
    ///
    /// ```
    ///  use std::collections::VecDeque;
    ///  use std::env;
    ///  use cl_parse::{CommandLine, CommandLineDef};
    ///  // Simulate env::args()
    ///  let env_args=Vec::new();
    ///  let cl = CommandLineDef::new().parse(env_args.into_iter());
    ///   // Test Program Name
    ///   assert_eq!(true, cl.program_name().is_empty());
    /// ```
    pub(crate) fn new(
        program_name: String,
        options: HashMap<String, String>,
        argument_map: HashMap<String, String>,
    ) -> Self {
        CommandLine {
            program_name,
            options,
            argument_map,
        }
    }

    /// Returns the number of options parsed
    ///
    /// # Examples
    ///
    /// ```
    ///  use std::collections::VecDeque;
    ///  use std::env;
    ///  use cl_parse::{CommandLine, CommandLineDef};
    ///  // Simulate env::args()
    ///  let env_args=vec![String::from("program"), String::from("-f"), String::from("/file/path"), String::from("arg1")];
    ///  let cl = CommandLineDef::new()
    ///   .add_option(vec!["-f","--filename"], Some("filepath"), None, "The file to be parsed")
    ///   .add_flag(vec!["-b"], "A binary flag option")
    ///   .add_argument("arg-0")
    ///   .parse(env_args.into_iter());
    ///
    ///   // Test Program Name
    ///   assert_eq!(cl.options(), 5); // -f, --filename, -b, -h, --help
    /// ```
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
    ///  use cl_parse::{CommandLine, CommandLineDef};
    ///  // Simulate env::args()
    ///  let env_args=vec![String::from("program"), String::from("-f"), String::from("/file/path")];
    ///  let cl = CommandLineDef::new().add_option(vec!["-f","--filename"], Some("filepath"),
    ///      None, "The file to be parsed").parse(env_args.into_iter());
    ///  let filename:String = cl.option("-f");
    ///   // Test Program Name
    ///   assert_eq!(filename, "/file/path".to_string());
    /// ```
    pub fn option<T>(&self, name: &str) -> T
    where
        T: FromStr,
    {
        let option = self
            .options
            .get(name)
            .unwrap_or_else(|| panic!("{}", &T.option_not_found(name)));
        match T::from_str(option) {
            Ok(t) => t,
            Err(_) => panic!("{}", T.option_cannot_convert(name, option, std::any::type_name::<T>())),
        }
    }

    /// Returns the number of arguments parsed
    ///
    /// # Examples
    ///
    /// ```
    ///  use std::collections::VecDeque;
    ///  use std::env;
    ///  use cl_parse::{CommandLine, CommandLineDef};
    ///  // Simulate env::args()
    ///  let env_args=vec![String::from("program"), String::from("-f"), String::from("/file/path")];
    ///  let cl = CommandLineDef::new().add_option(vec!["-f","--filename"], Some("filepath"),
    ///      None, "The file to be parsed").parse(env_args.into_iter());
    ///
    ///   // Test Program Name
    ///   assert_eq!(cl.arguments(), 0);
    /// ```
    pub fn arguments(&self) -> usize {
        self.argument_map.len()
    }

    /// Returns the argument based by index
    ///
    /// # Examples
    ///
    /// ```
    ///  use std::collections::VecDeque;
    ///  use std::env;
    ///  use cl_parse::{CommandLine, CommandLineDef};
    ///  // Simulate env::args()
    ///  let env_args=vec![String::from("program"), String::from("-f"), String::from("/file/path"), String::from("arg1_value")];
    ///  let cl = CommandLineDef::new()
    ///   .add_option(vec!["-f","--filename"], Some("filepath"), None, "The file to be parsed")
    ///   .add_argument("arg1_name")
    ///   .parse(env_args.into_iter());
    ///
    ///  let arg1:String = cl.argument("arg1_name");
    ///  assert_eq!(arg1, "arg1_value");
    ///
    /// ```
    pub fn argument<T>(&self, name : &str) -> T
    where
        T: FromStr,
    {
        let argument = self
            .argument_map
            .get(name)
            .unwrap_or_else(|| panic!("{}", &T.argument_invalid_name(name)));
        match T::from_str(argument) {
            Ok(t) => t,
            Err(_) => panic!("{}", T.argument_cannot_convert(name, argument, std::any::type_name::<T>())),
        }
    }

    /// Returns the program name specified on the command line
    ///
    /// # Examples
    ///
    /// ```
    ///  use std::collections::VecDeque;
    ///  use std::env;
    ///  use cl_parse::{CommandLine, CommandLineDef};
    ///  // Simulate env::args()
    ///  let env_args=vec![String::from("program"), String::from("-f"), String::from("/file/path")];
    ///  let cl = CommandLineDef::new().add_option(vec!["-f","--filename"], Some("filepath"),
    ///      None, "The file to be parsed").parse(env_args.into_iter());
    ///
    ///   // Test Program Name
    ///   assert_eq!(false, cl.program_name().is_empty());
    /// ```
    pub fn program_name(&self) -> &str {
        &self.program_name
    }
}
