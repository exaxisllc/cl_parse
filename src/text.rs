use lazy_static::lazy_static;
use std::collections::HashMap;
use sys_locale::get_locale;

type TextFactory = fn() -> Box<dyn Text + Sync>;

lazy_static! {
    static ref TEXT_MAP: HashMap<&'static str, TextFactory> = {
        let mut tm: HashMap<&'static str, TextFactory> = HashMap::new();
        tm.insert("en", || Box::new(en_us::EnUs {}));
        tm.insert("en-US", || Box::new(en_us::EnUs {}));
        tm
    };
    pub(crate) static ref T: Box<dyn Text + Sync> = get_text().expect("Could not load locale text");
}

fn get_text() -> Option<Box<dyn Text + Sync>> {
    let locale1 = get_locale().unwrap_or("en-US".to_string());
    let text_factory = TEXT_MAP.get(locale1.as_str()).or_else(|| {
        let locale2 = locale1.split_once("-").unwrap_or(("en", "US")).0;
        TEXT_MAP
            .get(locale2)
            .or_else(|| TEXT_MAP.get("en").or(None))
    })?;
    Some(text_factory())
}

pub(crate) trait Text {
    fn option_redefined(&self, option: &str) -> String;
    fn argument_defined_ne_found(&self, defined: usize, found: usize) -> String;
    fn option_value_required(&self, option: &str) -> String;
    fn option_multiple_found(&self, option: &str) -> String;
    fn option_multiple_flags(&self, flag: char) -> String;
    fn option_invalid_flag(&self, option: &str) -> String;
    fn option_not_defined(&self, option: &str) -> String;
    fn flag_not_defined(&self, flag: &str) -> String;
    fn option_invalid_long_name(&self, option: &str) -> String;
    fn option_invalid_short_name(&self, option: &str) -> String;
    fn option_invalid_name(&self, option: &str) -> String;
    fn option_required(&self, option: &str) -> String;
    fn option_not_found(&self, option: &str) -> String;
    fn option_value_invalid(&self, option: &str, valid_values: &[&'static str]) -> String;

    fn argument_invalid_index(&self, index: usize) -> String;
    fn option_cannot_convert(&self, option: &str, value: &str) -> String;
    fn argument_cannot_convert(&self, index: usize, value: &str) -> String;
    fn usage(&self, program_name: &str) -> String;
}

mod en_us;
