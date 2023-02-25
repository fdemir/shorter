use std::{collections::HashMap, fs::read_to_string};

use bpaf::*;

mod hash;

impl Op {
    fn handle(&self) {
        match self {
            Op::Get { uid } => self.get(uid),
            Op::Save { url } => self.save(url),
            Op::List => self.list(),
        }
    }

    fn get(&self, uid: &str) {
        let storage_file = "storage.json";
        let working_dir = std::env::current_dir().unwrap();
        let storage_path = working_dir.join(storage_file);

        let storage_file_exists = storage_path.exists();

        if storage_file_exists {
            let file = read_to_string(storage_file).unwrap();
            let value: HashMap<String, String> =
                serde_json::from_str(&file).expect("Json was not well formatted!");

            match value.get(uid) {
                Some(url) => println!("{}", url),
                None => println!("Not found"),
            }
        } else {
            println!("No storage file found");
        }
    }

    fn save(&self, url: &str) {
        let short_val = hash::gen(url);

        let storage_file = "storage.json";
        let working_dir = std::env::current_dir().unwrap();
        let storage_path = working_dir.join(storage_file);

        let storage_file_exists = storage_path.exists();

        if storage_file_exists {
            let file = read_to_string(storage_file).unwrap();
            let mut value: HashMap<String, String> =
                serde_json::from_str(&file).expect("Json was not well formatted!");

            match value.get(&short_val) {
                Some(_) => println!("Exist: {}", short_val),
                None => {
                    value.insert(short_val, url.to_string());
                    let json = serde_json::to_string(&value).unwrap();
                    std::fs::write(storage_file, json).unwrap();
                }
            }
        } else {
            let mut value: HashMap<String, String> = HashMap::new();
            value.insert(short_val, url.to_string());
            let json = serde_json::to_string(&value).unwrap();
            std::fs::write(storage_file, json).unwrap();
        }
    }

    fn list(&self) {
        let storage_file = "storage.json";
        let working_dir = std::env::current_dir().unwrap();
        let storage_path = working_dir.join(storage_file);

        let storage_file_exists = storage_path.exists();

        if storage_file_exists {
            let file = read_to_string(storage_file).unwrap();
            let value: HashMap<String, String> =
                serde_json::from_str(&file).expect("Json was not well formatted!");

            for (key, value) in value.iter() {
                println!("{}: {}", key, value);
            }
        } else {
            println!("No storage file found");
        }
    }
}

#[derive(Clone, Debug, Bpaf)]
enum Op {
    #[bpaf(command)]
    /// Get an url by uid
    Get {
        /// help here
        #[bpaf(positional)]
        uid: String,
    },
    #[bpaf(command)]
    /// Save an url
    Save {
        /// help here
        #[bpaf(positional)]
        url: String,
    },

    #[bpaf(command)]
    /// List all saved urls
    List,
}

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options, version)]
// TODO: Put a description here
struct Input {
    #[bpaf(external)]
    op: Op,
}

fn main() {
    let opts = input().run();

    opts.op.handle();
}
