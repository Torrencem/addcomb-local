use std::cmp;
use rayon::prelude::*;
use fastset::*;

mod hfolds;

use self::hfolds::*;

pub fn nu(n: u8, m: u32, h: u8) -> u8 {
    let mut curr_greatest = 0;
    for a in each_set_exact(n as u32, m) {
        let size = a.hfoldsumset(h, n).size();
        if size > curr_greatest {
            curr_greatest = size;
        }
    }
    curr_greatest
}

pub fn nu_exceptions(n: u32) -> u32 {
    // E.1.1 for critical numbers for bounds
    let params: Vec<(u32, u32)> = iproduct!(2u32..=n/2, 3u32..=n/2).collect();

    let excepts_number = params.into_par_iter().filter(|(h, m)| {
            let expected = cmp::min(n, choose(m + h - 1, *h)) as u8;
            nu(n as u8, *m, *h as u8) != expected
        })
        //.inspect(|(h, m)| println!("Exception: h={}, m={}", h, m))
        .count() as u32;

    excepts_number
}