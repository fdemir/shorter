use std::path::PathBuf;

use bpaf::*;
use url::Url;

#[derive(Clone, Debug, Bpaf)]
#[bpaf(options, version)]
struct Input {
    save: Url,
    get: String,
    delete: String,
    list: bool,
    file: Option<PathBuf>,
}

fn opts() -> OptionParser<Input> {
    let save = short('s')
        .long("save")
        .help("Save a url")
        .argument::<String>("UID")
        .parse(|url| Url::parse(&url).map_err(|e| e.to_string()));

    let get = short('g')
        .long("get")
        .help("Get a url")
        .argument::<String>("UID");

    let delete = short('d')
        .long("delete")
        .help("Delete a url")
        .argument::<String>("UID");

    let list = short('l').long("list").help("List all urls").switch();

    let file = short('f')
        .long("file")
        .help("File to save to")
        .argument::<String>("FILE")
        .parse(|file| {
            let path = PathBuf::from(file);
            if path.exists() {
                Ok(path)
            } else {
                // TODO: print error
                Err(format!("File does not exist"))
            }
        })
        .optional();

    construct!(Input {
        save,
        get,
        delete,
        list,
        file
    })
    .to_options()
    .descr("This is a test program.")
}

fn main() {
    let opts = opts().run();

    // TODO: implement...
}
