use std::fmt::Display;

#[derive(Clone)]
pub enum ArgumentedOptPatArg {
    Specific(&'static [&'static str]),
    Any,
}

#[derive(Clone)]
pub enum OptionPattern {
    Standalone(&'static str),
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
