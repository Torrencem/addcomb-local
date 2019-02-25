use std::cmp;
use rayon::prelude::*;
use fastset::*;
use comb::*;

pub fn nu(n: u32, m: u32, h: u32) -> u32 {
    let mut greatest_set = empty_set();
    let mut curr_greatest = 0;
    for a in each_set_exact(n, m) {
        let size = a.hfoldsumset(h, n).size();
        if size > curr_greatest {
            if size == n {
                info!("[nu] Found spanning set: {}", a);
                return n;
            }
            curr_greatest = size;
            greatest_set = a;
        }
    }
    info!("[nu] Set with greatest sumset: {}", greatest_set);
    info!("[nu] (sumsets to:) {}", greatest_set.hfoldsumset(h, n));
    curr_greatest
}

pub fn nu_exceptions(n: u32) -> u32 {
    // E.1.1 for critical numbers for bounds
    let params: Vec<(u32, u32)> = iproduct!(2u32..=n/2, 3u32..=n/2).collect();

    let excepts_number = params.into_par_iter().filter(|(h, m)| {
            let expected = cmp::min(n, choose(m + h - 1, *h));
            nu(n, *m, *h) != expected
        })
        .inspect(|(h, m)| info!("Exception: h={}, m={}", h, m))
        .count();

    excepts_number as u32
}

pub fn nu_interval(n: u32, m: u32, intv: (u32, u32)) -> u32 {
    let mut greatest_set = empty_set();
    let mut curr_greatest = 0;
    for a in each_set_exact(n, m) {
        let size = a.hfoldintervalsumset(intv, n).size();
        if size > curr_greatest {
            if size == n {
                info!("[nu] Found spanning set: {}", a);
                return n;
            }
            curr_greatest = size;
            greatest_set = a;
        }
    }
    info!("[nu] Set with greatest sumset: {}", greatest_set);
    info!("[nu] (sumsets to:) {}", greatest_set.hfoldintervalsumset(intv, n));
    curr_greatest
}

pub fn nu_signed(n: u32, m: u32, h: u32) -> u32 {
    let mut greatest_set = empty_set();
    let mut curr_greatest = 0;
    for a in each_set_exact(n, m) {
        let size = a.hfoldsignedsumset(h, n).size();
        if size > curr_greatest {
            if size == n {
                info!("[nu] Found spanning set: {}", a);
                return n;
            }
            curr_greatest = size;
            greatest_set = a;
        }
    }
    info!("[nu] Set with greatest sumset: {}", greatest_set);
    info!("[nu] (sumsets to:) {}", greatest_set.hfoldsignedsumset(h, n));
    curr_greatest
}

// Behaves according to problem A.18
pub fn nu_signed_exception(n: u32, m: u32, h: u32) -> bool {
    let expected = cmp::min(n, c(h, m));
    nu_signed(n, m, h) != expected
}

pub fn nu_signed_interval(n: u32, m: u32, intv: (u32, u32)) -> u32 {
    let mut greatest_set = empty_set();
    let mut curr_greatest = 0;
    for a in each_set_exact(n, m) {
        let size = a.hfoldintervalsignedsumset(intv, n).size();
        if size > curr_greatest {
            if size == n {
                info!("[nu] Found spanning set: {}", a);
                return n;
            }
            curr_greatest = size;
            greatest_set = a;
        }
    }
    info!("[nu] Set with greatest sumset: {}", greatest_set);
    info!("[nu] (sumsets to:) {}", greatest_set.hfoldintervalsignedsumset(intv, n));
    curr_greatest
}

pub fn nu_restricted(n: u32, m: u32, h: u32) -> u32 {
    let mut greatest_set = empty_set();
    let mut curr_greatest = 0;
    for a in each_set_exact(n, m) {
        let size = a.hfoldrestrictedsumset(h, n).size();
        if size > curr_greatest {
            if size == n {
                info!("[nu] Found spanning set: {}", a);
                return n;
            }
            curr_greatest = size;
            greatest_set = a;
        }
    }
    info!("[nu] Set with greatest sumset: {}", greatest_set);
    info!("[nu] (sumsets to:) {}", greatest_set.hfoldrestrictedsumset(h, n));
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
        .inspect(|(h, m)| info!("Exception: h={}, m={}", h, m))
        .count();

    excepts_number as u32
}

