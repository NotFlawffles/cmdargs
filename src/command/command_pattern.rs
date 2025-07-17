use crate::option::{Options, option_pattern::OptionPattern};

#[derive(Clone)]
pub struct CommandPattern<'a> {
    pub name: String,
    pub value_count: usize,
    pub option_patterns: &'static [OptionPattern],
    pub callback: &'a dyn Fn(&'a [String], &Options),
}

impl<'a> CommandPattern<'a> {
    pub fn new(
        name: &str,
        value_count: usize,
        option_patterns: &'static [OptionPattern],
        callback: &'a dyn Fn(&'a [String], &Options),
    ) -> Self {
        Self {
            name: name.to_string(),
            value_count,
            option_patterns,
            callback,
        }
    }
}
