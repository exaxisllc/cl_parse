use crate::text::Text;

#[derive(Default)]
pub(super) struct EnUs {}

impl Text for EnUs {
  fn option_redefined(&self, option: &str) -> String {
    format!("Option '{option}' cannot be redefined")
  }

  fn argument_defined_ne_found(&self, defined: usize, found: usize) -> String {
    format!("Defined {defined} arguments, found {found} arguments")
  }

  fn option_value_required(&self, option: &str) -> String {
    format!("A value is required for option '{option}'")
  }

  fn option_multiple_found(&self, option: &str) -> String {
    format!("Multiple '{option}' options or aliases on commandline")
  }

  fn option_multiple_flags(&self, flag: char) -> String {
    format!("Multiple '-{flag}' options or aliases on commandline")
  }

  fn option_invalid_flag(&self, option: &str) -> String {
    format!("Option '{option}' is not a flag")
  }

  fn option_not_defined(&self, option: &str) -> String {
    format!("Option '{option}' not defined")
  }

  fn flag_not_defined(&self, flag: &str) -> String {
    format!("Flag '{flag}' not defined")
  }

  fn option_invalid_long_name(&self, option: &str) -> String {
    format!("Invalid option name '{option}'. Long option names must start with '--' and be greater than 1 character. e.g. --lo")
  }

  fn option_invalid_short_name(&self, option: &str) -> String {
    format!("Invalid option name '{option}'. Short option names must start with '-' and be 1 character. e.g. -f")
  }

  fn option_invalid_name(&self, option: &str) -> String {
    format!("Invalid option name '{option}'. Options must start with '-' or '--'")
  }

  fn option_required(&self, option: &str) -> String {
    format!("Option '{option}' is required")
  }

  fn option_not_found(&self, option: &str) -> String {
    format!("Option '{option}' not found")
  }

  fn option_value_invalid(&self, option: &str, valid_values: &Vec<&'static str>) -> String {
    let vv = valid_values.join(",");
    format!("Option '{option}' must be one of [{vv}]")
  }

  fn argument_invalid_index(&self, index: usize) -> String {
    format!("Argument index '{index}' is out of bounds")
  }

  fn option_cannot_convert(&self, option: &str, value: &str) -> String {
    format!("Cannot convert option '{option}' from '{value}'")
  }

  fn argument_cannot_convert(&self, index: usize, value: &str) -> String {
    format!("Cannot convert argument '{index}' from '{value}'")
  }

  fn usage(&self, program_name: &str) -> String {
    format!("Usage: {program_name}")
  }
}