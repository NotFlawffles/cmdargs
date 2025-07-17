pub mod option_pattern;

pub enum Option {
    Standalone(&'static str),
    Argumented(&'static str, String),
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
