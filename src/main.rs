#[macro_use]
extern crate maplit;

mod gtimeline;
mod json;

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
    let location_iterator = gtimeline::parse(Box::new(json_tokenizer));

    location_iterator
        .take(1000)
        .for_each(|t| println!("{:?}", t));

    return 0;
}
