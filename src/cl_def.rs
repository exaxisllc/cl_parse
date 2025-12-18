use super::command_line::CommandLine;
use crate::text::T;
use std::cmp::max;
use std::collections::HashMap;

const SHORT_OPTION: &str = "-";
const LONG_OPTION: &str = "--";
const SHORT_HELP: &str = "-h";
const LONG_HELP: &str = "--help";
const TRUE: &str = "true";
const FALSE: &str = "false";

fn format_usage(msg: &str, usage: &str) -> String {
    format!("{msg}\n{usage}")
}

fn panic_msg(msg: String) {
    panic!("{}", msg)
}

/// Defines the valid commandline options and arguments for this program
#[derive(Default)]
pub struct CommandLineDef {
    /// The list of option_defs added
    option_defs: Vec<OptionDef>,
    /// Maps the individual aliases of the OptionDef to the OptionDef.
    option_def_map: HashMap<&'static str, usize>,
    /// Descriptive names for each of the arguments, e.g., `file_path`
    argument_names: Vec<&'static str>,
}

impl CommandLineDef {
    /// Creates a new CommandLineDef
    pub fn new() -> Self {
        let mut cl_def = CommandLineDef {
            option_defs: Vec::default(),
            option_def_map: HashMap::default(),
            argument_names: Vec::default(),
        };
        cl_def.add_option(vec!["-h", "--help"], None, None, "Display usage message");
        cl_def
    }

    /// A convenience function for adding flag options.
    ///
    /// # Arguments
    ///
    /// * `aliases` - The aliases for this option, e.g., `vec!["-n", "--negative"]`
    /// * `description` - The description of this option, e.g., `A negative number`
    ///
    /// # Panics
    ///
    /// * Panics if the alias does not start with `-` or `--`.
    /// * Panics if the alias starts with `--` and the length is less than 4.
    /// * Panics if the alias starts with `-` and the length is not equal to 2.
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
    pub fn add_flag(&mut self, aliases: Vec<&'static str>, description: &'static str) -> &mut Self {
        self.add_option(aliases, None, None, description)
    }

    /// Adds a new option definition to this commandline definition
    ///
    /// # Arguments
    ///
    /// * `aliases` - The aliases for this option, e.g., `vec!["-n", "--negative"]`
    /// * `value_name` - The `Option<&'static str>` name for the value associated with the option.
    ///   If set to `None`, this option will be treated as a flag, and its value will default to `false`.
    /// * `default_value` - An `Option<T>` containing the value to use if one is not supplied. If `None`,
    ///   then this option will be considered required and will panic if this option is not specified on
    ///   the commandline. If `value_name` == `None`, `default_value` will be ignored.
    /// * `description` - The description of this option, e.g., `A negative number`
    ///
    /// # Panics
    ///
    /// * Panics if the alias does not start with `-` or `--`.
    /// * Panics if the alias starts with `--` and the length is less than 4.
    /// * Panics if the alias starts with `-` and the length is not equal to 2.
    /// * Panics if an alias is defined more than once.
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
    pub fn add_option(
        &mut self,
        aliases: Vec<&'static str>,
        value_name: Option<&'static str>,
        default_value: Option<&'static str>,
        description: &'static str,
    ) -> &mut Self {
        self.add_option_with_values(aliases, value_name, default_value, description, Vec::new())
    }

