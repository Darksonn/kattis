use std::io::{self, Read};
use std::str::SplitAsciiWhitespace;
use std::cmp::Ordering;

#[derive(Copy,Clone,Debug)]
struct Point {
    x: f64,
    y: f64,
}
impl Point {
    fn read(iter: &mut SplitAsciiWhitespace) -> Point {
        let x = iter.next().unwrap().parse().unwrap();
        let y = iter.next().unwrap().parse().unwrap();
        Point { x, y }
    }
    fn dist2(self) -> f64 {
        let x2 = self.x * self.x;
        let y2 = self.y * self.y;
        x2 + y2
    }
    fn dist(self) -> f64 {
        self.x.hypot(self.y)
    }
    fn unit(self) -> Point {
        let dist = self.dist2().sqrt();
        Point {
            x: self.x / dist,
            y: self.y / dist,
        }
    }
    fn rotate90(self) -> Point {
        Point {
            x: -self.y,
            y: self.x,
        }
    }
    fn in_upper_plane(self) -> bool {
        if self.y == 0 {
            self.x > 0
        } else {
            self.y > 0
        }
    }
    fn cmp_unit(self, other: Point) -> Ordering {
        if !self.in_upper_plane() {
            if !other.in_upper_plane() {
                return self.x.partial_cmp(&other.x).unwrap();
            }
            return Ordering::Greater;
        }
        if !other.in_upper_plane() {
            return Ordering::Less;
        }
        other.x.partial_cmp(&self.x).unwrap()
    }
}
impl std::ops::Sub for Point {
    type Output = Point;
    fn sub(self, other: Self) -> Self {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, other: Self) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl std::ops::Neg for Point {
    type Output = Point;
    fn neg(self) -> Self {
        Point {
            x: -self.x,
            y: -self.y,
        }
    }
}
impl std::ops::Mul<f64> for Point {
    type Output = Point;
    fn mul(self, other: f64) -> Self {
        Point {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

#[derive(Copy,Clone,Debug)]
struct Circle {
    x: i64,
    y: i64,
    r: i64,
}
#[derive(Copy,Clone,Debug)]
enum CircleInt {
    None,
    Single(Point),
    Two(Point, Point),
}
impl Circle {
    fn intersection(self, other: Circle) -> CircleInt {
        let r2 = (self.r + other.r) * (self.r + other.r);
        let x2 = (self.x - other.x) * (self.x - other.x);
        let y2 = (self.y - other.y) * (self.y - other.y);
        let d2 = x2 + y2;
        match d2.cmp(&r2) {
            Ordering::Less => {
                let c1 = self.center();
                let c2 = other.center();
                let diff = c2 - c1;

                let d = diff.dist();
                let d2 = diff.dist2();
                let r2 = (other.r * other.r) as f64;
                #[allow(non_snake_case)]
                let R2 = (self.r * self.r) as f64;

                let x = (d2 - r2 + R2) / (2.0 * d);

                let sub = d2 - r2 + R2;
                let a = 0.5 * (4.0*d2*R2 - sub*sub).sqrt() / d;

                let v1 = diff.unit();
                let pi = c1 + v1 * x;
                let plus = v1.rotate90() * a;
                let p1 = pi + plus;
                let p2 = pi - plus;
                CircleInt::Two(p1, p2)
            },
            Ordering::Equal => {
                let c1 = self.center();
                let c2 = other.center();
                let diff = c2 - c1;
                CircleInt::Single(c1 + diff.unit() * self.r as f64)
            },
            Ordering::Greater => CircleInt::None,
        }
    }
    fn center(self) -> Point {
        Point { x: self.x as f64, y: self.y as f64 }
    }
}

enum Location {
    Inner(Point),
    Intersection(Point, Circle, Circle),
}

struct ShelfBuilder {
    circle: Circle,
    initial: i32,
    sections: Vec<(Point, i32)>,
}
impl ShelfBuilder {
    pub fn new(circle: Circle) -> Self {
        Self {
            circle,
            initial: 0,
            sections: Vec::new(),
        }
    }
    pub fn add_circle(&mut self, other: Circle, int: CircleInt) {
        match int {
            CircleInt::None => {},
            CircleInt::Single(_) => {},
            CircleInt::Two(p1, p2) => {
                self.sections.push((p1, -1));
                self.sections.push((p2, +1));
                if p1.unit().cmp_unit(p2.unit()) == Ordering::Less {
                    self.initial += 1;
                }
            },
        }
    }
    pub fn build(self) -> Shelf {
        let mut sections = Vec::new();
        let mut sum = self.initial;
        for (p, d) in self.sections {
            let psum = sum;
            sum += d;
            if sum == 0 ^ psum > 0 {
                sections.push(p);
            }
        }
        Shelf {
            circle: self.circle,
            zero_included: self.initial > 0,
            sections,
        }
    }
}

struct Shelf {
    circle: Circle,
    zero_included: bool,
    sections: Vec<Point>,
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf);
    let mut split = buf.split_ascii_whitespace();

    let barney = Point::read(&mut split);
    let mom = Point::read(&mut split);

    let n = split.next().unwrap().parse().unwrap();
    let mut points = Vec::with_capacity(n);
    for _ in 0..n {
        let x = split.next().unwrap().parse().unwrap();
        let y = split.next().unwrap().parse().unwrap();
        let r = split.next().unwrap().parse().unwrap();
        points.push(Circle { x, y, r });
    }
}
