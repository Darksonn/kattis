use std::ops::*;
use std::cmp::{min, max};
use std::io::Read;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
struct Song {
    high: u128,
    low: u128,
}

impl From<u32> for Song {
    #[inline]
    fn from(i: u32) -> Song {
        Song { low: i as u128, high: 0 }
    }
}

impl Sub for Song {
    type Output = Song;
    #[inline]
    fn sub(self, other: Song) -> Song {
        let (low, carry) = self.low.overflowing_sub(other.low);
        let high = self.high - other.high - carry as u128;
        Song { low, high }
    }
}
impl Add for Song {
    type Output = Song;
    #[inline]
    fn add(self, other: Song) -> Song {
        let (low, carry) = self.low.overflowing_add(other.low);
        let high = self.high + other.high + carry as u128;
        Song { low, high }
    }
}

impl Song {
    #[inline]
    pub fn mul_10(self) -> Song {
        let double = self + self;
        let quad = double + double;
        let eight = quad + quad;
        eight + double
    }
    pub fn from_string(s: &str) -> Song {
        let mut result = Song::from(0);
        for c in s.as_bytes() {
            let num = c - b'0';
            result = result.mul_10() + Song::from(num as u32);
        }
        result
    }
}

#[derive(Copy, Clone, Debug)]
struct Minutes(u32);

const MOD: u32 = 1000000007;

impl From<u32> for Minutes {
    fn from(i: u32) -> Minutes {
        Minutes(i % MOD)
    }
}

impl Add for Minutes {
    type Output = Minutes;
    fn add(self, other: Minutes) -> Minutes {
        let i = self.0.wrapping_add(other.0);
        Minutes(i % MOD)
    }
}
impl Sub for Minutes {
    type Output = Minutes;
    fn sub(self, other: Minutes) -> Minutes {
        let i = self.0.wrapping_sub(other.0).wrapping_add(MOD);
        Minutes(i % MOD)
    }
}

#[derive(Debug)]
enum Kind {
    Single(Minutes),
    Concat {
        left: usize,
        right: usize,
        left_len: Song,
    },
    Replace {
        inner: usize,
        song: Song,
        minutes: Minutes,
        difference: Minutes,
    },
}

#[derive(Debug)]
struct Eval {
    playlists: Vec<(Kind, Minutes, Song)>,
}
impl Eval {
    pub fn push(&mut self, kind: Kind, sum: Minutes, width: Song) {
        self.playlists.push((kind, sum, width));
    }
    pub fn sum(&self, pl: usize, from: Song, to: Song) -> Minutes {
        match self.playlists[pl] {
            (Kind::Single(m), _, _) => m,
            (Kind::Concat { left, right, left_len }, full_sum, width) => {
                if from == Song::from(0) && to + Song::from(1) == width {
                    full_sum
                } else {
                    let mut left_sum = Minutes::from(0);
                    let mut right_sum = Minutes::from(0);
                    if from < left_len {
                        left_sum = self.sum(left, from, min(to, left_len-Song::from(1)));
                    }
                    if left_len <= to {
                        right_sum = self.sum(right,
                                             max(from, left_len) - left_len,
                                             to - left_len);
                    }
                    left_sum + right_sum
                }
            },
            (Kind::Replace { inner, song, difference, .. }, _, _) => {
                let s = self.sum(inner, from, to);
                if from <= song && song <= to {
                    s + difference
                } else {
                    s
                }
            },
        }
    }
    pub fn value(&self, pl: usize, idx: Song) -> Minutes {
        match self.playlists[pl] {
            (Kind::Single(m), _, _) => m,
            (Kind::Concat { left, right, left_len }, _, _) => {
                if idx < left_len {
                    self.value(left, idx)
                } else {
                    self.value(right, idx - left_len)
                }
            },
            (Kind::Replace { inner, song, minutes, .. }, _, _) => {
                if idx == song {
                    minutes
                } else {
                    self.value(inner, idx)
                }
            },
        }
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().lock().read_to_string(&mut buf).unwrap();
    let mut iter = buf.split_ascii_whitespace();

    let len: usize = iter.next().unwrap().parse().unwrap();
    let queries: usize = iter.next().unwrap().parse().unwrap();
    let first_song: u32 = iter.next().unwrap().parse().unwrap();
    let first_song = Minutes::from(first_song);

    let mut eval = Eval {
        playlists: Vec::with_capacity(len + 1),
    };
    eval.push(Kind::Single(first_song), first_song, Song::from(1));

    for _ in 0..len {
        if iter.next() == Some("copy") {
            let left = iter.next().unwrap().parse().unwrap();
            let right = iter.next().unwrap().parse().unwrap();

            let (_, left_sum, left_len) = eval.playlists[left];
            let (_, right_sum, right_len) = eval.playlists[right];
            eval.push(
                Kind::Concat { left, right, left_len },
                left_sum + right_sum,
                left_len + right_len,
            );
        } else {
            let idx = iter.next().unwrap().parse().unwrap();
            let song = Song::from_string(iter.next().unwrap());
            let minutes = Minutes::from(iter.next().unwrap().parse::<u32>().unwrap());
            let difference = minutes - eval.value(idx, song);
            let (_, prev_sum, width) = eval.playlists[idx];
            eval.push(
                Kind::Replace {
                    inner: idx,
                    song,
                    minutes,
                    difference,
                },
                prev_sum + difference,
                width,
            );
        }
    }

    for _ in 0..queries {
        let idx = iter.next().unwrap().parse().unwrap();
        let from = Song::from_string(iter.next().unwrap());
        let to = Song::from_string(iter.next().unwrap());
        println!("{}", eval.sum(idx, from, to).0);
    }

}
