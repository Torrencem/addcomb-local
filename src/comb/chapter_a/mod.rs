use std::cmp;
use rayon::prelude::*;
use fastset::*;
use comb::*;

pub fn nu(n: u32, m: u32, h: u32) -> u32 {
    let mut curr_greatest = 0;
    for a in each_set_exact(n, m) {
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
            let expected = cmp::min(n, choose(m + h - 1, *h));
            nu(n, *m, *h) != expected
        })
        //.inspect(|(h, m)| println!("Exception: h={}, m={}", h, m))
        .count();

    excepts_number as u32
}

pub fn nu_interval(n: u32, m: u32, s: u32) -> u32 {
    let mut curr_greatest = 0;
    for a in each_set_exact(n, m) {
        let size = a.hfoldintervalsumset((0, s), n).size();
        if size > curr_greatest {
            curr_greatest = size;
        }
    }
    curr_greatest
}

pub fn nu_signed(n: u32, m: u32, h: u32) -> u32 {
    let mut curr_greatest = 0;
    for a in each_set_exact(n, m) {
        let size = a.hfoldsignedsumset(h, n).size();
        if size > curr_greatest {
            curr_greatest = size;
        }
    }
    curr_greatest
}

// Behaves according to problem A.18
pub fn nu_signed_exception(n: u32, m: u32, h: u32) -> bool {
    let expected = cmp::min(n, c(h, m));
    nu_signed(n, m, h) == expected
}

pub fn nu_signed_interval(n: u32, m: u32, s: u32) -> u32 {
    let mut curr_greatest = 0;
    for a in each_set_exact(n, m) {
        let size = a.hfoldintervalsignedsumset((0, s), n).size();
        if size > curr_greatest {
            curr_greatest = size;
        }
    }
    curr_greatest
}

pub fn nu_restricted(n: u32, m: u32, h: u32) -> u32 {
    let mut curr_greatest = 0;
    for a in each_set_exact(n, m) {
        let size = a.hfoldrestrictedsumset(h, n).size();
        if size > curr_greatest {
            curr_greatest = size;
        }
    }
    curr_greatest
}

pub fn nu_restricted_exceptions(n: u32) -> u32 {
    let params: Vec<(u32, u32)> = iproduct!(2u32..=n/2, 3u32..=n/2).collect();

    let excepts_number = params.into_par_iter().filter(|(h, m)| {
            if h >= m {
                return false;
            }
            let expected = cmp::min(n, choose(*m, *h));
            nu_restricted(n, *m, *h) != expected
        })
        //.inspect(|(h, m)| println!("Exception: h={}, m={}", h, m))
        .count();

    excepts_number as u32
}

pub fn nu_restricted_interval(n: u32, m: u32, s: u32) -> u32 {
    let mut curr_greatest = 0;
    for a in each_set_exact(n, m) {
        let size = a.hfoldintervalrestrictedsumset((0, s), n).size();
        if size > curr_greatest {
            curr_greatest = size;
        }
    }
    curr_greatest
}

pub fn nu_signed_restricted(n: u32, m: u32, h: u32) -> u32 {
    let mut curr_greatest = 0;
    for a in each_set_exact(n, m) {
        let size = a.hfoldrestrictedsignedsumset(h, n).size();
        if size > curr_greatest {
            curr_greatest = size;
        }
    }
    curr_greatest
}

pub fn nu_signed_restricted_exceptions(n: u32) -> u32 {
    let params: Vec<(u32, u32)> = iproduct!(2u32..n/2, 3u32..n/2).collect();

    let excepts_number = params.into_par_iter().filter(|(h, m)| {
            if h >= m {
                return false;
            }
            let expected = cmp::min(n, choose(*m, *h)*(2u32).pow(*h));
            nu_signed_restricted(n, *m, *h) != expected
        })
        // .inspect(|(h, m)| println!("Exception: h={}, m={}", h, m))
        .count();

    excepts_number as u32
}

pub fn nu_signed_restricted_interval(n: u32, m: u32, s: u32) -> u32 {
    let mut curr_greatest = 0;
    for a in each_set_exact(n, m) {
        let size = a.hfoldintervalrestrictedsignedsumset((0, s), n).size();
        if size > curr_greatest {
            curr_greatest = size;
        }
    }
    curr_greatest
}

#[cfg(test)]
mod tests {
    use super::*;

    // Recreate the table on page 114 (for Problem A.18)
    #[test]
    pub fn test_nu_signed() {
        let m = 3;
        for n in 10..22 {
            println!("{}: {}", n, nu_signed_exception(n, m, 2));
        }
    }

    // Partially recreate table on page 117 (for Problem A.31)
    #[test]
    pub fn test_nu_restricted() {
        for n in 8..=21 {
            println!("{}: {}", n, nu_restricted_exceptions(n));
        }
    }

    #[test]
    pub fn test_nu_signed_restricted() {
        for n in 8..=15 {
            println!("{}: {}", n, nu_signed_restricted_exceptions(n));
        }
    }
}