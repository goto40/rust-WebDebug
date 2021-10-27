extern crate markdown;
use chrono::Local;
use lazy_static::lazy_static;
mod command_management;

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub enum Flavor {
    HTML,
    TEXT,
}

pub enum CommandFunction {
    FullFn(FullFn),
    TextFn(TextFn),
}

pub type FullFn = Box<dyn Send + Sync + Fn(&Flavor, Vec<String>) -> String>;
pub type TextFn = Box<dyn Send + Sync + Fn(Vec<String>) -> String>;


pub fn add_command(name: &str, doc: &str, longdoc: &str, command: CommandFunction) -> Result<()> {
    let mut obj = command_management::COMMAND_LIST.write()?;
    obj.add_command(name, doc, longdoc, command)
}

pub fn exec_command(flavor: &Flavor, name: &str, params: Vec<String>) -> String {
    let obj = command_management::COMMAND_LIST.read().unwrap();
    match obj.commands.get(name) {
        Some(cmd) => (cmd.fun)(flavor, params),
        None => format!("command {} not found.", name),
    }
}

pub fn hello(flavor: &Flavor) -> String {
    exec_command(flavor, "help", vec![])
}

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
