use fastset::*;

pub fn chi(n: u32, h: u32) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in each_set_exact(n, m) {
            if !a.hfoldsumset(h, n).isfull(n) {
                found = true;
                break;
            }
        }
        if !found {
            return m;
        }
    }
    panic!();
}

pub fn chi_interval(n: u32, s: u32) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in each_set_exact(n, m) {
            if !a.hfoldintervalsumset((0, s), n).isfull(n) {
                found = true;
                break;
            }
        }
        if !found {
            return m;
        }
    }
    panic!();
}

pub fn chi_signed(n: u32, h: u32) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in each_set_exact(n, m) {
            if !a.hfoldsignedsumset(h, n).isfull(n) {
                found = true;
                break;
            }
        }
        if !found {
            return m;
        }
    }
    panic!();
}

pub fn chi_signed_interval(n: u32, s: u32) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in each_set_exact(n, m) {
            if !a.hfoldintervalsignedsumset((0, s), n).isfull(n) {
                found = true;
                break;
            }
        }
        if !found {
            return m;
        }
    }
    panic!();
}

pub fn chi_restricted(n: u32, h: u32) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in each_set_exact(n, m) {
            if !a.hfoldrestrictedsumset(h, n).isfull(n) {
                found = true;
                break;
            }
        }
        if !found {
            return m;
        }
    }
    panic!();
}

pub fn chi_restricted_interval(n: u32, s: u32) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in each_set_exact(n, m) {
            if !a.hfoldintervalrestrictedsumset((0, s), n).isfull(n) {
                found = true;
                break;
            }
        }
        if !found {
            return m;
        }
    }
    panic!();
}

pub fn chi_signed_restricted(n: u32, h: u32) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in each_set_exact(n, m) {
            if !a.hfoldrestrictedsignedsumset(h, n).isfull(n) {
                found = true;
                break;
            }
        }
        if !found {
            return m;
        }
    }
    panic!();
}

pub fn chi_signed_restricted_interval(n: u32, s: u32) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in each_set_exact(n, m) {
            if !a.hfoldintervalrestrictedsignedsumset((0, s), n).isfull(n) {
                found = true;
                break;
            }
        }
        if !found {
            return m;
        }
    }
    panic!();
}