use crate::option::{OptionPattern, Options};

#[derive(Clone)]
pub struct CommandPattern<'a> {
    pub name: String,
    pub value_count: usize,
    pub option_patterns: Vec<OptionPattern>,
    pub callback: &'a dyn Fn(&'a [String], &Options),
}

impl<'a> CommandPattern<'a> {
    pub fn new(
        name: &str,
        value_count: usize,
        option_patterns: Vec<OptionPattern>,
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
