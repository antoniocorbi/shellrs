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
            app: &'a mut ShellApp,
            pub cmd: String,
            pub args: Vec<String>,
            pub output: String,
            is_builtin: bool,
        }

        pub trait CommandExt {
            fn run(&mut self);
            fn is_builtin(&self) -> bool;
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
                    "type" => {
                        self.handle_type();
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

            // fn is_builtin(&self) -> bool {
            //     if self.nargs() > 0 {
            //         match self.args[0].as_str() {
            //             "echo" | "cd" | "pwd" | "exit" | "version" | "type" => true,
            //             _ => false,
            //         }
            //     } else {
            //         false
            //     }
            // }

            fn is_builtin(&self) -> bool {
                self.is_builtin
            }
        }

        impl Command<'_> {
            // pub fn new(line: &str) -> Self {
            //     let command = Command::parse(line);
            //
            //     command
            // }

            pub fn check_builtin(cmd: &str) -> bool {
                match cmd {
                    "echo" | "cd" | "pwd" | "exit" | "version" | "type" => true,
                    _ => false,
                }
            }

            pub fn nargs(&self) -> usize {
                self.args.len()
            }

            pub fn parse<'a>(line: &'a str, app: &'a mut ShellApp) -> Command<'a> {
                let mut command = Command {
                    app,
                    cmd: "".to_owned(),
                    args: vec![],
                    output: "".to_owned(),
                    is_builtin: false,
                };

                if line.len() != 0 {
                    let mut split = line.split(' ');
                    let cmd = match split.next() {
                        Some(c) => c,
                        None => "",
                    }
                    .to_string();

                    let args: Vec<_> = split.map(|s| s.to_owned()).collect();
                    //dbg!(&args);
                    let is_builtin = Command::check_builtin(&cmd);

                    command.cmd = cmd;
                    command.args = args;
                    command.is_builtin = is_builtin;
                }

                command
            }

            fn handle_type(&self) {
                if self.is_builtin() {
                    println!("{} is a shell builtin.", self.args[0])
                } else {
                    println!("{} is not a shell builtin.", self.args[0])
                }
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

            #[deprecated(since = "1.0.0", note = "please use `handle_echo` instead")]
            fn old_handle_echo(&mut self) {
                if self.nargs() != 0 {
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

            fn handle_echo(&mut self) {
                if self.args.is_empty() {
                    println!();
                    return;
                }

                // Use join() to handle separators efficiently
                self.output = self.args.join(" ");
                println!("[{}]", self.output);
            }

            fn handle_external(&mut self) {
                use std::io::ErrorKind;
                use std::process::Command;
                let cmd = "/etc/profiles/per-user/acorbi/bin/sh";
                let mut args = String::new();

                for s in &self.args {
                    args += &format!("{} ", &s);
                }
                let args = format!("{} {}", &self.cmd, args.trim());
                //dbg!(&args);

                //dbg!(cmd);
                //dbg!(&args);
                //let args: Vec<_> = args.split(" ").filter(|e| !e.is_empty()).collect();
                let args = ["-c", &args];
                //dbg!(&args);

                let result = Command::new(cmd).args(args).output();

                // dbg!(&self.args);

                match result {
                    Ok(output) => {
                        // dbg!(&output);

                        // let s = unsafe { String::from_utf8_unchecked(output.stdout) };

                        let s = String::from_utf8(output.stdout)
                            .map_err(|e| {
                                eprintln!("Error decoding output as UTF-8: {}", e);
                                String::new()
                            })
                            .unwrap_or_else(|_| String::from(""));

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
                        Ok(_bytes) => {
                            let cmdstr = command.trim();
                            //println!("cmd: \"{}\" (bytes read: {bytes})", cmdstr);

                            #[cfg(feature = "debug")]
                            for c in cmdstr.chars() {
                                match c {
                                    '\x03' => println!("Se presionó C-c"),
                                    '\x04' => println!("Se presionó C-d"),
                                    '\x1e' => println!("Se presionó C-z"),
                                    _ => println!("Otra tecla: {}", c),
                                }
                            }

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

#[cfg(test)]
mod tests {
    use super::modules::app::ShellApp;
    use super::modules::command::{Command, CommandExt};
    use std::env;
    use std::path::Path;

    // ─── Command::parse() tests ─────────────────────────────────────────────

    #[test]
    fn test_parse_empty() {
        let mut app = ShellApp::new();
        let cmd = Command::parse("", &mut app);
        assert_eq!(cmd.cmd, "");
        assert!(cmd.args.is_empty());
    }

    #[test]
    fn test_parse_single_word() {
        let mut app = ShellApp::new();
        let cmd = Command::parse("ls", &mut app);
        assert_eq!(cmd.cmd, "ls");
        assert!(cmd.args.is_empty());
    }

    #[test]
    fn test_parse_with_args() {
        let mut app = ShellApp::new();
        let cmd = Command::parse("echo hello world", &mut app);
        assert_eq!(cmd.cmd, "echo");
        assert_eq!(cmd.args, vec!["hello", "world"]);
    }

    #[test]
    fn test_parse_whitespace_only() {
        let mut app = ShellApp::new();
        let cmd = Command::parse("   ", &mut app);
        assert_eq!(cmd.cmd, "");
        // split(' ') on "   " gives ["", "", "", ""]
        assert_eq!(cmd.args, vec!["", "", ""]);
    }

    #[test]
    fn test_parse_no_command() {
        let mut app = ShellApp::new();
        let cmd = Command::parse("  foo", &mut app);
        assert_eq!(cmd.cmd, "");
        // split(' ') on "  foo" gives ["", "", "foo"]
        assert_eq!(cmd.args, vec!["", "foo"]);
    }

    // ─── Display test ───────────────────────────────────────────────────────

    #[test]
    fn test_display() {
        let mut app = ShellApp::new();
        let cmd = Command::parse("echo foo bar", &mut app);
        assert_eq!(format!("{}", cmd), "Cmd:echo Args:foo bar");
    }

    // ─── Built-in: echo tests ───────────────────────────────────────────────

    #[test]
    fn test_echo_basic() {
        let mut app = ShellApp::new();
        let mut cmd = Command::parse("echo hello world", &mut app);
        cmd.run();
        assert_eq!(cmd.output, "hello world");
    }

    #[test]
    fn test_echo_single_arg() {
        let mut app = ShellApp::new();
        let mut cmd = Command::parse("echo foo", &mut app);
        cmd.run();
        assert_eq!(cmd.output, "foo");
    }

    #[test]
    fn test_echo_no_args() {
        let mut app = ShellApp::new();
        let mut cmd = Command::parse("echo", &mut app);
        cmd.run();
        assert_eq!(cmd.output, "");
    }

    #[test]
    fn test_echo_empty_arg() {
        let mut app = ShellApp::new();
        let mut cmd = Command::parse("echo ''", &mut app);
        cmd.run();
        assert_eq!(cmd.output, "''");
    }

    // ─── Built-in: cd tests ─────────────────────────────────────────────────

    #[test]
    fn test_cd_to_existing_dir() {
        let original = env::current_dir().unwrap();
        let tmp = env::temp_dir();
        let line = format!("cd {}", tmp.display());
        let mut app = ShellApp::new();
        let mut cmd = Command::parse(&line, &mut app);
        cmd.run();
        assert_eq!(env::current_dir().unwrap(), tmp);
        env::set_current_dir(&original).unwrap();
        assert_eq!(env::current_dir().unwrap(), original);
    }

    #[test]
    fn test_cd_to_nonexistent() {
        let original = env::current_dir().unwrap();
        let mut app = ShellApp::new();
        let mut cmd = Command::parse("cd /_shellrs_test_nonexistent_dir_", &mut app);
        cmd.run();
        // Directory should not have changed
        assert_eq!(env::current_dir().unwrap(), original);
    }

    #[test]
    fn test_cd_no_args() {
        let original = env::current_dir().unwrap();
        let mut app = ShellApp::new();
        let mut cmd = Command::parse("cd", &mut app);
        cmd.run();
        // No args → nothing happens, directory unchanged
        assert_eq!(env::current_dir().unwrap(), original);
    }

    #[test]
    fn test_cd_relative() {
        let original = env::current_dir().unwrap();
        let mut app = ShellApp::new();
        let mut cmd = Command::parse("cd src", &mut app);
        cmd.run();
        // src exists in the project, so this should work
        let expected = original.join("src");
        assert_eq!(env::current_dir().unwrap(), expected);
        env::set_current_dir(&original).unwrap();
    }

    // ─── Built-in: pwd test ─────────────────────────────────────────────────

    #[test]
    fn test_pwd() {
        let mut app = ShellApp::new();
        let mut cmd = Command::parse("pwd", &mut app);
        cmd.run(); // just ensure it doesn't panic
    }

    // ─── Built-in: exit test ────────────────────────────────────────────────

    #[test]
    fn test_exit() {
        let mut app = ShellApp::new();
        let mut cmd = Command::parse("exit", &mut app);
        cmd.run();
        // ShellApp.quit is private so we can only verify no panic
    }

    // ─── Built-in: version test ─────────────────────────────────────────────

    #[test]
    fn test_version() {
        ShellApp::version(); // just ensure no panic
    }

    // ─── External command tests ─────────────────────────────────────────────

    #[test]
    fn test_external_found() {
        let mut app = ShellApp::new();
        let mut cmd = Command::parse("echo external", &mut app);
        cmd.run();
        // External echo should produce output
        assert!(!cmd.output.is_empty());
    }

    #[test]
    fn test_external_not_found() {
        let mut app = ShellApp::new();
        let mut cmd = Command::parse("_shellrs_nonexistent_cmd_", &mut app);
        cmd.run();
        assert!(cmd.output.is_empty());
    }

    // ─── ShellApp tests ────────────────────────────────────────────────────

    #[test]
    fn test_shell_app_new() {
        let app = ShellApp::new();
        let _ = app;
    }

    #[test]
    fn test_shell_app_prompt() {
        let mut app = ShellApp::new();
        app.prompt("> ");
    }

    #[test]
    fn test_shell_app_quit() {
        let mut app = ShellApp::new();
        app.quit();
    }
}

fn environment() {
    let path = std::env::var("PATH").expect("Problem getting ENV PATH");

    println!("PATH = [{}]", path);
}

fn main() -> ExitCode {
    let mut app = modules::app::ShellApp::new();
    //app.prompt("# ");

    environment();
    app.run();

    ExitCode::SUCCESS
}
