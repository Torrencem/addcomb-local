
use fastset::*;

pub fn phi(n: u32, h: u32) -> u32 {
    for m in 2u32.. {
        if m as u32 >= n {
            info!("[phi] No non-trivial spanning set found");
            return m;
        }
        for a in each_set_exact(n as u32, m) {
            if a.hfoldsumset(h, n).isfull(n) {
                info!("[phi] Found spanning set: {}", a);
                return m;
            }
        }
    }
    panic!();
}

#[inline]
pub fn phi_interval(n: u32, s: u32) -> u32 {
    for m in 1u32.. {
        if m as u32 >= n {
            info!("[phi] No non-trivial spanning set found");
            return m;
        }
        for a in each_set_exact(n as u32, m) {
            if a.hfoldintervalsumset((0, s), n).isfull(n) {
                info!("[phi] Found spanning set: {}", a);
                return m;
            }
        }
    }
    panic!();
}

// TODO: Maybe impliment f and g functions on page 131
//(need an upper bound on n though, maybe read paper?)

pub fn phi_signed(n: u32, h: u32) -> u32 {
    for m in 2u32.. {
        if m as u32 >= n {
            info!("[phi] No non-trivial spanning set found");
            return m;
        }
        for a in each_set_exact(n as u32, m) {
            if a.hfoldsignedsumset(h, n).isfull(n) {
                info!("[phi] Found spanning set: {}", a);
                return m;
            }
        }
    }
    panic!();
}

pub fn phi_signed_interval(n: u32, s: u32) -> u32 {
    for m in 1u32.. {
        if m as u32 >= n {
            info!("[phi] No non-trivial spanning set found");
            return m;
        }
        for a in each_set_exact(n as u32, m) {
            if a.hfoldintervalsignedsumset((0, s), n).isfull(n) {
                info!("[phi] Found spanning set: {}", a);
                return m;
            }
        }
    }
    panic!();
}

// Not a very researched function... (page 145)
pub fn phi_restricted(n: u32, h: u32) -> u32 {
    for m in 2u32.. {
        if m as u32 >= n {
            info!("[phi] No non-trivial spanning set found");
            return m;
        }
        for a in each_set_exact(n as u32, m) {
            if a.hfoldrestrictedsumset(h, n).isfull(n) {
                info!("[phi] Found spanning set: {}", a);
                return m;
            }
        }
    }
    panic!();
}

pub fn phi_restricted_interval(n: u32, s: u32) -> u32 {
    for m in 1u32.. {
        if m as u32 >= n {
            info!("[phi] No non-trivial spanning set found");
            return m;
        }
        for a in each_set_exact(n as u32, m) {
            if a.hfoldintervalrestrictedsumset((0, s), n).isfull(n) {
                info!("[phi] Found spanning set: {}", a);
                return m;
            }
        }
    }
    panic!();
}

pub fn phi_signed_restricted(n: u32, h: u32) -> u32 {
    for m in 2u32.. {
        if m as u32 >= n {
            info!("[phi] No non-trivial spanning set found");
            return m;
        }
        for a in each_set_exact(n as u32, m) {
            if a.hfoldrestrictedsignedsumset(h, n).isfull(n) {
                info!("[phi] Found spanning set: {}", a);
                return m;
            }
        }
    }
    panic!();
}

pub fn phi_signed_restricted_interval(n: u32, s: u32) -> u32 {
    for m in 1u32.. {
        if m as u32 >= n {
            info!("[phi] No non-trivial spanning set found");
            return m;
        }
        for a in each_set_exact(n as u32, m) {
            if a.hfoldintervalrestrictedsignedsumset((0, s), n).isfull(n) {
                info!("[phi] Found spanning set: {}", a);
                return m;
            }
        }
    }
    panic!();
}

#[cfg(test)]
mod tests {
    use super::*;

    // From table on page 129
    #[test]
    pub fn test_phi_interval() {
        assert_eq!(phi_interval(17, 2), 5);
        assert_eq!(phi_interval(18, 2), 6);
        assert_eq!(phi_interval(9, 3),  3);
        assert_eq!(phi_interval(8, 3),  2);
        assert_eq!(phi_interval(26, 4), 3);
        assert_eq!(phi_interval(49, 4), 4);
    }

    // From table on page 147
    #[test]
    pub fn test_phi_restricted_interval() {
        assert_eq!(phi_restricted_interval(10, 2), 4);
        assert_eq!(phi_restricted_interval(3, 2), 2);
        assert_eq!(phi_restricted_interval(5, 3), 3);
        assert_eq!(phi_restricted_interval(8, 4), 3);
        assert_eq!(phi_restricted_interval(9, 4), 4);
    }
}