    /// Adds a new option definition to this commandline definition.
    ///
    /// # Arguments
    ///
    /// * `aliases` - The aliases for this option, e.g., `vec!["-n", "--negative"]`
    /// * `value_name` - The `Option<&'static str>` name for the value associated with the option.
    ///   If set to `None`, this option will be treated as a flag, and its value will default to `false`.
    /// * `default_value` - An `Option<T>` containing the value to use if one is not supplied. If `None`,
    ///   then this option will be considered required and will panic if this option is not specified on
    ///   the commandline. If `value_name` == `None`, `default_value` will be ignored.
    /// * `description` - The description of this option, e.g., `A negative number`
    /// * `valid_values` - a vector of valid values to validate the option against
    ///
    /// # Panics
    ///
    /// * Panics if the alias does not start with `-` or `--`.
    /// * Panics if the alias starts with `--` and the length is less than 4.
    /// * Panics if the alias starts with `-` and the length is not equal to 2.
    /// * Panics if an alias is defined more than once.
    ///
    /// # Examples
    ///
    /// ```
    /// use cl_parse::CommandLineDef;
    /// let args=vec![
    ///   "program".to_string(),
    ///   "--level".to_string(), "low".to_string()
    /// ];
    /// let cl = CommandLineDef::new()
    ///   .add_option_with_values(vec!["--level"], Some("level"), Some("med"), "Operating Speed", vec!["low", "med", "high"])
    ///   .parse(args.into_iter());
    ///
    /// assert_eq!(cl.program_name(), "program");
    ///
    /// let level:String = cl.option("--level");
    /// assert_eq!(level, "low");
    ///
    /// ```
    pub fn add_option_with_values(
        &mut self,
        mut aliases: Vec<&'static str>,
        value_name: Option<&'static str>,
        default_value: Option<&'static str>,
        description: &'static str,
        valid_values: Vec<&'static str>,
    ) -> &mut Self {
        let default = if value_name.is_some() {
            default_value
        } else {
            Some(FALSE)
        };
        aliases.sort_by(|a, b| {
            a.trim_start_matches(SHORT_OPTION)
                .cmp(b.trim_start_matches(SHORT_OPTION))
        });
        self.option_defs.push(OptionDef::new(
            aliases,
            value_name,
            default,
            description,
            valid_values,
        ));
        let od_idx = self.option_defs.len() - 1;
        for alias in &self.option_defs[od_idx].aliases {
            if self.option_def_map.insert(alias, od_idx).is_some() {
                panic_msg(T.option_redefined(alias));
            }
        }
        self
    }

    /// Add a new argument definition to the commandline definition.
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
    ///   "arg1_value".to_string(),
    ///   "--bool".to_string(),
    ///   "arg2_value".to_string(),
    ///   "-n".to_string(), "-1".to_string(),
    ///   "arg3_value".to_string(),
    /// ];
    /// let cl = CommandLineDef::new()
    /// .add_option(vec!["-b","--bool"], None, Some("false"), "A boolean value")
    /// .add_option(vec!["-n","--num"], Some("num"), None, "A numeric value")
    /// .add_argument("arg1_name")
    /// .add_argument("arg2_name")
    /// .add_argument("arg3_name")
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
    /// let arg1:String = cl.argument("arg1_name");
    /// assert_eq!(arg1, "arg1_value");
    ///
    /// let arg2:String = cl.argument("arg2_name");
    /// assert_eq!(arg2, "arg2_value");
    ///
    /// let arg3:String = cl.argument("arg3_name");
    /// assert_eq!(arg3, "arg3_value");
    /// ```
    pub fn add_argument(&mut self, argument_name: &'static str) -> &mut Self {
        self.argument_names.push(argument_name);
        self
    }

    /// Creates a new CommandLine from this CommandLineDef and the args.
    ///
    /// # Arguments
    ///
    /// * `args` - A string iterator that holds the commandline arguments to be parsed
    ///
    /// * Panics if an option is specified and its value is missing.
    /// * Panics if an undefined option is present on the commandline. 
    /// * Panics if a required option is not present on the commandline.
    /// * Panics if number of arguments is incorrect.
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
    pub fn parse(&self, args: impl Iterator<Item = String>) -> CommandLine {
        let mut options: HashMap<String, String> = HashMap::default();
        let mut arguments: Vec<String> = Vec::default();

        // make the iterator peekable so we can see the next one
        let mut peekable_args = args.peekable();

        let program_name = peekable_args.next().unwrap_or_default();
        let usage = self.usage(&program_name);
        let mut skip_next = false;

        while let Some(arg) = peekable_args.next() {
            if arg == SHORT_HELP || arg == LONG_HELP {
                panic!("{}", usage);
            }
            if !skip_next {
                skip_next = if arg.starts_with(SHORT_OPTION) {
                    self.parse_option(arg, peekable_args.peek(), &usage, &mut options)
                } else {
                    if arg.starts_with("'") {
                        arguments.push(arg.trim_matches('\'').to_string());
                    } else if arg.starts_with("\"")
                    {
                        arguments.push(arg.trim_matches('"').to_string());
                    } else {
                        arguments.push(arg);
                    }
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
                &usage,
            ));
        }
        let mut argument_map = HashMap::default();
        for (k,v) in std::iter::zip(&self.argument_names, arguments) {
            argument_map.insert(k.to_string(), v);
        }

        self.validate_options(&mut options, &usage);
        CommandLine::new(program_name, options, argument_map)
    }

