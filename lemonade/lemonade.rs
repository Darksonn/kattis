use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;

#[derive(Copy,Clone)]
struct Kind(usize);

struct Graph {
    kinds: HashMap<String, Kind>,
    next_kind: Kind,
    exchange: Vec<f64>,
}

impl Graph {
    pub fn new() -> Self {
        let mut kinds = HashMap::new();
        kinds.insert(String::from("pink"), Kind(0));
        Self {
            kinds,
            next_kind: Kind(1),
            exchange: vec![0.0],
        }
    }

    pub fn add_edge(&mut self, from: &str, to: &str, exch: f64) {
        let from_idx = match self.kinds.get(from) {
            Some(from_idx) => from_idx,
            None => return, // We don't have any of this kind, so this is not usable
        };

        let past_exch = self.exchange[from_idx.0];
        let exchange = exch + past_exch;

        match self.kinds.get(to) {
            Some(to_idx) => {
                let prev_exchange = self.exchange[to_idx.0];
                if exchange > prev_exchange {
                    self.exchange[to_idx.0] = exchange;
                }
            }
            None => {
                let kind = self.next_kind;
                self.next_kind = Kind(kind.0 + 1);

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

    let mut graph = Graph::new();

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
    let blue = graph.exchange[blue_kind.0].exp2();
    if blue > 10.0 {
        println!("10.0");
    } else {
        println!("{:.10}", blue);
    }
}
