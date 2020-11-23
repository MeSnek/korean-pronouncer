use std::{collections::HashMap, process::exit};

pub mod hash;
mod output;

fn main() {
    let mut hashmap: HashMap<String, hash::HangulInfo> = HashMap::new();
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    let mut params: String = String::new();
    

    if args.is_empty() {
        output::help();
        exit(1);
    }
    if args[0].starts_with('-') {
        params = args.remove(0);
    }

    for elem in args.iter() {
        let hangul = String::from(elem);
        let hangul = &hangul[..];
        hash::fill_hashmap(&hangul, &mut hashmap);
    }
    let order = args.clone();
    output::output(params, &mut hashmap, order);
}