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
        struct Command {
            pub cmd: String,
            pub args: Vec<String>,
        }

        pub enum CommandType {
            Builtin(Command),
            External(Command),
        }
    }

    pub mod app {
        use std::io::Read;
        use std::io::{self, Write};

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

                    match io::stdin().read_line(&mut command) {
                        Ok(bytes) => {
                            let cmd = command.trim();
                            println!("cmd: \"{}\" (bytes read: {bytes})", cmd);

                            if cmd == "exit".to_owned() {
                                self.quit();
                            }
                        }
                        Err(_) => println!("{command}: command not found"),
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
