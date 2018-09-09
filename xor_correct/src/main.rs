use std::io::{self, BufRead};

fn main() {
    let k: usize;
    let mut arr: Vec<usize>;
    {
        let _stdin = io::stdin();
        let mut stdin = _stdin.lock();
        {
            let mut numstr = String::new();
            stdin.read_line(&mut numstr).unwrap();
            let mut iter = numstr.trim().split_whitespace().map(|x| x.parse::<usize>().unwrap());
            arr = Vec::with_capacity(iter.next().unwrap());
            k = iter.next().unwrap();
        }
        {
            let mut elems_str = String::new();
            stdin.read_line(&mut elems_str).unwrap();
            for elem in elems_str.trim().split_whitespace().map(|x| x.parse::<usize>().unwrap()) {
                arr.push(elem);
            }
        }
    }
    let arr = arr;
    let mut tmp: usize = 0;
    let mut halfs: Vec<usize> = Vec::with_capacity(arr.len() + 1);
    halfs.push(0);
    for e in arr.iter() {
        tmp ^= e;
        halfs.push(tmp);
    }
    let halfs = halfs;
    let mut result: usize = 0;
    for begin in 0..arr.len() {
        let beginhalf = halfs[begin];
        result += halfs.iter().skip(begin + 1).filter(|&endhalf| beginhalf ^ endhalf < k).count();
    }
    println!("{}", result);
}
