
use std::io;

fn main() {
    let len = read_num();
    let mut cases: Vec<usize> = Vec::with_capacity(len);
    for _ in 0..len {
        cases.push(read_num());
    }
    let cases = cases;
    for c in cases {
        println!("{}", (0.167964430778802 * (c as f64) - 1.4159140891407) as usize );
    }
}

fn read_num() -> usize {
    let mut numstr = String::new();
    io::stdin().read_line(&mut numstr).unwrap();
    numstr.trim().parse().unwrap()
}

