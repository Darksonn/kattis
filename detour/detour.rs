use std::io::{self, Read, BufReader};
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Reverse;

struct Problem {
    n: usize,
    neighbours: Vec<HashMap<usize, u64>>,
}


#[derive(Ord,PartialOrd,Eq,PartialEq)]
struct QueueState {
    dist: u64,
    node: usize,
    prev: usize,
}

impl Problem {
    fn dijkstra(&self, from: usize) -> Vec<Option<usize>> {
        let mut state = vec![None; self.n];
        let mut queue = BinaryHeap::new();
        queue.push(Reverse(QueueState {
            dist: 0,
            node: from,
            prev: from,
        }));
        while let Some(Reverse(n)) = queue.pop() {
            if state[n.node].is_some() { continue; }
            state[n.node] = Some(n.prev);
            for (neigh, d) in self.neighbours[n.node].iter().map(|(a,b)| (*a,*b)) {
                if state[neigh].is_some() { continue; }
                queue.push(Reverse(QueueState {
                    dist: n.dist + d,
                    node: neigh,
                    prev: n.node,
                }));
            }
        }
        state
    }
    fn eliminate(&mut self) {
        for (n, state) in self.dijkstra(1).into_iter().enumerate() {
            if let Some(state) = state {
                self.neighbours[n].remove(&state);
            }
        }
    }
    fn solve(&self) -> Option<Vec<usize>> {
        let res = self.dijkstra(0);
        let mut path = vec![1];
        let mut i = 1;
        while i != 0 {
            let prev = res[i]?;
            path.push(prev);
            i = prev;
        }
        Some(path)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin = BufReader::new(stdin.lock());
    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_ascii_whitespace();

    let n = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let mut p = Problem {
        n,
        neighbours: vec![HashMap::new(); n],
    };

    for _ in 0..m {
        let a: usize = iter.next().unwrap().parse().unwrap();
        let b: usize = iter.next().unwrap().parse().unwrap();
        let d = iter.next().unwrap().parse().unwrap();
        p.neighbours[a].insert(b, d);
        p.neighbours[b].insert(a, d);
    }

    p.eliminate();
    match p.solve() {
        Some(path) => {
            print!("{}", path.len());
            for i in path.into_iter().rev() {
                print!(" {}", i);
            }
        }
        None => print!("impossible"),
    }
    println!();
}
