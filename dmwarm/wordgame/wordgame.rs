use std::io::{self, Read};
use std::collections::{HashMap, VecDeque};

struct Graph {
    nodes: Vec<String>,
    maybe_neigh: HashMap<String, Vec<usize>>,
    edges: Vec<Vec<usize>>,
    idx1: usize,
    idx2: usize,
}
impl Graph {
    pub fn add_neighs(&mut self, s: String) {
        self.nodes.push(s.to_string());
    }
    pub fn create_edges(&mut self) {
        let first = self.nodes[0].clone();
        let second = self.nodes[1].clone();
        self.nodes.sort_unstable();
        self.idx1 = self.nodes.iter().position(|a| a == &first).unwrap();
        self.idx2 = self.nodes.iter().position(|a| a == &second).unwrap();
        for (idx, s) in self.nodes.iter().enumerate() {
            for i in 0..s.len() {
                let mut buf = String::new();
                buf.push_str(&s[..i]);
                buf.push(' ');
                buf.push_str(&s[i+1..]);
                self.maybe_neigh.entry(buf).or_insert(Vec::new()).push(idx);
            }
        }
        let mut buf = String::new();
        for (idx1, node) in self.nodes.iter().enumerate() {
            for i in 0..node.len() {
                buf.clear();
                buf.push_str(&node[..i]);
                buf.push(' ');
                buf.push_str(&node[i+1..]);
                for idx2 in self.maybe_neigh[&buf].iter().cloned() {
                    if idx1 == idx2 { continue; }
                    if compare(node, self.nodes[idx2].as_str()) {
                        self.edges[idx1].push(idx2);
                    }
                }
                self.edges[idx1].sort_unstable();
            }
        }
    }
    pub fn dijkstra(&self) -> Option<Vec<&str>> {
        let n = self.nodes.len();
        let mut previous = vec![None; n];
        let mut heap = VecDeque::new();
        heap.push_front(HeapVal {
            dist: 0,
            idx: self.idx1,
            prev: self.idx1,
        });
        while let Some(item) = heap.pop_back() {
            if previous[item.idx].is_some() { continue; }
            previous[item.idx] = Some(item.prev);
            for neigh in self.edges[item.idx].iter().cloned() {
                heap.push_front(HeapVal {
                    dist: item.dist + 1,
                    idx: neigh,
                    prev: item.idx,
                });
            }
        }
        let mut order = Vec::new();
        order.push(self.nodes[self.idx2].as_str());
        let mut i = self.idx2;
        while i != self.idx1 {
            i = match previous[i] {
                Some(i) => i,
                None => return None,
            };
            order.push(self.nodes[i].as_str());
        }
        Some(order)
    }
}

#[derive(Eq,PartialEq,Ord,PartialOrd)]
struct HeapVal {
    dist: u32,
    idx: usize,
    prev: usize,
}

fn compare(a: &str, b: &str) -> bool {
    let mut diffs = 0;
    assert!(a.len() == b.len());
    for (a,b) in a.as_bytes().iter().zip(b.as_bytes()) {
        if a != b {
            diffs += 1;
        }
    }
    diffs == 1
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();

    let mut split = buf.split_whitespace();
    let _ = split.next().unwrap();
    let n = split.next().unwrap().parse::<usize>().unwrap() + 2;

    let mut g = Graph {
        nodes: Vec::with_capacity(n),
        maybe_neigh: HashMap::with_capacity(6*n),
        edges: vec![Vec::new(); n],
        idx1: 0,
        idx2: 0,
    };

    for _ in 0..n {
        g.add_neighs(split.next().unwrap().to_string());
    }
    g.create_edges();
    match g.dijkstra() {
        Some(path) => for node in path.into_iter().rev() {
            println!("{}", node);
        },
        None => println!("IMPOSSIBLE"),
    }
}

