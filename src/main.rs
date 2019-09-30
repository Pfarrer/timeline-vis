extern crate zip;

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
    json::parse(
        archive
            .by_name("Takeout/Standortverlauf/Standortverlauf.json")
            .unwrap(),
    );

    return 0;
}
