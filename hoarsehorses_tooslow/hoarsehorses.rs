use std::io::{self, Read};
use std::cmp::{min, max, Ordering};

fn sign(i: i64) -> i64 {
    if i <= 0 { -1 } else { 1 }
}

struct Interval(i64, i64);
impl Interval {
    fn new(a: i64, b: i64) -> Self {
        Self(min(a,b), max(a,b))
    }
    fn intersects(self, other: Self) -> bool {
        self.0 <= other.1 && other.0 <= self.1
    }
}

#[derive(Copy,Clone)]
struct Angle {
    point: PointI,
}
impl Angle {
    pub fn from_line(line: Line) -> Angle {
        let p = line.from - line.to;
        let s = sign(p.0) * sign(p.1);
        Angle {
            point: PointI(p.0.abs() * s, p.1.abs())
        }
    }
}
impl PartialEq for Angle {
    fn eq(&self, other: &Angle) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl Eq for Angle { }
impl PartialOrd for Angle {
    fn partial_cmp(&self, other: &Angle) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Angle {
    fn cmp(&self, other: &Angle) -> Ordering {
        if self.point.1 < 0 {
            if other.point.1 < 0 {
                return Angle { point: -self.point }.cmp(&Angle { point: -other.point });
            } else {
                return Ordering::Greater;
            }
        }
        if other.point.1 < 0 {
            return Ordering::Less;
        }
        (self.point * other.point).cmp(&0).reverse()
    }
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
    pub fn adjoin(self, other: Line) -> Option<Line> {
        let v1 = self.to - self.from;
        let v2 = other.to - other.from;
        if v1 * v2 == 0 {
            let ix1 = Interval::new(self.from.0, self.to.0);
            let iy1 = Interval::new(self.from.1, self.to.1);
            let ix2 = Interval::new(other.from.0, other.to.0);
            let iy2 = Interval::new(other.from.1, other.to.1);
            if !ix1.intersects(ix2) { return None; }
            if !iy1.intersects(iy2) { return None; }

            let p0 = PointI(0, 0);
            let p1 = v1;
            let p2 = other.to - self.from;
            let p3 = other.from - self.from;
            let mut arr = [p0, p1, p2, p3];

            arr.sort_unstable_by(|p,q| {
                p.0.cmp(&q.0).then(p.1.cmp(&q.1))
            });
            Some(Line {
                from: arr[0] + self.from,
                to: arr[3] + self.from,
            })
        } else {
            None
        }
    }
}

fn adjoin_pairs(lines: &mut Vec<Line>) {
    lines.sort_unstable_by_key(|line| Angle::from_line(*line));
    let mut i = 0;
    while i < lines.len() {
        let angle = Angle::from_line(lines[i]);
        let mut j = i+1;
        while j < lines.len() && angle == Angle::from_line(lines[j]) {
            if let Some(res) = lines[i].adjoin(lines[j]) {
                lines[i] = res;
                lines.remove(j);
            } else {
                j += 1;
            }
        }
        i += 1;
    }
}

struct Graph {
    nodes: Vec<PointR>,
    edges: Vec<Vec<usize>>,
    nodes_by_line: Vec<Vec<usize>>,
}
impl Graph {
    pub fn construct(lines: &[Line]) -> Graph {
        let mut g = Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
            nodes_by_line: vec![Vec::new(); lines.len()],
        };
        for (i, line1) in lines.iter().copied().enumerate() {
            for (j, line2) in lines.iter().copied().enumerate().skip(i+1) {
                if let Some(p) = line1.intersection(line2) {
                    let len = g.nodes.len();
                    g.nodes_by_line[i].push(len);
                    g.nodes_by_line[j].push(len);
                    g.nodes.push(p);
                }
            }
        }
        println!("a");
        g.edges.resize(g.nodes.len(), Vec::new());
        for ptr in g.nodes_by_line.iter_mut() {
            if ptr.len() == 0 { continue; }
            let nodes = &g.nodes;
            let first = nodes[ptr[0]];
            ptr.sort_unstable_by(|i, j| {
                let p1 = nodes[*i] - first;
                let p2 = nodes[*j] - first;
                let diffx = p1.0 - p2.0;
                let diffy = p1.1 - p2.1;
                if diffx.abs() > diffy.abs() {
                    diffx.partial_cmp(&0.0).unwrap_or(Ordering::Equal)
                } else {
                    diffy.partial_cmp(&0.0).unwrap_or(Ordering::Equal)
                }
            });
            let iter1 = ptr.iter();
            let iter2 = ptr.iter().skip(1);
            for (p, q) in iter1.zip(iter2).map(|(p,q)| (*p, *q)) {
                g.edges[p].push(q);
                g.edges[q].push(p);
            }
        }
        println!("b");
        for (p, p_edges) in g.edges.iter_mut().enumerate() {
            let nodes = &g.nodes;
            let p = nodes[p];
            p_edges.sort_unstable_by(|i, j| {
                let p = nodes[*i] - p;
                let q = nodes[*j] - p;
                let a1 = p.1.atan2(p.0);
                let a2 = q.1.atan2(q.0);
                a1.partial_cmp(&a2).unwrap()
            });
        }
        g
    }
}

fn main() {
    let start = std::time::Instant::now();
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

    adjoin_pairs(&mut lines);
    let g = Graph::construct(&lines);
    let end = start.elapsed();
    println!("{} {:?}", g.edges.len(), end);

}