    fn usage(&self, program_name: &str) -> String {
        let mut flags: Vec<char> = Vec::default();
        let mut options: Vec<String> = Vec::default();
        let mut requireds: Vec<String> = Vec::default();
        let mut help_lines: Vec<(String, String, String)> = Vec::default();
        let mut max_len = 0;

        for od in &self.option_defs {
            let mut help_options = od.aliases.join(", ");
            if od.value_name.is_some() {
                let value_name = od.value_name.unwrap();
                help_options = format!("{help_options} <{value_name}>");
                if od.default_value.is_none() {
                    requireds.push(format!("{} <{}>", od.aliases[0], value_name));
                } else {
                    options.push(format!("[{} <{}>]", od.aliases[0], value_name));
                }
            } else if od.aliases[0].starts_with(LONG_OPTION) {
                options.push(od.aliases[0].to_string())
            } else {
                flags.push(od.aliases[0].chars().last().unwrap())
            }
            max_len = max(max_len, help_options.len());
            let valid_values = if od.valid_values.is_empty() {
                "".to_string()
            } else {
                od.valid_values.join(",")
            };
            help_lines.push((help_options, od.description.to_string(), valid_values));
        }

        let mut usage = T.usage(program_name);

        if !flags.is_empty() {
            flags.sort();
            usage.push_str(&format!(
                " [-{}]",
                flags
                    .iter()
                    .fold(String::default(), |acc, c| { acc + &c.to_string() })
            ));
        }

        if !options.is_empty() {
            options.sort_by(|a, b| {
                a.trim_start_matches(SHORT_OPTION)
                    .cmp(b.trim_start_matches(SHORT_OPTION))
            });
            usage.push_str(&format!(" {}", options.join(" ")));
        }

        let x: &[_] = &['[', '-'];
        if !requireds.is_empty() {
            requireds.sort_by(|a, b| a.trim_start_matches(x).cmp(b.trim_start_matches(x)));
            usage.push_str(&format!(" {}", requireds.join(" ")));
        }

        if !self.argument_names.is_empty() {
            usage.push_str(&format!(" <{}>", self.argument_names.join("> <")));
        }

        for (options, description, valid_values) in help_lines {
            if valid_values.is_empty() {
                usage.push_str(&format!("\n{options:>max_len$} : {description}"));
            } else {
                usage.push_str(&format!(
                    "\n{options:>max_len$} : [{valid_values}]. {description}"
                ));
            }
        }

        usage
    }

    fn find_option_def(&self, option: &str) -> Option<&OptionDef> {
        let od_idx = self.option_def_map.get(option)?;
        Some(&self.option_defs[*od_idx])
    }

    fn validate_options(&self, options: &mut HashMap<String, String>, usage: &str) {
        for option in self.option_def_map.keys() {
            if !options.contains_key(*option)
                && let Some(od) = self.find_option_def(option) {
                    let default = od.default_value.unwrap_or_else(|| {
                        panic!("{}", format_usage(&T.option_required(option), usage))
                    });
                    options.insert(option.to_string(), default.to_string());
            }
            let od = self.find_option_def(option).unwrap();
            let value = options.get(*option).unwrap().as_str();
            if !od.valid_values.is_empty() && !od.valid_values.contains(&value) {
                panic_msg(format_usage(
                    &T.option_value_invalid(option, &od.valid_values),
                    usage,
                ));
            }
        }
    }

