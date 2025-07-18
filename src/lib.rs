//! # CommandArgs
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
//! use crate::{args::Args, command::{command_pattern::CommandPattern, Command}, option::option_pattern::{ArgumentedOptPatArg, OptionPattern}};
//! 
//! fn main() {
//!     let command = Command::from_args(
//!         Args::CommandLineArgs,
//!         &[
//!             CommandPattern::new(
//!                 "greet",
//!                 1,
//!                 &[
//!                     OptionPattern::Argumented(
//!                         "method",
//!                         ArgumentedOptPatArg::Specific(&["hello", "hi"]),
//!                     ),
//!                     OptionPattern::Argumented("also-greet", ArgumentedOptPatArg::Any),
//!                 ],
//!                 &|args, opts| {
//!                     // implementation of "greet" command
//!                     let name = args.get(0).unwrap();
//!                     let method = opts.get_argumented("method").map_or("hello", |method| method);
//!                     let also_greet = opts.get_argumented("also-greet");
//! 
//!                     println!("{method} {name}!");
//! 
//!                     if let Some(name) = also_greet {
//!                         println!("and {method} {name}!");
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
pub mod option;
