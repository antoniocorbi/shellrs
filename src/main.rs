// Copyright (C) 2026  Antonio-Miguel Corbi Bellot
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

#![allow(unused_imports)]
use std::os::unix::process::CommandExt;
use std::process::ExitCode;

mod modules {

    pub mod command {
        use std::env;
        use std::path::Path;
        use std::{fmt, ops::RemAssign};

        use crate::modules::app::ShellApp;

        pub struct Command<'a> {
            pub app: &'a mut ShellApp,
            pub cmd: String,
            pub args: Vec<String>,
            pub output: String,
        }

        pub trait BuiltIn {
            fn run(&mut self);
        }

        impl BuiltIn for Command<'_> {
            fn run(&mut self) {
                match self.cmd.as_str() {
                    "echo" => { /* lógica de echo */ }
                    "cd" => {
                        /* lógica de cd */
                        self.handle_cd();
                    }
                    "pwd" => {
                        self.handle_pwd();
                    }
                    "exit" => {
                        self.app.quit();
                    }
                    _ => println!("unknown builtin"),
                }
            }
        }

        impl Command<'_> {
            // pub fn new(line: &str) -> Self {
            //     let command = Command::parse(line);
            //
            //     command
            // }

            pub fn parse<'a>(line: &'a str, app: &'a mut ShellApp) -> Command<'a> {
                let mut command = Command {
                    app,
                    cmd: "".to_owned(),
                    args: vec![],
                    output: "".to_owned(),
                };

                if line.len() != 0 {
                    let mut split = line.split(' ');
                    let cmd = match split.next() {
                        Some(c) => c,
                        None => "",
                    }
                    .to_string();

                    let args = split.map(|s| s.to_owned()).collect();

                    command.cmd = cmd;
                    command.args = args;
                }

                command
            }

            fn handle_pwd(&self) {
                if let Ok(path) = env::current_dir() {
                    println!("{}", path.display());
                }
            }

            fn handle_cd(&self) {
                let root = Path::new(&self.args[0]);
                // assert!(env::set_current_dir(&root).is_ok());
                if env::set_current_dir(&root).is_ok() {
                    println!("{}", root.display());
                }
            }
        }

        impl fmt::Display for Command<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "Cmd:{} Args:{}", self.cmd, self.args.join(" "))
            }
        }
    }

    pub mod app {
        use crate::modules::command::Command;
        use std::io::Read;
        use std::io::{self, Write};
        //use std::process::Command;

        use crate::modules::command::BuiltIn;

        pub struct ShellApp {
            prompt: String,
            quit: bool,
        }

        impl ShellApp {
            pub fn new() -> Self {
                ShellApp {
                    prompt: "$ ".to_owned(),
                    quit: false,
                }
            }

            pub fn prompt(&mut self, p: &str) {
                self.prompt = p.to_owned();
            }

            pub fn quit(&mut self) {
                self.quit = true;
            }

            pub fn run(&mut self) {
                loop {
                    print!("{}", self.prompt);
                    let _ = io::stdout().flush();

                    let mut command: String = String::new();

                    // if let Ok(bytes) = io::stdin().read_line(&mut command) {
                    //     let cmdstr = command.trim();
                    //     println!("cmd: \"{}\" (bytes read: {bytes})", cmdstr);
                    //
                    //     let cmd = crate::modules::command::Command::new(cmdstr);
                    //     println!("{}", cmd);
                    //
                    //     if cmdstr == "exit".to_owned() {
                    //         self.quit();
                    //     }
                    // }

                    match io::stdin().read_line(&mut command) {
                        Ok(bytes) => {
                            let cmdstr = command.trim();
                            println!("cmd: \"{}\" (bytes read: {bytes})", cmdstr);

                            let mut command = Command::parse(cmdstr, self);
                            command.run();

                            // if cmdstr == "exit".to_owned() {
                            //     self.quit();
                            // }
                        }
                        Err(_) => println!("{command}: Error reading command"),
                    };

                    // Quit ceremony
                    if self.quit {
                        break;
                    }
                }
            }
        }
    }
}

fn main() -> ExitCode {
    let mut app = modules::app::ShellApp::new();
    //app.prompt("# ");

    app.run();

    ExitCode::SUCCESS
}
