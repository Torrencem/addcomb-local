use fastset::*;

// Temporary
macro_rules! info {
    ($( $x:expr ),*) => {
        
    };
}

#[no_mangle]
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

#[no_mangle]
pub fn chi_interval(n: u32, ia: u32, ib: u32) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in each_set_exact(n, m) {
            if !a.hfoldintervalsumset((ia, ib), n).isfull(n) {
                info!("[chi] For m={}, found {}, which doesn't give a full sumset", m, a);
                info!("[chi] (gives:) {}", a.hfoldintervalsumset((ia, ib), n));
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

#[no_mangle]
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

#[no_mangle]
pub fn chi_signed_interval(n: u32, ia: u32, ib: u32) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in each_set_exact(n, m) {
            if !a.hfoldintervalsignedsumset((ia, ib), n).isfull(n) {
                info!("[chi] For m={}, found {}, which doesn't give a full sumset", m, a);
                info!("[chi] (gives:) {}", a.hfoldintervalsignedsumset((ia, ib), n));
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

#[no_mangle]
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

#[no_mangle]
pub fn chi_restricted_interval(n: u32, ia: u32, ib: u32) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in each_set_exact(n, m) {
            if !a.hfoldintervalrestrictedsumset((ia, ib), n).isfull(n) {
                info!("[chi] For m={}, found {}, which doesn't give a full sumset", m, a);
                info!("[chi] (gives:) {}", a.hfoldintervalrestrictedsumset((ia, ib), n));
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

#[no_mangle]
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

#[no_mangle]
pub fn chi_signed_restricted_interval(n: u32, ia: u32, ib: u32) -> u32 {
    for m in 1.. {
        let mut found = false;
        for a in each_set_exact(n, m) {
            if !a.hfoldintervalrestrictedsignedsumset((ia, ib), n).isfull(n) {
                info!("[chi] For m={}, found {}, which doesn't give a full sumset", m, a);
                info!("[chi] (gives:) {}", a.hfoldintervalrestrictedsignedsumset((ia, ib), n));
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