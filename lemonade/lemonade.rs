use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;

#[derive(Copy,Clone)]
struct Kind(usize);

struct Graph {
    kinds: HashMap<String, Kind>,
    exchange: Vec<f64>,
}

impl Graph {
    pub fn new(size: usize) -> Self {
        let mut kinds = HashMap::with_capacity(size);
        kinds.insert(String::from("pink"), Kind(0));

        let mut exchange = Vec::with_capacity(size);
        exchange.push(0.0);

        Self {
            kinds,
            exchange,
        }
    }

    pub fn get_exch(&self, kind: Kind) -> f64 {
        unsafe {
            *self.exchange.get_unchecked(kind.0)
        }
    }
    pub fn set_exch(&mut self, kind: Kind, val: f64) {
        unsafe {
            *self.exchange.get_unchecked_mut(kind.0) = val;
        }
    }

    pub fn add_edge(&mut self, from: &str, to: &str, exch: f64) {
        let from_idx = match self.kinds.get(from) {
            Some(from_idx) => *from_idx,
            None => return, // We don't have any of this kind, so this is not usable
        };

        let past_exch = self.get_exch(from_idx);
        let exchange = exch + past_exch;

        match self.kinds.get(to).map(|to_idx| *to_idx) {
            Some(to_idx) => {
                let prev_exchange = self.get_exch(to_idx);
                if exchange > prev_exchange {
                    self.set_exch(to_idx, exchange);
                }
            }
            None => {
                let kind = Kind(self.exchange.len());

                self.kinds.insert(to.to_string(), kind);
                self.exchange.push(exchange);
            }
        }

    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin = BufReader::new(stdin.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let n = buf.trim().parse().unwrap();

    let mut graph = Graph::new(n);

    for _ in 0..n {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        let mut split = buf.split_ascii_whitespace();

        let offered = split.next().unwrap();
        let wanted = split.next().unwrap();
        let exchange: f64 = split.next().unwrap().parse().unwrap();

        graph.add_edge(wanted, offered, exchange.log2());
    }

    let blue_kind = match graph.kinds.get("blue") {
        Some(bk) => bk,
        None => {
            println!("0.0");
            return;
        }
    };
    let blue = graph.get_exch(*blue_kind).exp2();
    if blue > 10.0 {
        println!("10.0");
    } else {
        println!("{:.10}", blue);
    }
}
