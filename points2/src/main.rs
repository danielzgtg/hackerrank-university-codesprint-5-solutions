use std::io::{self, BufRead};
use std::rc::Rc;
use std::cell::RefCell;

//#[derive(Debug)]
struct Neighborhood {
    idxs: Vec<usize>,
}

impl Neighborhood {
    fn new(idx1: usize, idx2: usize) -> Neighborhood {
        Neighborhood {
            idxs: vec![idx1, idx2],
        }
    }
    fn eat(&mut self, nodes: &mut [Node], food: &mut Rc<RefCell<Neighborhood>>) {
        for &idx in food.borrow().idxs.iter() {
            nodes[idx].neighborhood = Some(food.clone());
            self.idxs.push(idx);
        }
    }
}

//#[derive(Debug)]
struct Node {
    connections: usize,
    neighborhood: Option<Rc<RefCell<Neighborhood>>>,
}

impl Node {
    fn new() -> Node {
        Node {
            connections: 0,
            neighborhood: None,
        }
    }
}

fn connect(nodes: &mut [Node], idx1: usize, idx2: usize, neighborhoods: &mut Vec<Rc<RefCell<Neighborhood>>>) {
    unsafe {
        let (node1, node2) = get_nodes(nodes, idx1, idx2);
        node1.connections += 1;
        node2.connections += 1;
        if let Some(ref mut neighborhood1) = node1.neighborhood {
            if let Some(ref mut neighborhood2) = node2.neighborhood {
                if !Rc::ptr_eq(neighborhood1, neighborhood2) {
                    neighborhood1.borrow_mut().eat(nodes, neighborhood2);
                }
            } else {
                neighborhood1.borrow_mut().idxs.push(idx2);
                node2.neighborhood = Some(neighborhood1.clone());
            }
        } else {
            if let Some(ref mut neighborhood2) = node2.neighborhood {
                neighborhood2.borrow_mut().idxs.push(idx1);
                node1.neighborhood = Some(neighborhood2.clone());
            } else {
                let neighborhood3 = Rc::new(RefCell::new(Neighborhood::new(idx1, idx2)));
                node1.neighborhood = Some(neighborhood3.clone());
                node2.neighborhood = Some(neighborhood3.clone());
                neighborhoods.push(neighborhood3);
            }
        }
    }
}

unsafe fn get_nodes<'a, 'b, 'c>(nodes: &'a mut [Node], idx1: usize, idx2: usize) -> (&'b mut Node, &'c mut Node){
    let graphsize = nodes.len();
    assert!(idx1 != idx2);
    assert!(idx1 < graphsize);
    assert!(idx2 < graphsize);
    (&mut *(nodes.get_unchecked_mut(idx1) as *mut Node), &mut *(nodes.get_unchecked_mut(idx2) as *mut Node))
}

fn solve(neighborhoods: Vec<Rc<RefCell<Neighborhood>>>, a: usize, b: usize, nodes: Vec<Node>) -> usize {
    //println!("{:#?}", nodes);
    let mut result = 0;
    for neighborhood in neighborhoods.iter().map(|x| x.borrow()) {
        if neighborhood.idxs.len() < 2 {
            continue;
        }
        let mut min = 9999999;
        let mut max = 0;
        for &idx in neighborhood.idxs.iter() {
            let c = nodes[idx].connections;
            if c > max {
                max = c;
            }
            if c < min {
                min = c;
            }
        }
        min *= a;
        max *= b;
        for &idx in neighborhood.idxs.iter() {
            let node = &nodes[idx];
            if node.connections > min && node.connections < max {
                result += 1;
            }
        }
    }
    result
}

fn main() {
    let nodes: Vec<Node>;
    let a: usize;
    let b: usize;
    let neighborhoods: Vec<Rc<RefCell<Neighborhood>>>;
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
        let mut tmpnodes: Vec<Node> = (0..graphsize).map(|_| Node::new()).collect();
        let mut tmpneighborhoods: Vec<Rc<RefCell<Neighborhood>>> = Vec::with_capacity(200);
        for _ in 0..numedges {
            let mut read = String::new();
            stdin.read_line(&mut read).unwrap();
            let mut iter = read.trim().split_whitespace().map(|x| x.parse::<usize>().unwrap());
            let idx1 = iter.next().unwrap() - 1;
            let idx2 = iter.next().unwrap() - 1;
            if idx1 == idx2 { continue; }
            connect(&mut tmpnodes, idx1, idx2, &mut tmpneighborhoods);
        }
        nodes = tmpnodes;
        neighborhoods = tmpneighborhoods;
    }
    loop {}
    println!("{}", solve(neighborhoods, a, b, nodes));
}

