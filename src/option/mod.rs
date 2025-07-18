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
    pub fn get_standalone(&self, name: &'static str) -> bool {
        for option in self.options.iter() {
            if let Option::Standalone(optname) = option && *optname == name {
                return true;
            }
        }

        false
    }

    pub fn get_argumented(&self, name: &'static str) -> std::option::Option<&str> {
        for option in self.options.iter() {
            if let Option::Argumented(optname, value) = option && *optname == name {
                return Some(value);
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
