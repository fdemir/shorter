use bpaf::*;

mod hash;
mod storage;

use storage::Storage;

impl Op {
    fn handle(&self) {
        match self {
            Op::Get { uid } => self.get(uid),
            Op::Save { url } => self.save(url),
            Op::List => self.list(),
        }
    }

    fn get(&self, uid: &str) {
        let storage = Storage::new();

        match storage.get(uid) {
            Ok(url) => println!("{}", url),
            Err(e) => println!("{}", storage::format_error(e)),
        }
    }

    fn save(&self, url: &str) {
        let storage = storage::Storage::new();

        match storage.save(url) {
            Ok(uid) => println!("{}", uid),
            Err(e) => println!("{}", storage::format_error(e)),
        }
    }

    fn list(&self) {
        let storage = storage::Storage::new();

        match storage.list() {
            Ok(list) => {
                for (key, value) in list {
                    println!("{}: {}", key, value);
                }
            }
            Err(e) => println!("{}", storage::format_error(e)),
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
