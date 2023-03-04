use bpaf::*;

mod hash;
mod storage;

use storage::Storage;

fn url_check(url: &String) -> bool {
    // I did not want to add a new package for url parsing. SO I just did a simple check. May it works fine.
    let url = url.as_str();
    let url = url.trim();
    let url = url.trim_end_matches('/');

    if url.starts_with("http://") || url.starts_with("https://") {
        return true;
    }

    false
}

impl Op {
    fn handle(&self) {
        let storage = Storage::new();

        match self {
            Op::Get { uid } => self.get(uid, &storage),
            Op::Save { url } => self.save(url, &storage),
            Op::List => self.list(&storage),
            Op::Delete { uid } => self.delete(uid, &storage),
        }
    }

    fn get(&self, uid: &str, storage: &Storage) {
        match storage.get(uid) {
            Ok(url) => println!("{}", url),
            Err(e) => println!("{}", storage::format_error(e)),
        }
    }

    fn save(&self, url: &str, storage: &Storage) {
        match storage.save(url) {
            Ok(uid) => println!("{}", uid),
            Err(e) => println!("{}", storage::format_error(e)),
        }
    }

    fn list(&self, storage: &Storage) {
        match storage.list() {
            Ok(list) => {
                for (key, value) in list {
                    println!("{}: {}", key, value);
                }
            }
            Err(e) => println!("{}", storage::format_error(e)),
        }
    }

    fn delete(&self, uid: &str, storage: &Storage) {
        match storage.delete(uid) {
            Ok(_) => println!("Deleted"),
            Err(e) => println!("{}", storage::format_error(e)),
        }
    }
}

#[derive(Clone, Debug, Bpaf)]
enum Op {
    #[bpaf(command)]
    #[bpaf(short('g'))]
    #[bpaf(long("get"))]
    /// Get an url by uid
    Get {
        /// help here
        #[bpaf(positional)]
        uid: String,
    },
    #[bpaf(command)]
    #[bpaf(short('s'))]
    #[bpaf(long("save"))]
    /// Save an url
    Save {
        /// help here
        #[bpaf(positional)]
        #[bpaf(guard(url_check, "Invalid url"))]
        url: String,
    },
    #[bpaf(command)]
    #[bpaf(short('d'))]
    #[bpaf(long("delete"))]
    /// Delete an url by uid
    Delete {
        #[bpaf(positional)]
        uid: String,
    },

    #[bpaf(command)]
    #[bpaf(short('l'))]
    #[bpaf(long("list"))]
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
