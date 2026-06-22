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
        use crate::modules::app::ShellApp;
        use std::env;
        use std::path::Path;
        use std::{fmt, ops::RemAssign};

        pub struct Command<'a> {
            pub app: &'a mut ShellApp,
            pub cmd: String,
            pub args: Vec<String>,
            pub output: String,
        }

        pub trait CommandExt {
            fn run(&mut self);
        }

        impl CommandExt for Command<'_> {
            fn run(&mut self) {
                match self.cmd.as_str() {
                    // First check for built-ins
                    "echo" => {
                        /* lógica de echo */
                        self.handle_echo();
                    }
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
                    "version" => {
                        ShellApp::version();
                    }
                    // Now, external ones
                    _ => {
                        self.handle_external();
                    }
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
                if let Some(dir) = self.args.first() {
                    let root = Path::new(dir);
                    if env::set_current_dir(root).is_err() {
                        eprintln!("cd: {}: No such file or directory", dir);
                    } else {
                        println!("{}", root.display());
                    }
                }
            }

            fn handle_echo(&mut self) {
                if self.args.len() != 0 {
                    let mut line: String = String::new();
                    for a in &self.args {
                        line = format!("{line} {a}");
                    }
                    self.output = line.trim().to_owned();
                    println!("[{}]", self.output);
                } else {
                    println!();
                }
            }

            fn handle_external(&mut self) {
                use std::io::ErrorKind;
                use std::process::Command;

                let result = Command::new(&self.cmd).args(&self.args).output();

                match result {
                    Ok(output) => {
                        let s = unsafe { String::from_utf8_unchecked(output.stdout) };
                        self.output = s;

                        if self.output.len() != 0 {
                            println!("{}", self.output);
                        } else {
                            println!();
                        }
                        // println!("El comando existía y se ejecutó.");
                        // println!("¿Se ejecutó con éxito?: {}", status.success());
                    }
                    Err(ref e) if e.kind() == ErrorKind::NotFound => {
                        eprintln!("Error: command not found");
                    }
                    Err(e) => {
                        println!("I/O error: {}", e);
                    }
                }

                // dbg!(&self.cmd);
                // dbg!(&self.args);
                // println!("External Command: output: {s}");
                // println!("\nExternal Command: status: {}", output.status);
            }
        }

        impl fmt::Display for Command<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "Cmd:{} Args:{}", self.cmd, self.args.join(" "))
            }
        }
    }

    pub mod app {
        use crate::modules::command::{Command, CommandExt};
        use std::io::{self, Read, Write};

        const APP_VERSION: &str = "V0.1.0-󰀫";

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

            pub fn version() {
                println!("{}", APP_VERSION);
            }

            pub fn prompt(&mut self, p: &str) {
                self.prompt = p.to_owned();
            }

            pub fn quit(&mut self) {
                self.quit = true;
            }

            pub fn run(&mut self) {
                let mut command: String = String::new();

                loop {
                    print!("{}", self.prompt);
                    let _ = io::stdout().flush().unwrap();

                    // Clean the buffer
                    command.clear();

                    match io::stdin().read_line(&mut command) {
                        // Ctrl-D
                        Ok(0) => {
                            // EOF detected (Ctrl-D / Ctrl-Z)
                            println!("\nBailing out...");
                            self.quit();
                            //break;
                        }
                        Ok(bytes) => {
                            let cmdstr = command.trim();
                            //println!("cmd: \"{}\" (bytes read: {bytes})", cmdstr);

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
