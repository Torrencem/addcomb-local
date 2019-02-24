use fastset::*;

pub fn chi(n: u32, h: u32) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in each_set_exact(n, m) {
            if !a.hfoldsumset(h, n).isfull(n) {
                info!("[chi] For m={}, found {}, which doesn't give a full sumset", m, a);
                info!("[chi] (gives:) {}", a.hfoldsumset(h, n));
                found = true;
                break;
            }
        }
        if !found {
            return m;
        }
    }
    unreachable!();
}

pub fn chi_interval(n: u32, intv: (u32, u32)) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in each_set_exact(n, m) {
            if !a.hfoldintervalsumset(intv, n).isfull(n) {
                info!("[chi] For m={}, found {}, which doesn't give a full sumset", m, a);
                info!("[chi] (gives:) {}", a.hfoldintervalsumset(intv, n));
                found = true;
                break;
            }
        }
        if !found {
            return m;
        }
    }
    unreachable!();
}

pub fn chi_signed(n: u32, h: u32) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in each_set_exact(n, m) {
            if !a.hfoldsignedsumset(h, n).isfull(n) {
                info!("[chi] For m={}, found {}, which doesn't give a full sumset", m, a);
                info!("[chi] (gives:) {}", a.hfoldsignedsumset(h, n));
                found = true;
                break;
            }
        }
        if !found {
            return m;
        }
    }
    unreachable!();
}

pub fn chi_signed_interval(n: u32, intv: (u32, u32)) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in each_set_exact(n, m) {
            if !a.hfoldintervalsignedsumset(intv, n).isfull(n) {
                info!("[chi] For m={}, found {}, which doesn't give a full sumset", m, a);
                info!("[chi] (gives:) {}", a.hfoldintervalsignedsumset(intv, n));
                found = true;
                break;
            }
        }
        if !found {
            return m;
        }
    }
    unreachable!();
}

pub fn chi_restricted(n: u32, h: u32) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in each_set_exact(n, m) {
            if !a.hfoldrestrictedsumset(h, n).isfull(n) {
                info!("[chi] For m={}, found {}, which doesn't give a full sumset", m, a);
                info!("[chi] (gives:) {}", a.hfoldrestrictedsumset(h, n));
                found = true;
                break;
            }
        }
        if !found {
            return m;
        }
    }
    unreachable!();
}

pub fn chi_restricted_interval(n: u32, intv: (u32, u32)) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in each_set_exact(n, m) {
            if !a.hfoldintervalrestrictedsumset(intv, n).isfull(n) {
                info!("[chi] For m={}, found {}, which doesn't give a full sumset", m, a);
                info!("[chi] (gives:) {}", a.hfoldintervalrestrictedsumset(intv, n));
                found = true;
                break;
            }
        }
        if !found {
            return m;
        }
    }
    unreachable!();
}

pub fn chi_signed_restricted(n: u32, h: u32) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in each_set_exact(n, m) {
            if !a.hfoldrestrictedsignedsumset(h, n).isfull(n) {
                info!("[chi] For m={}, found {}, which doesn't give a full sumset", m, a);
                info!("[chi] (gives:) {}", a.hfoldrestrictedsignedsumset(h, n));
                found = true;
                break;
            }
        }
        if !found {
            return m;
        }
    }
    unreachable!();
}

pub fn chi_signed_restricted_interval(n: u32, intv: (u32, u32)) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in each_set_exact(n, m) {
            if !a.hfoldintervalrestrictedsignedsumset(intv, n).isfull(n) {
                info!("[chi] For m={}, found {}, which doesn't give a full sumset", m, a);
                info!("[chi] (gives:) {}", a.hfoldintervalrestrictedsignedsumset(intv, n));
                found = true;
                break;
            }
        }
        if !found {
            return m;
        }
    }
    unreachable!();
}