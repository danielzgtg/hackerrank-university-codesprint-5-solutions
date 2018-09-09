
use std::io::{self, BufRead};

fn gen_mods() -> Vec<(usize, usize)> {
    // Vec<(factor,next_multiple)>
    let mut result: Vec<(usize, usize)> = Vec::with_capacity(78497);
    result.push((2, 2));
    'outer:
    for x in 3..1000000_usize {
        if x & 1 == 0 { continue; }
        let sqrt = high_sqrt(x);
        for &(y, _) in result.iter() {
            if y > sqrt {
                break;
            }
            if x % y == 0 {
                continue 'outer;
            }
        }
        result.push((x, 0));
    }
    for x in result.iter_mut() {
        let y = x.0;
        let z = y*y*y;
        x.0 = z;
        x.1 = z;
    }
    result
}

struct Case {
    index: usize,
    upperbound: usize,
    count: usize,
}

impl Case {
    fn new(index: usize, upperbound: usize) -> Case {
        Case {
            index,
            upperbound,
            count: 0,
        }
    }
}

impl PartialEq for Case {
    fn eq(&self, rhs: &Case) -> bool {
        self.upperbound == rhs.upperbound
    }
}

impl Eq for Case {}

impl PartialOrd for Case {
    fn partial_cmp(&self, rhs: &Case) -> Option<std::cmp::Ordering> {
        self.upperbound.partial_cmp(&rhs.upperbound)
    }
}

impl Ord for Case {
    fn cmp(&self, rhs: &Case) -> std::cmp::Ordering {
        self.upperbound.cmp(&rhs.upperbound)
    }
}

#[inline]
fn high_sqrt(num: usize) -> usize {
    (num as f64).sqrt() as usize + 1
}

struct Page {
    base: usize,
    top: usize,
    table: [bool; 1048576],
}

impl Page {
    fn new(mods: &mut [(usize, usize)]) -> Page {
        let mut result = Page {
            base: 0,
            top: 1048576,
            table: [false; 1048576],
        };
        result.populate(mods);
        result
    }
    fn populate(&mut self, mods: &mut [(usize, usize)]) {
        // modd.0 : factor
        // modd.1 : next_multiple
        for modd in mods.iter_mut() {
            if modd.0 >= self.top {
                break;
            }
            while modd.1 < self.top {
                self.table[modd.1 & 1048575] = true;
                modd.1 += modd.0;
            }
        }
    }
    fn zero(&mut self) {
        for x in self.table.iter_mut() {
            *x = false;
        }
    }
    fn next(&mut self, mods: &mut [(usize, usize)]) {
        self.zero();
        self.base = self.top;
        self.top += 1048576;
        self.populate(mods);
    }
    fn get(&self, index: usize) -> bool {
        self.table[index & 1048575]
    }
}

fn main() {
    let mut mods = gen_mods();
    let mut page = Page::new(&mut mods);
    let _stdin = io::stdin();
    let mut stdin = _stdin.lock();
    let mut numstr = String::new();
    stdin.read_line(&mut numstr).unwrap();
    let len: usize = numstr.trim().parse().unwrap();
    let mut cases: Vec<Case> = Vec::with_capacity(len);
    {
        for index in 0..len {
            numstr.clear();
            stdin.read_line(&mut numstr).unwrap();
            let upperbound: usize = numstr.trim().parse().unwrap();
            cases.push(Case::new(index, upperbound));
        }
    }
    cases.sort_unstable();
    let mut count = 0;
    let mut number = 5;
    for case in cases.iter_mut() {
        let upperbound = case.upperbound;
        while number < upperbound {
            number += 1;
            if number >= page.top {
                page.next(&mut mods);
            }
            if page.get(number) {
                count += 1;
            }
        }
        case.count = count;
    }
    let mut results: Vec<usize> = vec![0; len];
    for case in cases {
        results[case.index] = case.count;
    }
    for result in results {
        println!("{}", result);
    }
}


