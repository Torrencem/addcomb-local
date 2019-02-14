
use fastset::*;

pub fn choose(n: u32, k: u32) -> u32 {
    if k == 0 {
        1
    } else {
        (n * choose(n - 1, k - 1)) / k
    }
}

#[inline]
fn bit_scan_low(val: u64) -> u8 {
    return val.trailing_zeros() as u8;
}

impl FastSet {
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