use std::io::{self, BufRead};

#[derive(Debug)]
struct Node {
    adj: Vec<bool>,
    reach: Vec<bool>,
    adjnum: usize,
}

impl Node {
    fn new(idx: usize, graphsize: usize) -> Node {
        let mut result = Node {
            adj: (0..graphsize).map(|_| false).collect(),
            reach: (0..graphsize).map(|_| false).collect(),
            adjnum: 0,
        };
        result.reach[idx] = true;
        result
    }

    fn connect(nodes: &mut Vec<Node>, idx1: usize, idx2: usize) {
        let (node1, node2) = get_nodes(nodes, idx1, idx2);
        node1.adj[idx2] = true;
        node1.reach[idx2] = true;
        node2.adj[idx1] = true;
        node2.reach[idx1] = true;
    }

    fn reachables<'a>(&self, nodes: &'a [Node], buf: &mut Vec<&'a Node>) {
        for (a, &b) in self.reach.iter().enumerate() {
            if b {
                buf.push(&nodes[a]);
            }
        }
    }
}

fn get_nodes(nodes: &mut [Node], idx1: usize, idx2: usize) -> (&mut Node, &mut Node){
    let graphsize = nodes.len();
    assert!(idx1 != idx2);
    assert!(idx1 < graphsize);
    assert!(idx2 < graphsize);
    unsafe {
        (&mut *(nodes.get_unchecked_mut(idx1) as *mut Node), &mut *(nodes.get_unchecked_mut(idx2) as *mut Node))
    }
}

fn populate_reach(nodes: &mut [Node]) {
    let mut heatmap1: Vec<usize> = (0..nodes.len()).collect();
    let mut heatmap2: Vec<usize> = Vec::with_capacity(nodes.len());
    let mut heatmap4: Vec<(usize, usize)> = vec![];

    while !heatmap1.is_empty() {
        for &x in heatmap1.iter() {
            for (a, y) in nodes[x].reach.iter().enumerate() {
                if *y {
                    let other = &nodes[a];
                    for (b, z) in nodes[x].reach.iter().enumerate() {
                        if *z && !other.reach[b] {
                            heatmap4.push((a, b));
                            if !heatmap2.contains(&b) {
                                heatmap2.push(b);
                            }
                        }
                    }
                }
            }
            for &(a, b) in heatmap4.iter() {
                nodes[a].reach[b] = true;
            }
            heatmap4.clear();
        }
        let mut heatmap3 = heatmap1;
        heatmap1 = heatmap2;
        heatmap3.clear();
        heatmap2 = heatmap3;
    }

    for n in nodes {
        n.adjnum = n.adj.iter().filter(|x| **x).count();
    }
}

fn main() {
    let nodes: Vec<Node>;
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
        let mut tmpnodes: Vec<Node> = (0..graphsize).map(|x| Node::new(x, graphsize)).collect();
        for _ in 0..numedges {
            let mut read = String::new();
            stdin.read_line(&mut read).unwrap();
            let mut iter = read.trim().split_whitespace().map(|x| x.parse::<usize>().unwrap());
            let idx1 = iter.next().unwrap() - 1;
            let idx2 = iter.next().unwrap() - 1;
            if idx1 == idx2 { continue; }
            Node::connect(&mut tmpnodes, idx1, idx2);
        }
        populate_reach(&mut tmpnodes);
        nodes = tmpnodes;
    }
    println!("{}", solve(&nodes, a, b));
}

fn solve(nodes: &[Node], a: usize, b: usize) -> usize {
    let (min, ml, mh, max) = bounds(nodes);
    if min * a >= mh || max * b <= ml {
        return 0;
    }
    let mut buf: Vec<&Node> = vec![];
    let mut goodnodes: usize = 0;
    //for (d, x) in nodes.iter().enumerate() {
    for x in nodes {
        let adjnum = x.adjnum;
        if b > 1 && adjnum > 1 {
            'outer1:
            for (m, &y) in x.reach.iter().enumerate() {
                if y {
                    buf.clear();
                    let other = &nodes[m];
                    other.reachables(nodes, &mut buf);
                    for i in 0..buf.len() {
                        if nodes[i].adjnum * a < adjnum {
                            //println!("good {} {} {}", i, d, d);
                            goodnodes += 1;
                            break 'outer1;
                        }
                    }
                }
            }
        } else {
            'outer2:
            for (m, &y) in x.reach.iter().enumerate() {
                if y {
                    buf.clear();
                    let other = &nodes[m];
                    other.reachables(nodes, &mut buf);
                    for i in 0..buf.len() {
                        for j in i + 1..buf.len() {
                            if nodes[i].adjnum * a < adjnum && nodes[j].adjnum * b > adjnum {
                                //println!("good {} {} {}", i, d, j);
                                goodnodes += 1;
                                break 'outer2;
                            }
                        }
                    }
                }
            }
        }
    }
    //for node in nodes {
    //    println!("{:?}", node);
    //}
    goodnodes
}

fn bounds(nodes: &[Node]) -> (usize, usize, usize, usize) {
    let mut adjnums: Vec<usize> = nodes.iter().map(|x| x.adjnum).collect();
    adjnums.sort_unstable();
    let min = *adjnums.first().unwrap();
    let max = *adjnums.last().unwrap();
    for x in adjnums {
        if x > min && x < max {
            return (min, x, x, max);
        }
    }
    (min, min, max, max)
}



