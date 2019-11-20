use std::io::{self, BufRead, BufReader};

fn parse_time(t: &str) -> usize {
    let h: usize = t[0..2].parse().unwrap();
    let m: usize = t[2..4].parse().unwrap();
    h * 60 + m - 9 * 60
}

fn main() {
    let stdin = io::stdin();
    let mut stdin = BufReader::new(stdin.lock());

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let mut split = buf.split_whitespace();

    let _ = split.next().unwrap();
    let cal_entries = split.next().unwrap().parse().unwrap();
    let meeting_duration = split.next().unwrap().parse().unwrap();

    let mut ps = vec![0i32; 1 + (17 - 9 + 1) * 60];

    for _ in 0..cal_entries {
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        let mut split = buf.split_whitespace();

        let _ = split.next().unwrap();
        let start = parse_time(split.next().unwrap());
        let end = parse_time(split.next().unwrap());

        ps[start] += 1;
        ps[end] -= 1;

        assert!(start < end);
    }

    let mut sum = 0;
    for (i, ptr) in ps.iter_mut().enumerate() {
        sum += *ptr;
        *ptr = sum;
    }

    let mut size = 0;
    for i in 0..(17 - 9) * 60 {
        if ps[i] > 0 {
            size = 0;
        } else {
            size += 1;
            if size >= meeting_duration {
                let i = i + 1 - meeting_duration as usize;
                let hours = (i / 60) + 9;
                let minutes = i % 60;
                println!("{:04}", hours * 100 + minutes);
                return;
            }
        }
    }
    println!("IMPOSSIBLE");

}
