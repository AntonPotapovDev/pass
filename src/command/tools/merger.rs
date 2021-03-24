use std::io;
use std::io::prelude::*; 

use crate::context::{self, PassListModel};
use super::msg;

pub fn interactive_merge(new: PassListModel, old: &mut PassListModel) {
    match choose_way() {
        ResolveWay::Old => accept_old(new, old),
        ResolveWay::New => accept_new(new, old),
        ResolveWay::Merge => merge(new, old),
        ResolveWay::Abort => (),
    }
}

enum ResolveWay {
    Old,
    New,
    Merge,
    Abort,
}

fn choose_way() -> ResolveWay {
    let stdin = io::stdin();

    loop {
        print!("{} " ,msg::strings::CHOOSE_WAY);
        io::stdout().flush().unwrap();

        let mut buffer = String::new();
        stdin.read_line(&mut buffer).unwrap();

        match &buffer.trim().to_lowercase()[..] {
            "o" => return ResolveWay::Old,
            "n" => return ResolveWay::New,
            "m" => return ResolveWay::Merge,
            "a" => return ResolveWay::Abort,
            _ => (),
        }
    }
}

fn accept_old(new: PassListModel, old: &mut PassListModel) {
    new.into_iter().for_each(|(key, value)| {
        old.entry(key).or_insert(value);
    });
}

fn accept_new(new: PassListModel, old: &mut PassListModel) {
    context::merge_models(&new, old);
}

fn merge(new: PassListModel, old: &mut PassListModel) {
    println!("{}", msg::strings::MERGE_HELP);

    new.into_iter().for_each(|(key, new_value)| {
        match old.get(&key) {
            Some(old_value) => {
                let solved = ask_resolve(&key, old_value.clone(), new_value);
                old.insert(key, solved);
            },
            None => { old.insert(key, new_value); },
        }
    });
}

fn ask_resolve(key: &String, old_value: String, new_value: String) -> String {
    let stdin = io::stdin();

    loop {
        print!("{} (O/N): ", key);
        io::stdout().flush().unwrap();

        let mut buffer = String::new();
        stdin.read_line(&mut buffer).unwrap();

        match &buffer.trim().to_lowercase()[..] {
            "o" => return old_value,
            "n" => return new_value,
            _ => (),
        }
    }
}
