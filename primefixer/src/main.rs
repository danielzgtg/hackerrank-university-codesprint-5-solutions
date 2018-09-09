
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("primes.in").unwrap();
    println!("pub static PRIMES: &'static [u32] = &[");
    for line in BufReader::new(file).lines() {
        print!("{}", line.unwrap().trim());
        print!(",");
    }
    println!("];");
}
