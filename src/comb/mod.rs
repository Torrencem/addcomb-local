
pub mod chapter_a;
pub mod chapter_b;
pub mod hfolds;

use std::cmp;

pub fn choose(n: u32, k: u32) -> u32 {
    if k == 0 {
        1
    } else {
        (n * choose(n - 1, k - 1)) / k
    }
}

pub fn c(h: u32, m: u32) -> u32 {
    (1..=cmp::min(m, h-1)+1).map(|i| {
        choose(m, i) * choose(h - 1, i - 1) * 2u32.pow(i)
    }).sum()
}