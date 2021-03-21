use crate::model::PassListModel;

pub trait Command {
    fn execute(&self, model: &mut PassListModel);
}

pub struct List;

impl Command for List {
    fn execute(&self, model: &mut PassListModel) {
        model.iter().for_each(|(key, _value)| println!("{}", key));
    }
}

pub struct Add {
    pub key: String,
    pub pass: String,
}

impl Command for Add {
    fn execute(&self, model: &mut PassListModel) {
        if model.contains_key(&self.key) {
            println!("Password for the given key is already exist");
        } else {
            model.insert(self.key.clone(), self.pass.clone());
        }
    }
}

pub struct Remove {
    pub key: String,
}

impl Command for Remove {
    fn execute(&self, model: &mut PassListModel) {
        if model.contains_key(&self.key) {
            model.remove(&self.key);
        } else {
            println!("No passwords for that key");
        }
    }
}

pub struct Update {
    pub key: String,
    pub pass: String,
}

impl Command for Update {
    fn execute(&self, model: &mut PassListModel) {
        if model.contains_key(&self.key) {
            model.insert(self.key.clone(), self.pass.clone());
        } else {
            println!("No passwords for that key");
        }
    }
}
