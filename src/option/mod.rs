pub mod option_pattern;

/// Value generated from matching against `OptionPattern` accordingly. `Option`'s are used as
/// containers.
pub enum Option {
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
    /// - `String`: Value of the option.
    Argumented(&'static str, String),
}

/// Wrapper around `Option` values. A container that is primarily used to improve user access in
/// `callback` via its methods.
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
    /// Attempts to find a standalone option by its name, if it fails to find an option with that
    /// name OR the name referred to an option that is NOT a standalone, it returns `false`,
    /// otherwise returns `true`.
    pub fn get_standalone(&self, name: &'static str) -> bool {
        for option in self.options.iter() {
            if let Option::Standalone(optname) = option
                && *optname == name
            {
                return true;
            }
        }

        false
    }

    /// Attempts to find an argumented option by its name, if it fails to find an option with that
    /// name OR the name referred to an option that is NOT an argumented, it returns `None`,
    /// otherwise returns the value of the option.
    pub fn get_argumented(&self, name: &'static str) -> std::option::Option<&str> {
        for option in self.options.iter() {
            if let Option::Argumented(optname, value) = option
                && *optname == name
            {
                return Some(value);
            }
        }

        None
    }

    /// Returns all options matched without check.
    pub fn values(&self) -> &Vec<Option> {
        &self.options
    }
}

impl From<Vec<Option>> for Options {
    fn from(value: Vec<Option>) -> Self {
        Self { options: value }
    }
}
