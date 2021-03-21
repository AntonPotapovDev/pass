mod model;
mod command;
mod cmd_parser;

use std::env::Args;

use cmd_parser::parse_command;

const FILENAME: &str = ".data";

fn main() {
    let mut model = model::from_file(FILENAME).unwrap();

    match parse_args(std::env::args()) {
        Ok((cmd, args)) => {
            match parse_command(&cmd, args) {
                Ok(command) => command.execute(&mut model),
                Err(_) => println!("Unknown command"),
            }
        },
        Err(_) => println!("Usage: pass <command> [args]"),
    }

    model::serialize(model, FILENAME).unwrap();
}

fn parse_args(args: Args) -> Result<(String, Vec<String>), ()> {
    let mut args = args.collect::<Vec<String>>();

    if args.len() < 2 { return Err(()); }

    let cmd = args.remove(0);

    Ok((cmd, args))
}
