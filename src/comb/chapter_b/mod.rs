use std::cmp;
use fastset::*;
use comb::*;

pub fn phi(n: u32, h: u32) -> u32 {
    phi_interval(n, h) + 1
}

#[inline]
pub fn phi_interval(n: u32, s: u32) -> u32 {
    // Proposition B.10
    let lower_bound = cmp::max(1, (((factorial(s) * n) as f32).powf(1f32/(s as f32)).ceil() as i32) - (s as i32)) as u32;

    for m in lower_bound.. {
        for a in each_set_exact(n as u32, m) {
            if a.hfoldintervalsumset((0, s), n).isfull(n) {
                info!("[phi] Found spanning set: {}", a);
                return m;
            }
        }
    }
    panic!();
}

// TODO: Maybe impliment f and g functions on page 132
//(need an upper bound on n though, maybe read paper?)

pub fn phi_signed(n: u32, h: u32) -> u32 {
    for m in 2u32.. {
        for a in each_set_exact(n as u32, m) {
            if a.hfoldsignedsumset(h, n).isfull(n) {
                info!("[phi] Found spanning set: {}", a);
                return m;
            }
        }
    }
    panic!();
}

pub fn phi_signed_interval(n: u32, s: u32) -> u32 {
    for m in 1u32.. {
        for a in each_set_exact(n as u32, m) {
            if a.hfoldintervalsignedsumset((0, s), n).isfull(n) {
                info!("[phi] Found spanning set: {}", a);
                return m;
            }
        }
    }
    panic!();
}

// Not a very researched function... (page 145)
pub fn phi_restricted(n: u32, h: u32) -> u32 {
    for m in 2u32.. {
        for a in each_set_exact(n as u32, m) {
            if a.hfoldrestrictedsumset(h, n).isfull(n) {
                info!("[phi] Found spanning set: {}", a);
                return m;
            }
        }
    }
    panic!();
}

pub fn phi_restricted_interval(n: u32, s: u32) -> u32 {
    // Proposition B.73
    let mut lower_bound = 1u32;
    if s == 2 {
        lower_bound = ((((8*n - 7) as f32).sqrt() - 1.0)/2.0).ceil() as u32;
        info!("Found lower bound: {}", lower_bound);
    }
    for m in lower_bound.. {
        for a in each_set_exact(n as u32, m) {
            if a.hfoldintervalrestrictedsumset((0, s), n).isfull(n) {
                info!("[phi] Found spanning set: {}", a);
                return m;
            }
        }
    }
    panic!();
}

pub fn phi_signed_restricted(n: u32, h: u32) -> u32 {
    for m in 2u32.. {
        for a in each_set_exact(n as u32, m) {
            if a.hfoldrestrictedsignedsumset(h, n).isfull(n) {
                info!("[phi] Found spanning set: {}", a);
                return m;
            }
        }
    }
    panic!();
}

pub fn phi_signed_restricted_interval(n: u32, s: u32) -> u32 {
    for m in 1u32.. {
        for a in each_set_exact(n as u32, m) {
            if a.hfoldintervalrestrictedsignedsumset((0, s), n).isfull(n) {
                info!("[phi] Found spanning set: {}", a);
                return m;
            }
        }
    }
    panic!();
}

#[cfg(test)]
mod tests {
    use super::*;

    // From table on page 129
    #[test]
    pub fn test_phi_interval() {
        assert_eq!(phi_interval(17, 2), 5);
        assert_eq!(phi_interval(18, 2), 6);
        assert_eq!(phi_interval(9, 3),  3);
        assert_eq!(phi_interval(8, 3),  2);
        assert_eq!(phi_interval(26, 4), 3);
        assert_eq!(phi_interval(49, 4), 4);
    }

    // From table on page 147
    #[test]
    pub fn test_phi_restricted_interval() {
        assert_eq!(phi_restricted_interval(10, 2), 4);
        assert_eq!(phi_restricted_interval(3, 2), 2);
        assert_eq!(phi_restricted_interval(5, 3), 3);
        assert_eq!(phi_restricted_interval(8, 4), 3);
        assert_eq!(phi_restricted_interval(9, 4), 4);
    }
}