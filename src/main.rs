use serde::{Serialize, Deserialize};
use std::env;
use std::fs;
use std::process;

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct Message {
    userName: String,
    content: String
}

fn main() {
    let mut args = env::args();
    let _ = args.next();
    let path = args.next()
        .unwrap_or_else(|| {
            eprintln!("Incorrect argument count");
            process::exit(1);
        });
    let pretty_arg = args.next();
    let mut pretty = false;
    if let Some(pretty_arg) = pretty_arg {
        if pretty_arg.eq("pretty") {
            pretty = true;
        }
    }

    let raw_data = fs::read_to_string(path)
        .unwrap_or_else(|_| {
            eprintln!("Invalid file path");
            process::exit(1);
        });

    let messages: Vec<Message> = serde_json::from_str(raw_data.as_str()).unwrap();

    if pretty {
        fs::write("parsed.json", serde_json::to_string_pretty(&messages).unwrap())
            .unwrap_or_else(|_| {
                eprintln!("Couldn't write to file");
                process::exit(1);
            });
    } else {
        fs::write("parsed.json", serde_json::to_string(&messages).unwrap())
            .unwrap_or_else(|_| {
                eprintln!("Couldn't write to file");
                process::exit(1);
            });
    }
}
