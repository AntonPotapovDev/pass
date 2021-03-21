mod model;
mod command;
mod cmd_parser;

use std::env::Args;

use cmd_parser::*;
use command::builders::CmdBuilder;

const FILENAME: &str = ".data";

fn main() {
    let mut model = model::from_file(FILENAME).unwrap();

    match parse_args(std::env::args()) {
        Ok((cmd, args)) =>  match resolve_command(&cmd) {
            Ok(builder) => match builder.build(args) {
                Ok(command) => command.execute(&mut model),
                Err(_) => command_usage(&cmd, builder)
            },
            Err(_) => unknown_command(&cmd),
        },
        Err(_) => help(),
    }

    model::serialize(model, FILENAME).unwrap();
}

fn parse_args(mut args: Args) -> Result<(String, Vec<String>), ()> {
    args.next();

    let mut args = args.collect::<Vec<String>>();

    if args.len() < 1 { return Err(()); }

    let cmd = args.remove(0);

    Ok((cmd, args))
}

fn help() {
    println!("Usage: pass <command> [args]");
    println!("Supported commands:");
    println!("  {:6} - add new password", CMD_ADD);
    println!("  {:6} - remove password", CMD_REMOVE);
    println!("  {:6} - update password", CMD_UPDATE);
    println!("  {:6} - show all keys", CMD_LIST);
    println!("  {:6} - show password by key", CMD_SHOW);
}

fn command_usage(cmd_name: &str, cmd: Box<dyn CmdBuilder>) {
    println!("Usage for \"{}\": {} {}", cmd_name, cmd_name, cmd.cmd_usage());
}

fn unknown_command(cmd: &str) {
    println!("Unknown command: \"{}\"", cmd)
}
