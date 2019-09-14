use std::io::{BufRead, BufReader, self};

struct Problem {
    n: usize,
    adj: Vec<Vec<usize>>,
}
impl Problem {
    fn solve(&self) -> Option<Vec<usize>> {
        let mut seq = Vec::with_capacity(self.n);
        let mut visited = vec![false; self.n];
        seq.push(0);
        visited[0] = true;
        for i in 0..self.n {
            let now = *seq.get(i)?;
            for j in self.adj[now].iter().copied() {
                if visited[j] { continue; }
                visited[j] = true;
                seq.push(j);
            }
        }
        Some(seq)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin = BufReader::new(stdin.lock());

    let mut buf = String::new();

    stdin.read_line(&mut buf).unwrap();
    let size = buf.trim().parse().unwrap();

    let mut p = Problem {
        n: size,
        adj: vec![Vec::new(); size],
    };

    for i in 0..size {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();

        for j in 0..size {
            if buf.as_bytes()[j] == b'1' {
                p.adj[i].push(j);
            }
        }
    }

    if let Some(list) = p.solve() {
        let mut prefix = "";
        for i in list.into_iter().rev() {
            print!("{}{}", prefix, i);
            prefix = " ";
        }
        println!();
    } else {
        println!("impossible");
    }
}
