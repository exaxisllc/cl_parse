use crate::{LONG_OPTION, SHORT_OPTION};

/// Defines the valid options for this program
pub(crate) struct OptionDef {
  /// The aliases for this option. e.g. -f --filename
  pub(crate) aliases:Vec<&'static str>,
  /// The name for the value associated with the option. e.g. -f path. If None,
  /// this option will be treated as a flag and the default value will be false.
  pub(crate) value_name:Option<&'static str>,
  /// An Option containing the value to use if one is not supplied. If `None`,
  /// then this option will be considered required and will panic if the program tries to use it.
  pub(crate) default_value:Option<&'static str>,
  /// The description of this option. e.g. The file to be read.
  pub(crate) description:&'static str,
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
  /// then this option will be considered required and will panic if a value is not specified on the
  /// commandline. if `value_name` is None, `default_value` is ignored.
  /// * `description` - The description of this option. e.g. The file to be read.
  ///
  /// # Panics
  ///
  /// * Panics if the alias does not start with '-' or '--'.
  /// * Panics if the alias starts with '--' and the length is less than 4
  /// * Panics if the alias starts with '-' and the length is not equal to 2
  ///
  #[inline]
  pub(crate) fn new(aliases:Vec<&'static str>, value_name:Option<&'static str>, default_value:Option<&'static str>, description:&'static str) -> Self {
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
