use bpaf::*;
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Op {
    Get(String),
    Save(String),
}

// TODO: i did not understand why this is needed
impl FromStr for Op {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(2, ' ');
        let op = parts.next().unwrap();
        let arg = parts.next().unwrap();

        match op {
            "get" => Ok(Op::Get(arg.to_string())),
            "save" => Ok(Op::Save(arg.to_string())),
            _ => Err(format!("Unknown operation: {}", op)),
        }
    }
}

impl Op {
    fn handle(&self) {
        match self {
            Op::Get(uid) => self.get(uid),
            Op::Save(url) => self.save(url),
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
#[bpaf(options, version)]
struct Input {
    op: Op,
}

fn init_ops() -> OptionParser<Input> {
    let get = short('g')
        .long("get")
        .help("Get a shortened URL by UID")
        .argument::<String>("UID")
        .map(Op::Get);

    let op = construct!([get]);

    construct!(Input { op })
        .to_options()
        .descr("Short the given url!")
}

fn main() {
    let opts = init_ops().run();

    opts.op.handle();
}
