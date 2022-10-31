mod config;
use bread_cli as bread;
use config::Config;

fn main() {
    let (input, output) = Config::new().unwrap().to_io();
    if let Err(e) = bread::convert(input, output) {
        eprintln!("{e:?}");
    }
}
