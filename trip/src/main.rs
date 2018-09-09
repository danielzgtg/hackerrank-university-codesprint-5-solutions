use std::io::{self, BufRead};

#[derive(Debug)]
struct Node {
    edges: Vec<usize>,
    revedges: Vec<usize>,
    dist: usize,
    name: char,
}

impl Node {
    fn new(name: char) -> Node {
        Node {
            edges: vec![],
            revedges: vec![],
            dist: 999999,
            name,
        }
    }
}

fn connect(nodes: &mut [Node], idx1: usize, idx2: usize) {
    if !nodes[idx1].edges.contains(&idx2) {
        nodes[idx1].edges.push(idx2);
        nodes[idx2].revedges.push(idx1);
    }
}

fn calc_distance(nodes: &mut [Node], f: usize) {
    nodes[f].dist = 0;
    let mut heatmap1: Vec<usize> = Vec::with_capacity(nodes.len());
    let mut heatmap2: Vec<usize> = Vec::with_capacity(nodes.len());
    let mut heatmap3: Vec<usize> = Vec::with_capacity(nodes.len());
    heatmap1.push(f);
    while !heatmap1.is_empty() {
        for &hotidx in heatmap1.iter() {
            let newdist: usize;
            {
                let me = &nodes[hotidx];
                newdist = me.dist + 1;
                heatmap3.clear();
                heatmap3.extend_from_slice(&me.revedges);
            }
            for &hotedge in heatmap3.iter() {
                let mut other = &mut nodes[hotedge];
                if other.dist > newdist {
                    other.dist = newdist;
                    if !heatmap2.contains(&hotedge) {
                        heatmap2.push(hotedge);
                    }
                }
            }
        }
        let mut heatmap4 = heatmap1;
        heatmap4.clear();
        heatmap1 = heatmap2;
        heatmap2 = heatmap4;
    }
}

fn recurse(nodes: &[Node], s: usize, prev: &str) -> String {
    let me = &nodes[s];
    let mut cur = String::from(prev);
    cur.push(me.name);
    let cur = cur;
    if me.dist == 0 {
        return cur;
    }
    let mut iter = me.edges.iter();
    let mut result = recurse(nodes, *iter.next().unwrap(), &cur);
    for &next in iter {
        let newresult = recurse(nodes, next, &cur);
        if newresult < result {
            result = newresult
        }
    }
    result
}

fn prune(nodes: &mut [Node]) {
    unsafe {
        for meidx in 0..nodes.iter().len() {
            let me = take_force(nodes, meidx);
            if me.dist == 0 { continue; }
            let expect = me.dist - 1;
            me.edges.retain(|&x| take_force(nodes, x).dist == expect);
            if !me.edges.is_empty() {
                let min = me.edges.iter().map(|&x| take_force(nodes, x).name).min().unwrap();
                me.edges.retain(|&x| take_force(nodes, x).name == min);
            }
        }
    }
}

#[inline]
unsafe fn take_force<'a>(nodes: &mut [Node], idx: usize) -> &'a mut Node {
    assert!(idx < nodes.len());
    &mut *(nodes.get_unchecked_mut(idx) as *mut Node)
}

fn main() {
    let nodes: Vec<Node>;
    let s: usize;
    let f: usize;
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
        }
        read.clear();
        stdin.read_line(&mut read).unwrap();
        let mut tmpnodes: Vec<Node>;
        {
            let names = read.trim();
            tmpnodes = names.chars().map(|x| Node::new(x)).collect();
        }
        assert!(graphsize == tmpnodes.len());
        for _ in 0..numedges {
            read.clear();
            stdin.read_line(&mut read).unwrap();
            let mut iter = read.trim().split_whitespace();
            let idx1 = iter.next().unwrap().parse::<usize>().unwrap() - 1;
            let idx2 = iter.next().unwrap().parse::<usize>().unwrap() - 1;
            connect(&mut tmpnodes, idx1, idx2);
        }
        read.clear();
        stdin.read_line(&mut read).unwrap();
        {
            let mut iter = read.trim().split_whitespace();
            s = iter.next().unwrap().parse::<usize>().unwrap() - 1;
            f = iter.next().unwrap().parse::<usize>().unwrap() - 1;
        }
        calc_distance(&mut tmpnodes, f);
        prune(&mut tmpnodes);
        nodes = tmpnodes;
    }
    if nodes[s].dist != 999999 {
        println!("{}", recurse(&nodes, s, ""));
    } else {
        println!("No way");
    }
}
