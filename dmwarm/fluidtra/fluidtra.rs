use std::io::{self, BufRead, BufReader};
use std::cmp::{min, max};

struct Pipe {
    y: u64,
    h: u64,
    l: u64,
    r: u64,
}

fn main() {
    let stdin = io::stdin();
    let mut stdin = BufReader::new(stdin.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let n = buf.trim().parse().unwrap();

    let mut pipes = Vec::with_capacity(n);
    for _ in 0..n {
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        let mut split = buf.split_whitespace();

        let y = split.next().unwrap().parse().unwrap();
        let h = split.next().unwrap().parse().unwrap();
        let l = split.next().unwrap().parse().unwrap();
        let r = split.next().unwrap().parse().unwrap();
        let pipe = Pipe { y, h, l, r };
        pipes.push(pipe);

    }

    let mut min_per_pipe = vec![0; n];
    for i in (0..n).rev() {
        let next_min_water = *min_per_pipe.get(i+1).unwrap_or(&0);
        let min_water = max(pipes[i].y + pipes[i].r, next_min_water);
        min_per_pipe[i] = min_water;
    }

    for i in 0..n {
        println!("{}", min(min_per_pipe[i] - pipes[i].y, pipes[i].h));
    }

}
