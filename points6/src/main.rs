use std::io::{self, BufRead};

#[inline]
fn get_residents<'a, 'b, 'c>(list: &'a mut Vec<Option<Vec<usize>>>, idx1: usize, idx2: usize) -> (&'b mut Option<Vec<usize>>, &'c mut Option<Vec<usize>>) {
    let graphsize = list.len();
    assert!(idx1 != idx2);
    assert!(idx1 < graphsize);
    assert!(idx2 < graphsize);
    unsafe {
        (&mut *(list.get_unchecked_mut(idx1) as *mut Option<Vec<usize>>), &mut *(list.get_unchecked_mut(idx2) as *mut Option<Vec<usize>>))
    }
}

fn main() {
    let connections: Vec<usize>;
    let residents: Vec<Option<Vec<usize>>>;
    let a: usize;
    let b: usize;
    {
        let mut read = String::new();
        let stdin_ = io::stdin();
        let mut stdin = stdin_.lock();
        stdin.read_line(&mut read).unwrap();
        let graphsize: usize;
        let numedges: usize;
        {
            let mut iter = read.trim().split_whitespace();
            graphsize = iter.next().unwrap().parse::<usize>().unwrap();
            numedges = iter.next().unwrap().parse::<usize>().unwrap();
            a = iter.next().unwrap().parse::<usize>().unwrap();
            b = iter.next().unwrap().parse::<usize>().unwrap();
        }
        let mut tmpconnections = vec![0; graphsize];
        let mut neighborhoods: Vec<usize> = (0..graphsize).collect();
        let mut tmpresidents: Vec<Option<Vec<usize>>> = (0..graphsize).map(|_| None).collect();
        for _ in 0..numedges {
            let mut read = String::new();
            stdin.read_line(&mut read).unwrap();
            let mut iter = read.trim().split_whitespace().map(|x| x.parse::<usize>().unwrap());
            let idx1 = iter.next().unwrap() - 1;
            let idx2 = iter.next().unwrap() - 1;
            if idx1 == idx2 { panic!(); }
            tmpconnections[idx1] += 1;
            tmpconnections[idx2] += 1;
            let neigh1 = neighborhoods[idx1];
            let neigh2 = neighborhoods[idx2];
            if neigh1 != neigh2 {
                let (ress1, ress2) = get_residents(&mut tmpresidents, neigh1, neigh2);
                if let &mut Some(ref mut res1) = ress1 {
                    if let &mut Some(ref mut res2) = ress2 {
                        for &res in res2.iter() {
                            neighborhoods[res] = neigh1;
                        }
                        res1.append(res2);
                        tmpresidents[idx2] = None;
                    } else {
                        res1.push(idx2);
                        neighborhoods[idx2] = neigh1;
                    }
                } else {
                    if let &mut Some(ref mut res2) = ress2 {
                        res2.push(idx1);
                        neighborhoods[idx1] = neigh2;
                    } else {
                        let neighborhood = vec![idx1, idx2];
                        neighborhoods[idx1] = idx1;
                        neighborhoods[idx2] = idx1;
                        tmpresidents[idx1] = Some(neighborhood);
                    }
                }
            }
        }
        connections = tmpconnections;
        residents = tmpresidents;
    }
    let mut count = 0;
    for mut neighborhoodd in residents { if let Some(mut neighborhood) = neighborhoodd { if !neighborhood.is_empty() {
        for buffer in neighborhood.iter_mut() {
            *buffer = connections[*buffer];
        }
        let min = neighborhood.iter().min().unwrap() * a;
        if min != 0 {
            if b > 1 {
                for connection in neighborhood {
                    if connection > min {
                        count += 1;
                    }
                }
            } else {
                let max = *neighborhood.iter().max().unwrap(); // b == 1
                for connection in neighborhood {
                    if connection > min && connection < max {
                        count += 1;
                    }
                }
            }
        }
    } } }
    println!("{}", count);
}

