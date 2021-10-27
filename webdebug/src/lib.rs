//! # webdebug - a simple command management for rust processes
//! 
//! Use [add_command] to add custom commands in your own modules.
//! Your functions can either be a [FullFn] supporting [Flavor::HTML] and [Flavor::MARKDOWN]
//! or a [TextFn] only supporting [Flavor::MARKDOWN].
//! 
//! ```rust
//!     webdebug::add_command(
//!         "demo2",
//!         "demo2 command",
//!         "a simple demo command",
//!         webdebug::CommandFunction::TextFn(Box::new(
//!             |_params:Vec<String>|{"This is a 🌶 demo".to_owned()}
//!         )),
//!     )
//! ```
//! 
//! [exec_command] is used, e.g., by a webserver to execute your commands
//! (see crate `webdebug-rocket`). The [Flavor] controls in what form the result
//! is expected: either [Flavor::HTML] or [Flavor::MARKDOWN].

extern crate markdown;
use chrono::Local;
use lazy_static::lazy_static;
mod command_management;

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Command output flavor.
pub enum Flavor {
    HTML,
    MARKDOWN,
}

/// Command function types
pub enum CommandFunction {
    FullFn(FullFn),
    TextFn(TextFn),
}

/// Command function supporting HTML and MARDOWN.
pub type FullFn = Box<dyn Send + Sync + Fn(&Flavor, Vec<String>) -> String>;
/// Command function supporting MARDOWN.
pub type TextFn = Box<dyn Send + Sync + Fn(Vec<String>) -> String>;


/// Add a new command to the list of available commands.
pub fn add_command(name: &str, doc: &str, longdoc: &str, command: CommandFunction) -> Result<()> {
    let mut obj = command_management::COMMAND_LIST.write()?;
    obj.add_command(name, doc, longdoc, command)
}

/// Execute a command to the list of available commands (done by, e.g., a web server).
pub fn exec_command(flavor: &Flavor, name: &str, params: Vec<String>) -> String {
    let obj = command_management::COMMAND_LIST.read().unwrap();
    match obj.commands.get(name) {
        Some(cmd) => (cmd.fun)(flavor, params),
        None => format!("command {} not found.", name),
    }
}

/// Welcome function (can be used by a webserver to display some meaningful text with some help).
pub fn hello(flavor: &Flavor) -> String {
    exec_command(flavor, "help", vec![])
}

/// Function to convert [Format::MARDOWN] to [Format::HTML].
pub fn htmlify(text_fn: &str) -> String {
    markdown::to_html(text_fn)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