    fn parse_option(
        &self,
        option: String,
        value: Option<&String>,
        usage: &str,
        options: &mut HashMap<String, String>,
    ) -> bool {
        let mut skip = false;

        if let Some(option_def) = self.find_option_def(&option) {
            let val = if option_def.value_name.is_none() {
                TRUE
            } else {
                if value.is_none() {
                    panic_msg(format_usage(&T.option_value_required(&option), usage));
                }
                skip = true;
                value.unwrap()
            };
            for alias in &option_def.aliases {
                if options.insert(alias.to_string(), val.to_string()).is_some() {
                    panic_msg(format_usage(&T.option_multiple_found(alias), usage));
                }
            }
        } else if !option.starts_with(LONG_OPTION) && option.starts_with(SHORT_OPTION) {
            let flags = option.trim_start_matches(SHORT_OPTION);
            for f in flags.chars() {
                let flag = format!("-{f}");
                let flag_def = self.find_option_def(&flag).unwrap_or_else(|| {
                    panic!("{}", format_usage(&T.flag_not_defined(&flag), usage))
                });
                if flag_def.value_name.is_none() {
                    if options.insert(flag, TRUE.to_string()).is_some() {
                        panic_msg(format_usage(&T.option_multiple_flags(f), usage));
                    }
                } else {
                    panic_msg(format_usage(&T.option_invalid_flag(&flag), usage));
                }
            }
        } else {
            panic_msg(format_usage(&T.option_not_defined(&option), usage));
        }
        skip
    }
}

/// Defines the valid options for this program
struct OptionDef {
    /// The aliases for this option, e.g., `vec!["-f", "--filename"]`
    aliases: Vec<&'static str>,
    /// The name for the value associated with the option, e.g., `file_path`. If `None`,
    /// this option will be treated as a flag and the default value will be `false`.
    value_name: Option<&'static str>,
    /// An Option containing the value to use if one is not supplied. If `None`,
    /// then this option will be considered required and will panic if the program tries to use it.
    default_value: Option<&'static str>,
    /// The description of this option, e.g., `The file to be read`.
    description: &'static str,
    /// Valid values accepted in this option
    valid_values: Vec<&'static str>,
}

impl OptionDef {
    /// Creates a new OptionDef
    ///
    /// # Arguments
    ///
    /// * `aliases` - The aliases for this option, e.g., `vec!["-f", "--filename"]`
    /// * `value_name` - The name for the value associated with the option, e.g., `file_path`. If `None`,
    ///   this option will be treated as a flag and the default value will be `false`.
    /// * `default_value` - An Option<T> containing the value to use if one is not supplied. If `None`,
    ///   then this option will be considered required and will panic if a value is not specified on the
    ///   commandline. If `value_name` == `None`, `default_value` is ignored.
    /// * `description` - The description of this option, e.g., `The file to be read`
    /// * `valid_values` - A list of values to validate this option against.
    ///
    /// # Panics
    ///
    /// * Panics if the alias does not start with `-` or `--`.
    /// * Panics if the alias starts with `--` and the length is less than 4.
    /// * Panics if the alias starts with `-` and the length is not equal to 2.
    ///
    pub(crate) fn new(
        aliases: Vec<&'static str>,
        value_name: Option<&'static str>,
        default_value: Option<&'static str>,
        description: &'static str,
        valid_values: Vec<&'static str>,
    ) -> Self {
        Self::validate_aliases(&aliases);
        OptionDef {
            description,
            aliases,
            value_name,
            default_value,
            valid_values,
        }
    }

    fn validate_aliases(aliases: &Vec<&'static str>) {
        for alias in aliases {
            let option_len = alias.trim_start_matches(SHORT_OPTION).len();
            if alias.starts_with(LONG_OPTION) {
                if option_len < 2 || alias.len() - option_len > 2 {
                    panic_msg(T.option_invalid_long_name(alias));
                }
            } else if alias.starts_with(SHORT_OPTION) {
                if option_len == 0 || option_len > 1 {
                    panic_msg(T.option_invalid_short_name(alias));
                }
            } else {
                panic_msg(T.option_invalid_name(alias));
            };
        }
    }
}
