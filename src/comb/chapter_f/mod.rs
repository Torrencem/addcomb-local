use fastset::*;

pub fn tau(n: u32, h: u32) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in each_set_exact_no_zero(n as u32, m as u32) {
            if !a.hfoldsumset(h, n).access(0) {
                found = true;
                break;
            }
        }
        if !found {
            return m - 1;
        }
    }
    panic!();
}

pub fn tau_interval(n: u32, s: u32) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in each_set_exact_no_zero(n as u32, m as u32) {
            if !a.hfoldintervalsumset((1,s), n).access(0) {
                found = true;
                break;
            }
        }
        if !found {
            return m - 1;
        }
    }
    panic!();
}

pub fn tau_restricted(n: u32, h: u32) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in each_set_exact_no_zero(n as u32, m as u32) {
            if !a.hfoldrestrictedsumset(h, n).access(0) {
                found = true;
                break;
            }
        }
        if !found {
            return m - 1;
        }
    }
    panic!();
}

pub fn tau_restricted_interval(n: u32, s: u32) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in each_set_exact_no_zero(n as u32, m as u32) {
            if !a.hfoldintervalrestrictedsumset((1,s), n).access(0) {
                found = true;
                break;
            }
        }
        if !found {
            return m - 1;
        }
    }
    panic!();
}

pub fn tau_signed(n: u32, h: u32) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in each_set_exact_no_zero(n as u32, m as u32) {
            if !a.hfoldsignedsumset(h, n).access(0) {
                found = true;
                break;
            }
        }
        if !found {
            return m - 1;
        }
    }
    panic!();
}

pub fn tau_signed_interval(n: u32, s: u32) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in each_set_exact_no_zero(n as u32, m as u32) {
            if !a.hfoldintervalsignedsumset((1,s), n).access(0) {
                found = true;
                break;
            }
        }
        if !found {
            return m - 1;
        }
    }
    panic!();
}

pub fn tau_signed_restricted(n: u32, h: u32) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in each_set_exact_no_zero(n as u32, m as u32) {
            if !a.hfoldrestrictedsignedsumset(h, n).access(0) {
                found = true;
                break;
            }
        }
        if !found {
            return m - 1;
        }
    }
    panic!();
}

pub fn tau_signed_restricted_interval(n: u32, s: u32) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in each_set_exact_no_zero(n as u32, m as u32) {
            if !a.hfoldintervalrestrictedsignedsumset((1,s), n).access(0) {
                found = true;
                break;
            }
        }
        if !found {
            return m - 1;
        }
    }
    panic!();
}