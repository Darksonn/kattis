use std::io::{self, Read};
use std::collections::HashSet;

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let b_ = b;
        b = a % b;
        a = b_;
    }
    a
}

struct State {
    seen: HashSet<u64>,
    intervals: Vec<u64>,
}
impl State {
    pub fn new() -> Self {
        Self {
            seen: HashSet::with_capacity(61 * 5 * 100000),
            intervals: Vec::with_capacity(61),
        }
    }
    pub fn insert(&mut self, item: u64) {
        let mut prev = 0;
        for other in self.intervals.iter_mut() {
            let new = gcd(item, *other);
            *other = new;
            if new != prev {
                self.seen.insert(new);
            }
            prev = new;
        }
        self.intervals.push(item);
        self.intervals.dedup();
        self.seen.insert(item);
    }
}

fn main() {
    let mut stdin = io::stdin();

    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();
    let mut split = buf.split_ascii_whitespace();

    let n = split.next().unwrap().parse().unwrap();
    let mut state = State::new();

    for _ in 0..n {
        let k = split.next().unwrap().parse().unwrap();
        state.insert(k);
    }

    println!("{}", state.seen.len());

}
