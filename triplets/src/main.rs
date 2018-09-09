use std::io;

fn main() {
    let arr: Vec<i64>;
    {
        let arrlen: usize;
        {
            let mut numstr = String::new();
            io::stdin().read_line(&mut numstr).unwrap();
            arrlen = numstr.trim().parse().unwrap();
        }
        if arrlen > 9 {
            println!("0");
            return;
        }
        let mut arrtmp = Vec::with_capacity(arrlen);
        {
            let mut splitstr = String::new();
            io::stdin().read_line(&mut splitstr).unwrap();
            arrtmp.extend(splitstr.trim().split_whitespace().map(|x| x.parse::<i64>().unwrap()));
        }
        arr = arrtmp;
    }
    solve(&arr);
}

fn solve(arr: &[i64]) {
    println!("{}", gen_arrays(arr));
}

fn gen_arrays(arr: &[i64]) -> usize {
    let len = arr.len();
    let indices = gen_indices(len);
    let mut result = Vec::new();
    for p in &indices {
        for q in &indices {
            for r in &indices {
                if unique(p, q, r) && full (p, q, r, len) && sum_good(p, q, r, arr) {
                    result.push((p, q, r));
                }
            }
        }
    }
    //println!("{:#?}", result);
    result.len()
}

fn gen_indices(len: usize) -> Vec<(usize, usize, usize)> {
    let mut result = vec![];
    for x in 0..len {
        for y in x..len {
            for z in y..len {
                let elem = (x, y, z);
                result.push(elem);
            }
        }
    }
    result
}

fn unique(idx1: &(usize, usize, usize), idx2: &(usize, usize, usize), idx3: &(usize, usize, usize)) -> bool {
    // idx1, idx2
    idx1.0 != idx2.0 && idx1.0 != idx2.1 && idx1.0 != idx2.2 &&
    idx1.1 != idx2.0 && idx1.1 != idx2.1 && idx1.1 != idx2.2 &&
    idx1.2 != idx2.0 && idx1.2 != idx2.1 && idx1.2 != idx2.2 &&
    // idx1, idx3
    idx1.0 != idx3.0 && idx1.0 != idx3.1 && idx1.0 != idx3.2 &&
    idx1.1 != idx3.0 && idx1.1 != idx3.1 && idx1.1 != idx3.2 &&
    idx1.2 != idx3.0 && idx1.2 != idx3.1 && idx1.2 != idx3.2 &&
    // idx3, idx2
    idx3.0 != idx2.0 && idx3.0 != idx2.1 && idx3.0 != idx2.2 &&
    idx3.1 != idx2.0 && idx3.1 != idx2.1 && idx3.1 != idx2.2 &&
    idx3.2 != idx2.0 && idx3.2 != idx2.1 && idx3.2 != idx2.2
}

fn full(idx1: &(usize, usize, usize), idx2: &(usize, usize, usize), idx3: &(usize, usize, usize), len: usize) -> bool {
    for i in 0..len {
        if !(
            idx1.0 == i || idx1.1 == i || idx1.2 == i ||
            idx2.0 == i || idx2.1 == i || idx2.2 == i ||
            idx3.0 == i || idx3.1 == i || idx3.2 == i
        ) {
            return false
        }
    }
    true
}

fn sum_one(idx: &(usize, usize, usize), arr: &[i64]) -> i64 {
    arr[idx.0] + arr[idx.1] + arr[idx.2]
}

fn sum_good(idx1: &(usize, usize, usize), idx2: &(usize, usize, usize), idx3: &(usize, usize, usize), arr: &[i64]) -> bool {
    let sum = sum_one(idx1, arr);
    sum == sum_one(idx2, arr) && sum == sum_one(idx3, arr)
}
