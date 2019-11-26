#[macro_use]
extern crate elastic_derive;
#[macro_use]
extern crate serde_derive;

extern crate elastic;
extern crate serde_json;

mod gtimeline;
mod json;
mod model;
mod utils;
mod es;

use std::fs;

fn main() {
    std::process::exit(real_main());
}

fn real_main() -> i32 {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return 1;
    }
    let fname = std::path::Path::new(&*args[1]);
    let file = fs::File::open(&fname).unwrap();

    let mut archive = zip::ZipArchive::new(file).unwrap();
    let json_tokenizer = json::JsonTokenizer::new(
        archive
            .by_name("Takeout/Standortverlauf/Standortverlauf.json")
            .unwrap(),
    );
    let location_iterator = gtimeline::parse_locations(Box::new(json_tokenizer));
    es::index(location_iterator.take(100000));

    return 0;
}
