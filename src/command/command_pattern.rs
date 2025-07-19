use crate::{
    option::{Options, option_pattern::OptionPattern},
};

/// Used to declare an accepted command pattern.
///
/// ### Fields
/// - `name`: Name of the command.
/// - `args_count`: Amount of arguments accepted.
/// - `option_patterns`: Accepted option patterns for the command.
/// - `callback`: A callback used when the command is being invoked,
/// definition: `&|args: &'a [String], opts: &[Options]|`.
#[derive(Clone)]
pub struct CommandPattern<'a> {
    pub name: String,
    pub args_count: usize,
    pub option_patterns: &'static [OptionPattern],
    pub callback: &'a dyn Fn(&'a [String], &Options),
}

impl<'a> CommandPattern<'a> {
    pub fn new(
        name: &str,
        args_count: usize,
        option_patterns: &'static [OptionPattern],
        callback: &'a dyn Fn(&'a [String], &Options),
    ) -> Self {
        Self {
            name: name.to_string(),
            args_count,
            option_patterns,
            callback,
        }
    }
}
