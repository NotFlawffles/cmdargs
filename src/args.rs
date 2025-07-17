use std::env;

pub enum Args {
    CommandLineArgs,
    Vec(Vec<&'static dyn ToString>),
}

impl Args {
    pub fn as_vec(&self) -> Vec<String> {
        match self {
            Self::CommandLineArgs => env::args().collect(),
            Self::Vec(vec) => vec.iter().map(|arg| arg.to_string()).collect(),
        }
    }
}
