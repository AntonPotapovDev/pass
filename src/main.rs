mod context;
mod command;
mod impexp;

use std::env::{self, Args};

use command::{resolver::*, builders::CmdBuilder};
use context::Context;

const FILENAME: &str = ".data";

fn main() {
    match parse_args(std::env::args()) {
        Ok(ParseResult{cmd, args, path}) => match resolve_command(&cmd) {
            Ok(builder) => match builder.build(args) {
                Ok(command) => {
                    let mut context = Context::from_file(&path).unwrap();

                    command.execute(&mut context);

                    context.flush().unwrap();
                },
                Err(_) => command_usage(&cmd, builder)
            },
            Err(_) => unknown_command(&cmd),
        },
        Err(_) => help(),
    }
}

struct ParseResult {
    cmd: String,
    args: Vec<String>,
    path: String,
}

fn parse_args(mut args: Args) -> Result<ParseResult, ()> {
    args.next();

    let mut args = args.collect::<Vec<String>>();

    if args.len() < 1 { return Err(()); }

    let mut dir = env::current_exe().unwrap();
    dir.pop();
    dir.push(FILENAME);
    let path = String::from(dir.to_str().unwrap());

    let cmd = args.remove(0);

    Ok(ParseResult {
        cmd,
        args,
        path,
    })
}

fn help() {
    println!("Usage: pass <command> [args]");
    println!("Supported commands:");
    println!("  {:6} - add new password", CMD_ADD);
    println!("  {:6} - remove password", CMD_REMOVE);
    println!("  {:6} - update password", CMD_UPDATE);
    println!("  {:6} - show all keys", CMD_LIST);
    println!("  {:6} - show password by key", CMD_SHOW);
    println!("  {:6} - export encrypted passwords", CMD_EXPORT);
    println!("  {:6} - import encrypted passwords", CMD_IMPORT);
    println!("  {:6} - rename specified key", CMD_RENAME);
    println!("  {:6} - clear password list", CMD_CLEAR);
    println!("  {:6} - copy password to clipboard", CMD_COPY);
}

fn command_usage(cmd_name: &str, cmd: Box<dyn CmdBuilder>) {
    println!("Usage for \"{}\": {} {}", cmd_name, cmd_name, cmd.cmd_usage());
}

fn unknown_command(cmd: &str) {
    println!("Unknown command: \"{}\"", cmd)
}
