use fastset::*;

pub fn tau(n: u32, h: u32) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in each_set_exact_no_zero(n, m) {
            if !a.hfoldsumset(h, n).access(0) {
                info!("[tau] For m={}, found {}, which gives a zero-free sumset", m, a);
                info!("[tau] (gives:) {}", a.hfoldsumset(h, n));
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
        for a in each_set_exact_no_zero(n, m) {
            if !a.hfoldintervalsumset((1,s), n).access(0) {
                info!("[tau] For m={}, found {}, which gives a zero-free sumset", m, a);
                info!("[tau] (gives:) {}", a.hfoldintervalsumset((1,s), n));
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
        for a in each_set_exact_no_zero(n, m) {
            if !a.hfoldrestrictedsumset(h, n).access(0) {
                info!("[tau] For m={}, found {}, which gives a zero-free sumset", m, a);
                info!("[tau] (gives:) {}", a.hfoldrestrictedsumset(h, n));
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
        for a in each_set_exact_no_zero(n, m) {
            if !a.hfoldintervalrestrictedsumset((1,s), n).access(0) {
                info!("[tau] For m={}, found {}, which gives a zero-free sumset", m, a);
                info!("[tau] (gives:) {}", a.hfoldintervalrestrictedsumset((1,s), n));
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
        for a in each_set_exact_no_zero(n, m) {
            if !a.hfoldsignedsumset(h, n).access(0) {
                info!("[tau] For m={}, found {}, which gives a zero-free sumset", m, a);
                info!("[tau] (gives:) {}", a.hfoldsignedsumset(h, n));
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
        for a in each_set_exact_no_zero(n, m) {
            if !a.hfoldintervalsignedsumset((1,s), n).access(0) {
                info!("[tau] For m={}, found {}, which gives a zero-free sumset", m, a);
                info!("[tau] (gives:) {}", a.hfoldintervalsignedsumset((1,s), n));
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
        for a in each_set_exact_no_zero(n, m) {
            if !a.hfoldrestrictedsignedsumset(h, n).access(0) {
                info!("[tau] For m={}, found {}, which gives a zero-free sumset", m, a);
                info!("[tau] (gives:) {}", a.hfoldrestrictedsignedsumset(h, n));
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
        for a in each_set_exact_no_zero(n, m) {
            if !a.hfoldintervalrestrictedsignedsumset((1,s), n).access(0) {
                info!("[tau] For m={}, found {}, which gives a zero-free sumset", m, a);
                info!("[tau] (gives:) {}", a.hfoldintervalrestrictedsignedsumset((1,s), n));
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