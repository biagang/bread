mod config;
use bread_cli as bread;
use config::Config;

fn main() {
    let (mut input, mut output) = Config::new().unwrap().into();
    if let Err(e) = bread::convert(input.as_mut(), output.as_mut()) {
        eprintln!("{e:?}");
    }
}