pub fn nu_restricted_interval(n: u32, m: u32, intv: (u32, u32)) -> u32 {
    let mut greatest_set = empty_set();
    let mut curr_greatest = 0;
    for a in each_set_exact(n, m) {
        let size = a.hfoldintervalrestrictedsumset(intv, n).size();
        if size > curr_greatest {
            if size == n {
                info!("[nu] Found spanning set: {}", a);
                return n;
            }
            curr_greatest = size;
            greatest_set = a;
        }
    }
    info!("[nu] Set with greatest sumset: {}", greatest_set);
    info!("[nu] (sumsets to:) {}", greatest_set.hfoldintervalrestrictedsumset(intv, n));
    curr_greatest
}

pub fn nu_signed_restricted(n: u32, m: u32, h: u32) -> u32 {
    let mut greatest_set = empty_set();
    let mut curr_greatest = 0;
    for a in each_set_exact(n, m) {
        let size = a.hfoldrestrictedsignedsumset(h, n).size();
        if size > curr_greatest {
            if size == n {
                info!("[nu] Found spanning set: {}", a);
                return n;
            }
            curr_greatest = size;
            greatest_set = a;
        }
    }
    info!("[nu] Set with greatest sumset: {}", greatest_set);
    info!("[nu] (sumsets to:) {}", greatest_set.hfoldrestrictedsignedsumset(h, n));
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

pub fn nu_signed_restricted_interval(n: u32, m: u32, intv: (u32, u32)) -> u32 {
    let mut greatest_set = empty_set();
    let mut curr_greatest = 0;
    for a in each_set_exact(n, m) {
        let size = a.hfoldintervalrestrictedsignedsumset(intv, n).size();
        if size > curr_greatest {
            if size == n {
                info!("[nu] Found spanning set: {}", a);
                return n;
            }
            curr_greatest = size;
            greatest_set = a;
        }
    }
    info!("[nu] Set with greatest sumset: {}", greatest_set);
    info!("[nu] (sumsets to:) {}", greatest_set.hfoldintervalrestrictedsignedsumset(intv, n));
    curr_greatest
}

#[cfg(test)]
mod tests {
    use super::*;

    // Based on page 109
    #[test]
    pub fn test_nu_exceptions() {
        let correct_table: Vec<u32> = vec![0, 0, 0, 0, 1, 0, 0, 1, 2, 2, 3, 1,
                                2, 2, 2, 4, 5, 3];
        assert_eq!((2..=19).map(|n| nu_exceptions(n)).collect::<Vec<u32>>(),
                    correct_table);
    }

    // Page 111
    #[test]
    pub fn test_nu_interval() {
        for n in [6, 10, 17].iter() {
            for m in 3..5 {
                for s in 1..3 {
                    assert!(nu_interval(*n, m, (0, s)) == nu(*n, m + 1, s));
                }
            }
        }
    }

    // Recreate the table on page 114 (for Problem A.18)
    #[test]
    pub fn test_nu_signed_exceptions() {
        let positives: Vec<(u32, u32)> = vec![(2, 7), (3, 14), (3, 16), (3, 18), (3, 20), (4, 22), (4, 33), (5, 34)];
        let negatives: Vec<(u32, u32)> = vec![(2, 5), (2, 9), (3, 19), (4, 34), (3, 21)];
        let h = 2;
        for (m, n) in positives {
            assert!(nu_signed_exception(n, m, h));
        }
        for (m, n) in negatives {
            assert!(!nu_signed_exception(n, m, h));
        }
    }

    // Partially recreate table on page 117 (for Problem A.31)
    #[test]
    pub fn test_nu_restricted() {
        let correct_table: Vec<u32> = vec![2, 0, 0, 0, 2, 2, 2, 2, 4];
        assert_eq!((10..=18).map(|n| nu_restricted_exceptions(n)).collect::<Vec<u32>>(),
                    correct_table);
    }
}