use crate::config::{Config};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_file(config: &Config)->Result<(), std::io::Error>{

    let file=File::open(config.path)?;
    let reader=BufReader::new(file);

    for (index, l) in reader.lines().enumerate(){
        let line=l?;
        if does_match_exists(&line, &config.query) {
            println!("{index}: {}", line);
        }
    }

    Ok(())
}
fn does_match_exists(context: &str, query: &str)->bool{
    context.to_lowercase().contains(&query.to_lowercase())
}
