
use std::io;

fn gen_high_primes() -> Vec<usize> {
    let mut result: Vec<usize> = Vec::with_capacity(78497);
    'outer:
    for x in 3..1000000_usize {
        if x & 1 == 0 { continue; }
        let sqrt = (x as f64).sqrt() as usize + 1;
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
    let len = read_num();
    let mut cases: Vec<usize> = Vec::with_capacity(len);
    for _ in 0..len {
        cases.push(read_num());
    }
    let cases = cases;
    let max = *cases.iter().max().unwrap() + 1;
    let mut values: Vec<bool> = vec![false; max];
    mark_values(&mut values, &primes, max);
    let values = values;
    let mut counts = Vec::with_capacity(len);
    {
        let mut acc = 0;
        for &v in values.iter() {
            if v {
                acc += 1;
            }
            counts.push(acc);
        }
    }
    let counts = counts;
    //println!("{:?}", counts);
    for c in cases {
        println!("{}", counts[c]);
    }
}

fn mark_values(values: &mut [bool], primes: &[usize], max: usize) {
    let mut t = 0;
    'outer:
    loop {
        t += 2;
        let t3 = t * t * t;
        if t3 > max {
            break;
        }
        for y in 1.. {
            let value = t3 * y;
            if value > max {
                continue 'outer;
            }
            values[value] = true;
        }
    }
    'outer2:
    for x in primes {
        let base = x * x * x;
        if base > max {
            break;
        }
        for y in 1.. {
            let value = base * y;
            if value > max {
                continue 'outer2;
            }
            values[value] = true;
        }
    }
}

fn read_num() -> usize {
    let mut numstr = String::new();
    io::stdin().read_line(&mut numstr).unwrap();
    numstr.trim().parse().unwrap()
}

