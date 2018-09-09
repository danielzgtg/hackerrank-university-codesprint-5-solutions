fn main() {
    println!("{:?}", gen_high_primes());
}

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
