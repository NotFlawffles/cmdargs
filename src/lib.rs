//! # CMDArgs
//! Minimal, precise library for primarily fetching and sorting runtime command line arguments accordingly.
//!
//! For each command line argument will be rawly read, based on a user-specified command pattern
//! it will check if it was expected, then it will ensure that `n` (user-defined) amount of
//! arguments will be read (order between arguments and options is arbitrary), at the same time
//! it will also be reading options based on user-specified set of option patterns.
//!
//! ## Example
//!
//! ```rust
//! use crate::{
//!     args::Args,
//!     command::Command,
//!     command_pattern::CommandPattern,
//!     option::{ArgumentedOptionPatternArgument, Option, OptionPattern},
//! };
//!
//! fn main() {
//!     let command = Command::from_args(
//!         Args::CommandLineArgs,
//!         vec![
//!             CommandPattern::new(
//!                 "greet",
//!                 1,
//!                 vec![
//!                     OptionPattern::Argumented(
//!                         "method",
//!                         ArgumentedOptionPatternArgument::Specific(&["hello", "hi"]),
//!                     ),
//!                     OptionPattern::Argumented("also-greet", ArgumentedOptionPatternArgument::Any),
//!                 ],
//!                 &|args, opts| {
//!                     // implementation of "greet" command
//!                     let name = args.get(0).unwrap();
//!
//!                     let method = opts.get("method").map_or("hello", |opt| {
//!                         if let Option::Argumented(.., value) = opt {
//!                             value
//!                         } else {
//!                             unreachable!()
//!                         }
//!                     });
//!
//!                     let also_greet = opts.get("also-greet");
//!
//!                     println!("{method} {name}!");
//!
//!                     if let Some(option) = also_greet {
//!                         if let Option::Argumented(.., name) = option {
//!                             println!("and {method} {name}!");
//!                         }
//!                     }
//!                 },
//!             ), /* you can also add more commands with the same logic and try other options */
//!         ],
//!     );
//!
//!     if let Err(message) = command {
//!         eprintln!("{message}");
//!         return;
//!     }
//!
//!     let command = command.unwrap();
//!     command.execute();
//! }
//! ```
//!
//! Result (the -- is required to prevent furhter options from passing into cargo):
//! ```shell
//! $ cargo run -- greet NotFlawffles -method hi -also-greet "John Doe"
//! hi NotFlawffles!
//! and hi John Doe!
//! ```
//!
//! ## Limitations (for now, will be resolved very soon)
//! - No support for (--) for options, only a generic (-) that works for any length.



pub mod args;
pub mod command;
pub mod command_pattern;
pub mod option;
