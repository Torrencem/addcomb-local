
use fastset::*;
use comb::*;

pub fn phi(n: u8, h: u8) -> u32 {
    for m in 2u32.. {
        if m as u8 >= n {
            return m;
        }
        for a in each_set_exact(n as u32, m) {
            if a.hfoldsumset(h, n).isfull(n) {
                return m;
            }
        }
    }
    panic!();
}

#[inline]
pub fn phi_interval_0s(n: u8, s: u8) -> u32 {
    phi(n, s) - 1
}

// TODO: Maybe impliment f and g functions on page 131
//(need an upper bound on n though, maybe read paper?)

pub fn phi_signed(n: u8, h: u8) -> u32 {
    for m in 2u32.. {
        if m as u8 >= n {
            return m;
        }
        for a in each_set_exact(n as u32, m) {
            if a.hfoldsignedsumset(h, n).isfull(n) {
                return m;
            }
        }
    }
    panic!();
}

// Not a very researched function... (page 145)
pub fn phi_restricted(n: u8, h: u8) -> u32 {
    for m in 2u32.. {
        if m as u8 >= n {
            return m;
        }
        for a in each_set_exact(n as u32, m) {
            if a.hfoldrestrictedsumset(h, n).isfull(n) {
                return m;
            }
        }
    }
    panic!();
}

pub fn phi_restricted_signed(n: u8, h: u8) -> u32 {
    for m in 2u32.. {
        if m as u8 >= n {
            return m;
        }
        for a in each_set_exact(n as u32, m) {
            if a.hfoldrestrictedsignedsumset(h, n).isfull(n) {
                return m;
            }
        }
    }
    panic!();
}

#[cfg(test)]
mod tests {
    use super::*;

    // table on page 129
    #[test]
    pub fn test_phi_interval() {
        let m = 5;
        for n in 1..60 {
            println!("{}: {}", n, phi_interval_0s(n, m));
        }
    }
}