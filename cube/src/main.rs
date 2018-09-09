
use std::io;

fn gen_high_primes() -> Vec<u64> {
    let mut result: Vec<u64> = Vec::with_capacity(1000000);
    'outer:
    for x in 3..1000000_u64 {
        if x & 1 == 0 { continue; }
        let sqrt = (x as f64).sqrt() as u64 + 1;
        for &y in result.iter() {
            if y > sqrt {
                break;
            }
            if x % y == 0 {
                continue 'outer;
            }
        }
        result.push(x);
    }
    result
}

fn main() {
    let primes = gen_high_primes();
    let cases = read_num() as usize;
    let mut result: Vec<usize> = Vec::with_capacity(cases);
    for _ in 0..cases {
        let high = read_num();
        let count: usize = (8..high+1).fold(0, |acc, elem| if cubeloving(elem, &primes) { acc + 1 } else { acc });
        result.push(count);
    }
    for r in result {
        println!("{}", r);
    }
}

fn read_num() -> u64 {
    let mut numstr = String::new();
    io::stdin().read_line(&mut numstr).unwrap();
    numstr.trim().parse().unwrap()
}

fn cubeloving(num: u64, primes: &[u64]) -> bool {
    let factored = factor(num, primes);
    let mut cur = 0;
    let mut count = 0;
    for x in factored {
        if cur != x {
            cur = x;
            count = 1;
            continue;
        }
        if count == 2 {
            return true;
        }
        count += 1;
    }
    false
}

fn factor(mut num: u64, primes: &[u64]) -> Vec<u64> {
    let mut result: Vec<u64> = vec![];
    if num < 2 {
        return result;
    }
    while num & 1 == 0 {
        num >>= 1;
        result.push(2);
    }
    let sqrthigh = sqrt_high(num);
    for &x in primes {
        if x > sqrthigh {
            break;
        }
        while num % x == 0 {
            result.push(x);
            num /= x;
        }
    }
    if num > 1 { result.push(num); }
    result.sort_unstable();
    result
}

fn sqrt_high(num: u64) -> u64 {
    let mut result = (num as f64).sqrt() as u64 + 1;
    while result * result < num {
        result += 1;
    }
    result
}
