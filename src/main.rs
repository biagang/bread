mod config;
use bread_cli as bread;
use config::Config;

fn main() {
    let config = Config::new().unwrap();
    let out_fmt = config.out_format();
    let (input, output) = config.into();
    if let Err(e) = bread::convert(input, output, out_fmt) {
        eprintln!("{e:?}");
    }
}
