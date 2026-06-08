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

mod modules {

    pub mod command {
        use std::{fmt, ops::RemAssign};

        pub struct Command {
            pub cmd: String,
            pub args: Vec<String>,
            pub output: String,
        }

        pub enum CommandType {
            Builtin(Command),
            External(Command),
        }

        impl Command {
            pub fn new(line: &str) -> Self {
                let command = Command::parse(line);

                command
            }

            fn parse(line: &str) -> Command {
                let mut command = Command {
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
        }

        impl fmt::Display for Command {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "Cmd:{} Args:{}", self.cmd, self.args.join(" "))
            }
        }
    }

    pub mod app {
        use std::io::Read;
        use std::io::{self, Write};
        use std::process::Command;

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

                            let cmd = crate::modules::command::Command::new(cmdstr);
                            println!("{}", cmd);

                            if cmdstr == "exit".to_owned() {
                                self.quit();
                            }
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

fn main() {
    let mut app = modules::app::ShellApp::new();
    //app.prompt("# ");

    app.run();
}
