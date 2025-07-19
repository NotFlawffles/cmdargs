use std::fmt::Display;

/// Specifies whether the `OptionPattern::Argumented` variant should match against a `Specific` set
/// of position values (e.g. `["exe", "lib"]`) or `Any` value.
#[derive(Clone)]
pub enum ArgumentedOptPatArg {
    Specific(&'static [&'static str]),
    Any,
}

/// Specifies which options are allowed to pass for an appropriate `Command` for each
/// `CommandPattern`.
#[derive(Clone)]
pub enum OptionPattern {
    /// Standalone option is an option with no required argument, (e.g. `-force`).
    ///
    /// ### Fields
    /// - `&'static str`: Name of the option.
    Standalone(&'static str),

    /// Argumented option is an option that requires a value, the value which was either positional
    /// or any, (e.g. `-warning-level all`).
    ///
    /// ### Fields
    /// - `&'static str`: Name of the option.
    /// - `ArgumentedOptPatArg`: Whether the option value should be `Specific` or `Any` value,
    /// refer to `ArgumentedOptPatArg`.
    Argumented(&'static str, ArgumentedOptPatArg),
}

impl OptionPattern {
    pub fn name(&self) -> &str {
        match self {
            Self::Standalone(name) => name,
            Self::Argumented(name, ..) => name,
        }
    }
}

impl Display for OptionPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Standalone(name) => write!(f, "-{name}"),
            Self::Argumented(name, expected_values) => write!(f, "-{name} {expected_values}"),
        }
    }
}

impl Display for ArgumentedOptPatArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Specific(values) => write!(f, "{values:?}"),
            Self::Any => write!(f, "<any value>"),
        }
    }
}
