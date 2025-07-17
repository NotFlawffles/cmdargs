use crate::{
    args::Args,
    command::command_pattern::CommandPattern,
    option::{
        Option, Options,
        option_pattern::{ArgumentedOptPatArg, OptionPattern},
    },
};

pub mod command_pattern;

pub struct Command<'a> {
    pub command_pattern: CommandPattern<'a>,
    pub arguments: Vec<String>,
    pub options: Options,
}

impl<'a> Command<'a> {
    pub fn from_args(args: Args, command_patterns: &[CommandPattern<'a>]) -> Result<Self, String> {
        let args = args.as_vec();
        let mut args = args.iter();
        _ = args.next();

        let Some(command) = args.next() else {
            return Err(format!(
                "Expected any of these commands:
    {:?}",
                command_patterns
                    .iter()
                    .map(|pat| &pat.name)
                    .collect::<Vec<_>>()
            ));
        };

        let Some(command_pattern) = command_patterns
            .iter()
            .find(|pat| pat.name == *command)
            .cloned()
        else {
            return Err(format!("Invalid command: {command}."));
        };

        let mut arguments = Vec::new();
        let mut options = Vec::new();
        let args = args.collect::<Vec<_>>();
        let args_left = args.len();
        let mut index = 0;

        while index < args_left {
            let arg = args.get(index).unwrap().to_string();

            if arg.starts_with('-') {
                let mut option = arg;
                option.remove(0);

                let pattern = command_pattern
                    .option_patterns
                    .iter()
                    .find(|pat| pat.name() == option);

                if pattern.is_none() {
                    return Err(format!(
                        "Invalid option: {option}, expected any of these:
    {}",
                        command_pattern
                            .option_patterns
                            .iter()
                            .map(|pat| format!("{pat}"))
                            .collect::<Vec<_>>()
                            .join(", ")
                    ));
                }

                let option = match pattern.unwrap() {
                    OptionPattern::Standalone(name) => Option::Standalone(name),

                    OptionPattern::Argumented(name, expected_values) => {
                        if index + 1 < args_left
                            && match expected_values {
                                ArgumentedOptPatArg::Specific(values) => {
                                    values.contains(&args.get(index + 1).unwrap().as_str())
                                }
                                ArgumentedOptPatArg::Any => true,
                            }
                        {
                            let arg = args.get(index + 1).unwrap();
                            index += 1;

                            Option::Argumented(name, arg.to_string())
                        } else {
                            return Err(format!("-{name} requires {}.", expected_values,));
                        }
                    }
                };

                options.push(option);
            } else {
                if arguments.len() + 1 > command_pattern.value_count {
                    return Err(format!(
                        "{command} expects {} arguments.",
                        command_pattern.value_count,
                    ));
                }

                arguments.push(arg.to_string());
            }

            index += 1;
        }

        if arguments.len() < command_pattern.value_count {
            return Err(format!(
                "{command} expects {} arguments.",
                command_pattern.value_count,
            ));
        }

        Ok(Self {
            command_pattern,
            arguments,
            options: Options::from(options),
        })
    }

    pub fn execute(&'a self) {
        (self.command_pattern.callback)(&self.arguments, &self.options);
    }
}
