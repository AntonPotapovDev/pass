use std::io::{self, Write, Read};
use std::fs::File;

pub type PassListModel = std::collections::HashMap::<String, String>;

pub struct Context {
    pub model: PassListModel,
    pub data_file_path: String,
}

impl Context {
    pub fn from_file(filename: &str) -> Result<Context, ()> {
        match File::open(filename) {
            Ok(mut file) => {
                let mut buff = String::new();
                file.read_to_string(&mut buff).unwrap();

                match model_from_string(buff) {
                    Ok(model) => Ok(Context {
                        model,
                        data_file_path: String::from(filename),
                    }),
                    Err(_) => Err(()) 
                }
            },
            Err(_) => Err(()),
        }
    }

    pub fn flush(self) -> Result<(), io::Error> {
        match File::create(self.data_file_path) {
            Ok(mut file) => {
                let lines = self.model.iter()
                    .map(|(key, value)| format!("{}\u{0}{}\n", key, value).into_bytes())
                    .flatten()
                    .collect::<Vec<u8>>();
    
                file.write(&lines)?;
    
                Ok(())
            },
            Err(err) => Err(err),
        }
    }
}

pub fn model_from_string(s: String) -> Result<PassListModel, ()> {
    let lines = s
        .split("\n")
        .map(|x| String::from(x))
        .filter(|x| !x.is_empty())
        .collect::<Vec<String>>();

    let mut model = PassListModel::new();

    for line in lines {
        let mut key_value = line
            .split("\u{0}")
            .map(|x| String::from(x))
            .collect::<Vec<String>>();

        if key_value.len() != 2 {
            return Err(());
        }

        let key = std::mem::replace(&mut key_value[0], String::new());
        let value = std::mem::replace(&mut key_value[1], String::new());

        model.insert(key, value);
    }

    Ok(model)
}

pub fn merge_models(f: PassListModel, s: &mut PassListModel) -> Result<(), Vec<String>> {
    let collisions = f.iter()
        .filter(|(key, _value)| s.contains_key(*key))
        .map(|(key, _value)| key.clone())
        .collect::<Vec<String>>();

    match collisions.len() > 0 {
        true => Err(collisions),
        false => {
            f.into_iter().for_each(|(key, value)| { s.insert(key, value); });
            Ok(())
        },
    }
}
