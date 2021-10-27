extern crate chrono;
extern crate lazy_static;
use std::collections::HashMap;
use std::sync::RwLock;
use crate::*;

lazy_static! {
    pub static ref COMMAND_LIST: RwLock<CommandList> = Default::default();
}

pub struct Command {
    pub doc: String,
    pub longdoc: String,
    pub fun: FullFn,
}

pub struct CommandList {
    pub commands: HashMap<String, Command>,
}

impl Default for CommandList {
    fn default() -> Self {
        let mut ret = CommandList {
            commands: Default::default(),
        };
        ret.add_command(
            "help",
            "get help on commands",
            " - help: get a list of all commands\n - help <cmd>: get specific help for one command",
            CommandFunction::TextFn(Box::new(help)),
        )
        .unwrap();
        ret
    }
}

fn help(params: Vec<String>) -> String {
    let obj = COMMAND_LIST.read().unwrap();
    match params.len() {
        0 => {
            let mut ret = String::new();
            ret.push_str(&format!("**Time**: {}\n\n**WebDebug Version**: {}\n\n**Commands**:\n", Local::now(), VERSION.unwrap()));
            for (key, value) in &obj.commands {
                ret.push_str(&format!(" - `{}`: {}\n", key, value.doc));
            }
            ret
        }
        1 => {
            if obj.commands.contains_key(&params[0]) {
                let cmd = &obj.commands[&params[0]];
                format!("Documentation for command `{}`:\n\n**Brief**: {}\n\n**Details**:\n{}", params[0], cmd.doc, cmd.longdoc)    
            }
            else {
                format!("Documentation for {} **not found**!", params[0])    
            }
        }
        _ => {
            format!("illegal number of parameters for help-command.")
        }
    }
}

impl CommandList {
    fn add_command_fn_obj(
        &mut self,
        name: &str,
        doc: &str,
        longdoc: &str,
        command: FullFn,
    ) -> Result<()> {
        self.commands.insert(
            name.to_owned(),
            Command {
                doc: doc.to_owned(),
                longdoc: longdoc.to_owned(),
                fun: command,
            },
        );
        Ok(())
    }

    pub fn add_command(
        &mut self,
        name: &str,
        doc: &str,
        longdoc: &str,
        command: CommandFunction,
    ) -> Result<()> {
        match command {
            CommandFunction::FullFn(f) => self.add_command_fn_obj(name, doc, longdoc, f),
            CommandFunction::TextFn(f) => self.add_command_fn_obj(
                name,
                doc,
                longdoc,
                Box::new(move |flavor: &Flavor, params: Vec<String>| -> String {
                    match flavor {
                        Flavor::TEXT => f(params),
                        Flavor::HTML => htmlify(&f(params)),
                    }
                }),
            ),
        }
    }
}
