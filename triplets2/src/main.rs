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
        let mut arrtmp = Vec::with_capacity(arrlen);
        {
            let mut splitstr = String::new();
            io::stdin().read_line(&mut splitstr).unwrap();
            arrtmp.extend(splitstr.trim().split_whitespace().map(|x| x.parse::<i64>().unwrap()));
        }
        arr = arrtmp;
    }
    println!("{}", solve(&arr));
}

fn solve(arr: &[i64]) -> usize {
    assert!(arr.len() >= 3);
    let mut counter: usize = 0;
    let mut idxs0: Vec<usize> = Vec::with_capacity(17);
    let mut idxs1: Vec<usize> = Vec::with_capacity(17);
    let mut idxs2: Vec<usize> = Vec::with_capacity(17);
    recurse(&mut idxs0, &mut idxs1, &mut idxs2, arr, 0, &mut counter);
    counter
}

fn recurse(idxs0: &mut Vec<usize>, idxs1: &mut Vec<usize>, idxs2: &mut Vec<usize>,
           arr: &[i64], level: usize, counter: &mut usize) {
    if level == arr.len() {
        if sum_good(idxs0, idxs1, idxs2, arr) {
            *counter += 1;
        }
        return;
    }
    let newlevel = level + 1;
    idxs0.push(level);
    recurse(idxs0, idxs1, idxs2, arr, newlevel, counter);
    idxs0.pop();
    idxs1.push(level);
    recurse(idxs0, idxs1, idxs2, arr, newlevel, counter);
    idxs1.pop();
    idxs2.push(level);
    recurse(idxs0, idxs1, idxs2, arr, newlevel, counter);
    idxs2.pop();
}

#[inline]
fn sum_one(idxs: &[usize], arr: &[i64]) -> i64 {
    idxs.iter().map(|&x| arr[x]).sum()
}

#[inline]
fn sum_good(idxs0: &[usize], idxs1: &[usize], idxs2: &[usize], arr: &[i64]) -> bool {
    if idxs0.len() == 0 || idxs1.len() == 0 || idxs2.len() == 0 { return false; }
    let sum0 = sum_one(idxs0, arr);
    let sum1 = sum_one(idxs1, arr);
    let sum2 = sum_one(idxs2, arr);
    let result = sum0 == sum1 && sum0 == sum2;
    result
}
