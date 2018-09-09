use std::io::{self, BufRead};

#[inline]
fn get_residents(list: &mut Vec<Vec<usize>>, newneigh: usize, oldneigh: usize) -> (&mut Vec<usize>, &mut Vec<usize>) {
    let graphsize = list.len();
    assert!(newneigh != oldneigh);
    assert!(newneigh < graphsize);
    assert!(oldneigh < graphsize);
    unsafe {
        (&mut *(list.get_unchecked_mut(newneigh) as *mut Vec<usize>), &mut *(list.get_unchecked_mut(oldneigh) as *mut Vec<usize>))
    }
}

fn main() {
    let connections: Vec<usize>;
    let residents: Vec<Vec<usize>>;
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
        let mut tmpresidents: Vec<Vec<usize>> = (0..graphsize).map(|x| vec![x]).collect();
        for _ in 0..numedges {
            let mut read = String::new();
            stdin.read_line(&mut read).unwrap();
            let mut iter = read.trim().split_whitespace().map(|x| x.parse::<usize>().unwrap());
            let idx1 = iter.next().unwrap() - 1;
            let idx2 = iter.next().unwrap() - 1;
            if idx1 == idx2 { panic!(); }
            tmpconnections[idx1] += 1;
            tmpconnections[idx2] += 1;
            let newneigh = neighborhoods[idx1];
            let oldneigh = neighborhoods[idx2];
            if newneigh != oldneigh {
                let (newresidents, oldresidents) = get_residents(&mut tmpresidents, newneigh, oldneigh);
                for resident in oldresidents.iter() {
                    neighborhoods[*resident] = newneigh;
                }
                newresidents.append(oldresidents);
            }
        }
        connections = tmpconnections;
        residents = tmpresidents;
    }
    let mut count = 0;
    for mut neighborhood in residents {
        if neighborhood.is_empty() {
            continue;
        }
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
    }
    println!("{}", count);
}

