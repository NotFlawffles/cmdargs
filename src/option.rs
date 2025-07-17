use std::fmt::Display;

pub enum Option {
    Standalone(&'static str),
    Argumented(&'static str, String),
}

#[derive(Clone)]
pub enum ArgumentedOptionPatternArgument {
    Specific(&'static [&'static str]),
    Any,
}

#[derive(Clone)]
pub enum OptionPattern {
    Standalone(&'static str),
    Argumented(&'static str, ArgumentedOptionPatternArgument),
}

pub struct Options {
    options: Vec<Option>,
}

impl Option {
    pub fn name(&self) -> &str {
        match self {
            Self::Standalone(name) => name,
            Self::Argumented(name, ..) => name,
        }
    }
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

impl Display for ArgumentedOptionPatternArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Specific(values) => write!(f, "{values:?}"),
            Self::Any => write!(f, "<any value>"),
        }
    }
}

impl Options {
    pub fn get(&self, name: &'static str) -> std::option::Option<&Option> {
        for option in self.options.iter() {
            if option.name() == name {
                return Some(option);
            }
        }

        None
    }

    pub fn values(&self) -> &Vec<Option> {
        &self.options
    }
}

impl From<Vec<Option>> for Options {
    fn from(value: Vec<Option>) -> Self {
        Self { options: value }
    }
}
