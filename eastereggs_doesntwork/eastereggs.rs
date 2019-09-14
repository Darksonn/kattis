use std::io::{self, Read};

#[derive(Copy, Clone, Debug)]
struct Point(i32, i32);

// dist^2, blue_i, red_i
#[derive(Copy, Clone)]
struct Edge(u64, usize, usize);

struct Problem {
    blue: Vec<Point>,
    red: Vec<Point>,
    edges: Vec<Edge>,
    needed: usize,
}

impl Problem {
    fn compute_edges(&mut self) {
        for (i, p1) in self.blue.iter().copied().enumerate() {
            for (j, p2) in self.red.iter().copied().enumerate() {
                let distx = (p1.0 - p2.0).abs() as u64;
                let disty = (p1.1 - p2.1).abs() as u64;
                let dist_squared = distx * distx + disty * disty;
                self.edges.push(Edge(dist_squared, i + self.red.len(), j));
            }
        }
        self.edges.sort_unstable_by_key(|edge| edge.0);
    }
    fn solve(self) -> u64 {
        let mut union_find = UnionFind::new(self.red.len(), self.blue.len());
        for edge in self.edges {
            println!(
                "Edge {:?} to {:?} of dist {}.",
                self.blue[edge.1 - self.red.len()],
                self.red[edge.2],
                (edge.0 as f64).sqrt()
            );
            union_find.union(edge.1, edge.2);
            if union_find.maximum < self.needed {
                return edge.0;
            }
        }
        unreachable!()
    }
}

#[derive(Copy, Clone)]
struct Data {
    num_red: usize,
    num_blue: usize,
}
impl Data {
    fn max(self) -> usize {
        std::cmp::max(self.num_red, self.num_blue)
    }
}
struct UnionFind {
    root: Vec<usize>,
    data: Vec<Data>,
    maximum: usize,
}
impl UnionFind {
    pub fn new(red: usize, blue: usize) -> Self {
        let size = red + blue;
        let mut vec = Vec::with_capacity(size);
        for _ in 0..red {
            vec.push(Data {
                num_red: 1,
                num_blue: 0,
            });
        }
        for _ in 0..blue {
            vec.push(Data {
                num_red: 0,
                num_blue: 1,
            });
        }
        Self {
            root: (0..size).collect(),
            data: vec,
            maximum: size,
        }
    }
    fn root(&mut self, a: usize) -> usize {
        let mut root = self.root[a];
        while root != self.root[root] {
            root = self.root[root];
        }
        let mut i = a;
        while i != root {
            let next = self.root[i];
            self.root[i] = root;
            i = next;
        }
        root
    }
    pub fn union(&mut self, a: usize, b: usize) {
        let a = self.root(a);
        let b = self.root(b);

        let da = self.data[a];
        let db = self.data[b];
        let data = Data {
            num_red: da.num_red + db.num_red,
            num_blue: da.num_blue + db.num_blue,
        };

        self.maximum += data.max();
        self.maximum -= da.max();
        self.maximum -= db.max();

        self.data[b] = data;
        self.root[a] = b;
    }
}

fn main() {
    let mut stdin = io::stdin();
    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();
    let mut split = buf.split_ascii_whitespace();

    let n = split.next().unwrap().parse().unwrap();
    let b = split.next().unwrap().parse().unwrap();
    let r = split.next().unwrap().parse().unwrap();

    let mut p = Problem {
        blue: Vec::with_capacity(b),
        red: Vec::with_capacity(r),
        edges: Vec::with_capacity(b * r),
        needed: n,
    };

    for _ in 0..b {
        let x = split.next().unwrap().parse().unwrap();
        let y = split.next().unwrap().parse().unwrap();
        p.blue.push(Point(x,y));
    }
    for _ in 0..r {
        let x = split.next().unwrap().parse().unwrap();
        let y = split.next().unwrap().parse().unwrap();
        p.red.push(Point(x,y));
    }

    p.compute_edges();
    let res = p.solve();
    println!("{}", (res as f64).sqrt());
}

