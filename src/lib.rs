use std::fmt;
use std::u64::MAX;
use std::cmp;

extern crate rayon;

use rayon::prelude::*;

#[macro_use] extern crate itertools;

#[inline]
fn bit_scan_low(val: u64) -> u8 {
    return val.trailing_zeros() as u8;
}

fn choose(n: u32, k: u32) -> u32 {
    if k == 0 {
        1
    } else {
        (n * choose(n - 1, k - 1)) / k
    }
}

#[derive(Copy, Clone)]
pub struct FastSet {
    contents: u64
}

pub fn singleton(i: u8) -> FastSet {
    return FastSet { contents: (1u64 << i) };
}

pub fn empty_set() -> FastSet {
    return FastSet { contents: 0u64 };
}

#[inline]
pub fn cycle(scontents: u64, i: u8, m: u8) -> u64 {
    let mut ret = scontents;
    let mut wrapped: u64 = MAX << (m - i); // Mask the elements which will get wrapped around
    wrapped &= ret;
    wrapped >>= m - i;
    ret <<= i;
    ret |= wrapped;
    ret &= !(MAX << m);
    ret
}

#[inline]
pub fn cycle_rev(scontents: u64, i: u8, m: u8) -> u64 {
    cycle(scontents, m - i, m)
}

impl FastSet {
    #[inline]
    pub fn access(&self, i: u8) -> bool{
        // assert!(i < 64);
        return self.contents & (1u64 << i) > 0;
    }

    #[inline]
    pub fn add(&mut self, i: u8) {
        self.contents |= 1u64 << i;
    }

    #[inline]
    pub fn isfull(&self, n: u8) -> bool {
        // Tests if the set is full up to (and including) n
        (!(self.contents & ((1u64 << (n + 1)) - 1)) << (64 - n)) == 0
    }

    #[inline]
    pub fn isempty(&self) -> bool {
        self.contents == 0u64
    }

    #[inline]
    pub fn size(&self) -> u8 {
        return self.contents.count_ones() as u8;
    }

    #[inline]
    pub fn union(&mut self, other: &FastSet) {
        self.contents |= other.contents;
    }

    #[inline]
    pub fn intersect(&mut self, other: &FastSet) {
        self.contents &= other.contents;
    }

    #[inline]
    pub fn cycle(&mut self, i: u8, m: u8) {
        // Add i (mod n) to every element of the set
        assert!(i < m);
        self.contents = cycle(self.contents, i, m);
    }

    #[inline]
    pub fn simplesumset(&self, other: &FastSet, m: u8) -> FastSet {
        let mut res = 0u64;
        let mut c1 = self.contents;
        while c1 != 0 {
            let shift = bit_scan_low(c1);
            let cycled = cycle(other.contents, shift, m);
            res |= cycled;

            c1 &= c1 - 1; // Bit magic
        }
        FastSet {contents: res}
    }

    #[inline]
    pub fn hfoldsumset(&self, h: u8, m: u8) -> FastSet {
        if h == 0 {
            return singleton(0);
        }
        let mut res = 0u64;
        let mut prev = 1u64;
        for _ in 0..h {
            let mut c1 = self.contents;
            while c1 != 0 {
                let shift = bit_scan_low(c1);
                let cycled = cycle(prev, shift, m);
                res |= cycled;

                c1 &= c1 - 1;
            }
            prev = res;
            res = 0u64;
        }
        FastSet {contents: prev}
    }
}

pub struct EachSet {
    state: u64,
    cap: u64,
}

impl Iterator for EachSet {
    type Item = FastSet;

    fn next(&mut self) -> Option<FastSet> {
        let curr = self.state;
        if curr >= self.cap {
            None
        } else {
            self.state += 1;
            Some(FastSet {contents: self.state - 1})
        }
    }
}

// Note to initialize this struct correctly
pub struct EachSetExact {
    state: u64,
    setmask: u64,
    doneflag: bool,
}

impl Iterator for EachSetExact {
    type Item = FastSet;

