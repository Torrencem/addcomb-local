
use fastset::*;



#[inline]
fn bit_scan_low(val: u64) -> u8 {
    return val.trailing_zeros() as u8;
}

impl FastSet {
    #[inline]
    pub fn simplesumset(&self, other: &FastSet, n: u8) -> FastSet {
        let mut res = 0u64;
        let mut c1 = self.contents;
        while c1 != 0 {
            let shift = bit_scan_low(c1);
            let cycled = cycle(other.contents, shift, n);
            res |= cycled;

            c1 &= c1 - 1; // Bit magic
        }
        FastSet {contents: res}
    }

    #[inline]
    pub fn hfoldsumset(&self, h: u8, n: u8) -> FastSet {
        if h == 0 {
            return singleton(0);
        }
        let mut res = 0u64;
        let mut prev = 1u64;
        for _ in 0..h {
            let mut c1 = self.contents;
            while c1 != 0 {
                let shift = bit_scan_low(c1);
                let cycled = cycle(prev, shift, n);
                res |= cycled;

                c1 &= c1 - 1;
            }
            prev = res;
            res = 0u64;
        }
        FastSet {contents: prev}
    }

    #[inline]
    pub fn hfoldrestrictedsumset(&self, h: u8, n: u8) -> FastSet {
        if h > self.size() {
            return empty_set();
        }
        if h == 0 {
            return singleton(0);
        }
        FastSet { contents: _hfrs(self.contents, 1u64, h, n, empty_set()) }
    }

    #[inline]
    pub fn hfoldsignedsumset(&self, h: u8, n: u8) -> FastSet {
        if h == 0 {
            return singleton(0);
        }
        FastSet { contents: _hfss(self.contents, 1u64, h, n, empty_set(), empty_set()) }
    }

    #[inline]
    pub fn hfoldrestrictedsignedsumset(&self, h: u8, n: u8) -> FastSet {
        if h > self.size() {
            return empty_set();
        }
        if h == 0 {
            return singleton(0);
        }
        FastSet { contents: _hfrss(self.contents, 1u64, h, n, empty_set()) }
    }
}

fn _hfrss(stat: u64, curr: u64, h: u8, n: u8, restrictions: FastSet) -> u64 {
    // A 1 in restrictions[i] means i has already been added
    if h == 0 {
        return curr;
    }
    let mut total = 0u64;
    let mut toadd = stat;
    while toadd != 0 {
        let shift = bit_scan_low(toadd);
        if !restrictions.access(shift) {
            let cycled = cycle(curr, shift, n);
            let mut newrestr = restrictions.clone();
            newrestr.add(shift);

            let rec_call = _hfrss(stat, cycled, h - 1, n, newrestr);
            total |= rec_call;

            // Also choose -cycled
            let cycled = cycle_rev(curr, shift, n);
            let mut newrestr = restrictions.clone();
            newrestr.add(shift);

            let rec_call = _hfrss(stat, cycled, h - 1, n, newrestr);
            total |= rec_call;
        }

        toadd &= toadd - 1;
    }
    total
}

fn _hfss(stat: u64, curr: u64, h: u8, n: u8, prestrictions: FastSet, nrestrictions: FastSet) -> u64 {
    if h == 0 {
        return curr;
    }
    let mut total = 0u64;
    let mut toadd = stat;
    while toadd != 0 {
        let shift = bit_scan_low(toadd);
        if !prestrictions.access(shift) {
            let cycled = cycle(curr, shift, n);
            let mut newnrestr = nrestrictions.clone();
            newnrestr.add(shift);

            let rec_call = _hfss(stat, cycled, h - 1, n, prestrictions.clone(), newnrestr);
            total |= rec_call;
        }
        if !nrestrictions.access(shift) {
            let cycled = cycle_rev(curr, shift, n);
            let mut newprestr = prestrictions.clone();
            newprestr.add(shift);

            let rec_call = _hfss(stat, cycled, h - 1, n, newprestr, nrestrictions.clone());
            total |= rec_call;
        }
        toadd &= toadd - 1;
    }
    total
}

fn _hfrs(stat: u64, curr: u64, h: u8, n: u8, restrictions: FastSet) -> u64 {
    // A 1 in restrictions[i] means i has already been added
    if h == 0 {
        return curr;
    }
    let mut total = 0u64;
    let mut toadd = stat;
    while toadd != 0 {
        let shift = bit_scan_low(toadd);
        if !restrictions.access(shift) {
            let cycled = cycle(curr, shift, n);
            let mut newrestr = restrictions.clone();
            newrestr.add(shift);

            let rec_call = _hfrs(stat, cycled, h - 1, n, newrestr);
            total |= rec_call;
        }

        toadd &= toadd - 1;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_hfrs() {
        let a = FastSet::from(&[2, 3][..]);
        let b = a.hfoldrestrictedsignedsumset(2, 13);
        println!("{:?}", b);
    }
}