use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);
    let mut v = vec![];
    for line in br.lines() {
        v.push(line?
            .trim()
            .parse()
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?);
    }
    Ok(v)
}

fn main() -> Result<(), Error> {

    let vec = read(File::open("day2.txt")?)?;

    let mut depth = 0;
    let mut horizontal = 0;

    let re = Regex::new(r"(.*) (.*)").unwrap();
    for elem in vec {
        for cap in re.captures_iter(&elem) {
            let direction: &str = &cap[1];
            let delta: i32 = cap[2].parse().unwrap_or(0);
            
            match direction {
                "up" => {depth-=delta}
                "down" => {depth+=delta}
                "forward" => {horizontal+=delta}
                &_ => {}
            }
            println!("{} {} {}", direction, delta, depth)
        }
    }
    println!("{}", horizontal * depth);

    Ok(())
}