    // Based on ideas from https://stackoverflow.com/a/29914908/6504760
    fn next(&mut self) -> Option<FastSet> {
        if self.doneflag {
            return None;
        }
        // Find the greatest number which can be moved to the left
        let can_be_moved_left = self.state & !(self.state >> 1) & !(self.setmask >> 1);
        let first_moveable = 64 - can_be_moved_left.leading_zeros();
        if first_moveable == 0 {
            self.doneflag = true;
            return Some(FastSet { contents: self.state });
        }
        let update_region = !((1 << (first_moveable - 1)) - 1) & !self.setmask;
        let to_fill_left = (self.state & update_region).count_ones() - 1;
        
        let old = self.state;
        // Clear the updated region
        self.state &= !update_region;
        let newregion = ((1 << (to_fill_left + 1)) - 1) << first_moveable;
        self.state |= newregion;

        Some(FastSet { contents: old })
    }
}

pub fn each_set(max_size: u8) -> EachSet {
    return EachSet { state: 0, cap: (1u64 << max_size) }
}

pub fn each_set_exact(max_size: u32, set_size: u32) -> EachSetExact {
    assert!(max_size > set_size);
    let naivestate = (1u64 << (set_size)) - 1;
    let setmask = !((1u64 << (max_size + 1)) - 1);
    return EachSetExact {state: naivestate, setmask: setmask, doneflag: false}
}

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

pub fn nu_exceptions_seq(n: u32) -> u32 {
    let mut curr_exps = 0;
    for h in 2..=(n/2 + 1) {
        for m in 3..=n/2 {
            let expected = cmp::min(n, choose(m + h - 1, h));
            if nu(n as u8, m, h as u8) != expected as u8 {
                curr_exps += 1;
                // println!("{}, {}, {}", n, h, m);
                // println!("exp: {}, act: {}", expected, nu(n as u8, m, h as u8));
            } else {
                // println!("OK {}, {}, {}", n, h, m);
            }
        }
    }
    curr_exps
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

impl fmt::Debug for FastSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FastSet {:?}", (0..64).filter(|n| self.access(*n)).collect::<Vec<u8>>())
    }
}

impl<'a> From<&'a [u8]> for FastSet {
    fn from(vals: &[u8]) -> Self {
        let mut me = empty_set();
        for val in vals {
            me.add(*val);
        }
        me
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sets() {
        let mut s = FastSet::from(&[2, 4, 10][..]);
        s.cycle(5, 11);
        println!("{:?}", s);
    }

    #[test]
    fn test_sumsets() {
        let s1 = FastSet::from(&[1, 3][..]);
        let s2 = FastSet::from(&[2, 4, 5][..]);
        let s3 = s1.simplesumset(&s2, 10);
        println!("{:?}", s3);
    }

    #[test]
    fn test_hfolds() {
        let s1 = FastSet::from(&[2, 3][..]);
        for iter in 0..=12 {
            println!("{}A = {:?}", iter, s1.hfoldsumset(iter, 13));
        }
        println!("");

        assert!(!s1.hfoldsumset(11, 13).isfull(12));
        assert!(s1.hfoldsumset(12, 13).isfull(12));
        // TODO: Maybe more tests of off-by-one isfulls?
    }

    #[test]
    fn test_multipurpose() {
        // Page 133
        // for n in 2..=21 {
        //     println!("n: {}, exceptions: {}", n, nu_exceptions(n));
        // }
        for A in each_set_exact(50, 5) {
            if A.hfoldsumset(3, 50).size() == 20 {
                println!("{:?}", A);
            }
        }
    }

    #[test]
    fn test_conjecture() {
        let h = 3;
        let m = 4;
        for n in 17..=50 {
            let expected = cmp::min(n, choose(m + h - 1, h)) as u8;
            if nu(n as u8, m, h as u8) != expected {
                println!("{}", n);
            }
        }
    }

    #[test]
    fn test_2() {
        println!("{:?}", FastSet::from(&[1,3,8][..]).hfoldsumset(2, 20));
    }

    #[test]
    fn test_phi() {
        for h in 1..10 {
            for n in 2..=9 {
                let mut found = false;
                for A in each_set_exact(10, n) {
                    if A.hfoldsumset(h, 10).isfull(10) {
                        println!("A: {:?}, h: {}, n: {}", A, h, n);
                        found = true;
                        break;
                    }
                }
                if found {
                    break;
                }
            }
        }
    }
}
