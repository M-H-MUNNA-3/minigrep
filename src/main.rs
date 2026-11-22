use minigrep::{config::Config, reader as rd};

use std::env::args;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let a: Vec<String> = args().collect();
    if a.len() < 3 {
        usage();
        std::process::exit(1);
    }
    let cfg = Config::new(&a[2], &a[1]);

    let _ = rd::read_file(&cfg)?;

    Ok(())
}
fn usage() {
    println!("{0} <Query> <File> ", "minigrep");
}
