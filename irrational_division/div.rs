use std::io::{self, BufRead, BufReader};

struct Dynamic {
    p: i32,
    q: i32,
    buf: Vec<Option<i32>>,
}
impl Dynamic {
    pub fn solve(&mut self, a: i32, b: i32) -> i32 {
        if a < 0 || b < 0 {
            return 0;
        }

        let idx = (a + 101 * b) as usize;
        if let Some(res) = self.buf[idx] {
            return res;
        }

        let mut best = None;
        for eaten in 1..=a+1 {
            let score_rest = self.solve(b, a - eaten);
            let score_me = self.value((a, 0), (a - eaten + 1, b));
            let score = score_me - score_rest;
            if let Some(other) = best {
                if other < score {
                    best = Some(score);
                }
            } else {
                best = Some(score);
            }
        }
        self.buf[idx] = best;
        best.unwrap()
    }
    // upper left, lower right
    #[inline]
    pub fn value(&self, from: (i32, i32), to: (i32, i32)) -> i32 {
        let area = (from.0 - to.0 + 1).abs() * (from.1 - to.1 + 1).abs();
        if area % 2 == 0 {
            return 0;
        }

        let dist_from_black = (from.0 - (self.q - 1)).abs() + from.1.abs();
        let upper_left_is_black = dist_from_black % 2 == 0;
        if upper_left_is_black { 1 } else { -1 }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin = BufReader::new(stdin.lock()).lines();
    let line: Vec<_> = stdin.next().unwrap().unwrap().split_ascii_whitespace().map(|s| s.parse().unwrap()).collect();
    let mut dp = Dynamic {
        p: line[0],
        q: line[1],
        buf: Vec::new(),
    };
    drop(line);
    drop(stdin);
    dp.buf.resize(101 * 101, None);
    dp.buf[0] = Some(dp.value((0,0), (0,0)));

    println!("{}", dp.solve(dp.q-1, dp.p-1));
}
