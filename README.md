# Shorter

_You can always be shorter._

Shorter is a simple URL shortener written in Rust. It stores the urls in-memory or in a file.

I am using bpaf to parse the command line arguments and some essential features to deliever the good cli experience.

## How it works?

Shorter uses the md5 hash of the long url to generate a unique id. The id is then converted to base62 and the first 7 characters are used as the short url.

It is unlikely that two urls will have the same hash, but if it happens, the short url will be the same. In that case, the short url will be overwritten.

_I will add a feature to handle collisions in the future._

## Usage

```
Usage: -s UID -g UID -d UID [-l] [-f FILE]

Available options:
    -s, --save <UID>    Save a url
    -g, --get <UID>     Get a url
    -d, --delete <UID>  Delete a url
    -l, --list          List all urls
    -f, --file <FILE>   File to save to
    -h, --help          Prints help information
```

## Build

```
cargo build --release
```

## Test

```
cargo test
```

### To myself

- pathbuf: todo!
- closure: todo!
- serde: todo!
- construct! todo!

### Todo

- [ ] Add a feature to handle collisions
- [ ] Detailed error handling
- [ ] Command descriptions

## License

This project is licensed under the terms of the MIT license.
