use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<i64>, Error> {
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
    let vec = read(File::open("input.txt")?)?;

    let mut sum = 0;
    for (pos, value) in vec[1..].iter().enumerate() {
        if value > &vec[pos] {
            sum += 1;            
        }
    }
    
    println!("How many measurements are larger than the previous measurement? {}", sum);

    let mut sums: i64 = 0;
    let first_window = &vec[0..3];
    let mut previous_sum: i64 = first_window.iter().sum();
    for (pos, _value) in vec[..vec.len()-2].iter().enumerate() {
        let window = &vec[pos..pos+3];
        let sum: i64 = window.iter().sum();
        if sum > previous_sum {
            sums += 1;
        }
        previous_sum = sum;
    }
    println!("How many sums are larger than the previous sum? {}", sums);

    Ok(())
}