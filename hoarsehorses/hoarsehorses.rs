use std::io::{self, Read};

fn sign(i: i64) -> i64 {
    if i <= 0 { -1 } else { 1 }
}

#[derive(Copy,Clone,Debug)]
struct PointI(i64, i64);

impl std::ops::Add for PointI {
    type Output = PointI;
    fn add(self, other: PointI) -> PointI {
        PointI(self.0 + other.0, self.1 + other.1)
    }
}
impl std::ops::Neg for PointI {
    type Output = PointI;
    fn neg(self) -> PointI {
        PointI(-self.0, -self.1)
    }
}
impl std::ops::Add<PointR> for PointI {
    type Output = PointR;
    fn add(self, other: PointR) -> PointR {
        PointR(self.0 as f64 + other.0, self.1 as f64 + other.1)
    }
}
impl std::ops::Sub for PointI {
    type Output = PointI;
    fn sub(self, other: PointI) -> PointI {
        PointI(self.0 - other.0, self.1 - other.1)
    }
}
// cross product
impl std::ops::Mul for PointI {
    type Output = i64;
    fn mul(self, other: PointI) -> i64 {
        self.0 * other.1 - self.1 * other.0
    }
}
impl std::ops::Mul<f64> for PointI {
    type Output = PointR;
    fn mul(self, other: f64) -> PointR {
        PointR(self.0 as f64 * other, self.1 as f64 * other)
    }
}

#[derive(Copy,Clone,Debug)]
struct PointR(f64, f64);
impl std::ops::Sub for PointR {
    type Output = PointR;
    fn sub(self, other: PointR) -> PointR {
        PointR(self.0 - other.0, self.1 - other.1)
    }
}

#[derive(Copy,Clone,Debug)]
struct Line {
    from: PointI,
    to: PointI,
}
impl Line {
    pub fn intersection(self, other: Line) -> Option<PointR> {
        let p1 = self.from;
        let p2 = other.from;
        let v1 = self.to - self.from;
        let v2 = other.to - other.from;

        let den = v1 * v2;
        let t1_num = (p2 - p1) * v2 * sign(den);
        let t2_num = -(p1 - p2) * v1 * sign(den);
        let den = den.abs();

        if den == 0 {
            None
        } else {
            if t1_num < 0 || t1_num > den {
                return None;
            }
            if t2_num < 0 || t2_num > den {
                return None;
            }

            let den = den as f64;
            let t1 = t1_num as f64 / den;

            Some(p1 + v1 * t1)
        }
    }
}

struct UnionFind {
    root: Vec<usize>,
    components: usize,
}
impl UnionFind {
    pub fn new(size: usize) -> Self {
        Self {
            root: (0..size).collect(),
            components: size,
        }
    }
    pub fn root(&mut self, mut a: usize) -> usize {
        let mut i = a;
        while i != self.root[i] {
            i = self.root[i];
        }
        while a != i {
            let na = self.root[a];
            self.root[a] = i;
            a = na;
        }
        i
    }
    pub fn union(&mut self, a: usize, b: usize) {
        let a = self.root(a);
        let b = self.root(b);
        if a != b {
            self.components -= 1;
            self.root[a] = b;
        }
    }
}

fn solve(lines: &[Line]) -> usize {
    let mut nodes_by_line = vec![0usize; lines.len()];
    let mut uf = UnionFind::new(lines.len());

    for (i, line1) in lines.iter().copied().enumerate() {
        for (j, line2) in lines.iter().copied().enumerate().skip(i+1) {
            if let Some(_) = line1.intersection(line2) {
                nodes_by_line[i] += 1;
                nodes_by_line[j] += 1;
                uf.union(i, j);
            }
        }
    }
    let mut components = uf.components;
    let mut nodes = 0;
    let mut edges = 0;
    for i in 0..lines.len() {
        let nbl = nodes_by_line[i];
        if nbl == 0 {
            components -= 1;
            continue;
        }
        nodes += nbl;
        edges += nbl - 1;
    }
    let nodes = nodes / 2;
    components + edges - nodes
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let mut split = buf.split_ascii_whitespace();

    let n = split.next().unwrap().parse().unwrap();
    let mut lines = Vec::with_capacity(n);
    for _ in 0..n {
        let x1 = split.next().unwrap().parse().unwrap();
        let y1 = split.next().unwrap().parse().unwrap();
        let x2 = split.next().unwrap().parse().unwrap();
        let y2 = split.next().unwrap().parse().unwrap();

        let from = PointI(x1, y1);
        let to = PointI(x2, y2);

        let line = Line { from, to };
        lines.push(line);
    }

    let res = solve(&lines);
    println!("{}", res);
}

