
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

#[cfg(test)]
mod tests {
    use super::*;

    // table on page 129
    #[test]
    pub fn test_phi_interval() {
        let m = 4;
        for n in 1..50 {
            println!("{}: {}", n, phi_interval_0s(n, m));
        }
    }
}