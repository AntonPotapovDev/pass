use std::io::{self, BufRead, Write};
use std::fs::File;

pub type PassListModel = std::collections::HashMap::<String, String>;

pub fn from_file(filename: &str) -> Result<PassListModel, std::io::Error> {
    match File::open(filename) {
        Ok(file) => {
            let mut model = PassListModel::new();

            io::BufReader::new(file).lines().for_each(|l| {
                let (key, value) = pair_from_line(&l.unwrap());
                model.insert(key, value);
            });

            Ok(model)
        },
        Err(err) => Err(err),
    }
}

pub fn pair_from_line(l: &str) -> (String, String) {
    let mut key_value = l
        .split("\u{0}")
        .map(|x| String::from(x))
        .collect::<Vec<String>>();

    assert_eq!(key_value.len(), 2);

    let key = std::mem::replace(&mut key_value[0], String::new());
    let value = std::mem::replace(&mut key_value[1], String::new());

    (key, value)
}

pub fn serialize(model: PassListModel, filename: &str) -> Result<(), std::io::Error> {
    match File::create(filename) {
        Ok(mut file) => {
            let lines = model.iter()
                .map(|(key, value)| format!("{}\u{0}{}\n", key, value).into_bytes())
                .flatten()
                .collect::<Vec<u8>>();

            file.write(&lines)?;

            Ok(())
        },
        Err(err) => Err(err),
    }
}
