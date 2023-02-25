use bpaf::*;
use std::str::FromStr;

impl Op {
    fn handle(&self) {
        match self {
            Op::Get { uid } => self.get(uid),
            Op::Save { url } => self.save(url),
        }
    }

    fn get(&self, uid: &str) {
        todo!("Save {}", uid)
    }

    fn save(&self, url: &str) {
        todo!("Save {}", url)
    }
}

#[derive(Clone, Debug, Bpaf)]
enum Op {
    #[bpaf(command)]
    /// Get something
    Get {
        /// help here
        #[bpaf(positional)]
        uid: String,
    },
    #[bpaf(command)]
    /// Save something
    Save {
        /// help here
        #[bpaf(positional)]
        url: String,
    },
}

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options, version)]
/// description
///
///
/// more description, note empty lines
///
///
/// even more
struct Input {
    /// for example
    verbose: bool,
    #[bpaf(external)]
    op: Op,
}

fn main() {
    let opts = input().run();

    todo!("{:?}", opts);
}
