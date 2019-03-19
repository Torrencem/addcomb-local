use fastset::*;
use wasm_result::*;

macro_rules! info {
    ($res_var:ident, $( $arg:tt )+) => {
        $res_var.push_line(format!($($arg)+));
    };
}

#[no_mangle]
pub fn tau(n: u32, h: u32) -> RawCString {
    let mut result = WasmResult::new();
    for m in (1..=n).rev() {
        let mut found = false;
        for a in each_set_exact_no_zero(n, m) {
            if !a.hfoldsumset(h, n).access(0) {
                info!(result, "Found {}, which gives a zero-free sumset", a);
                info!(result, "(gives:) {}", a.hfoldsumset(h, n));
                found = true;
                break;
            }
        }
        if found {
            return result.solve(m);
        }
    }
    unreachable!();
}

#[no_mangle]
pub fn tau_interval(n: u32, ia: u32, ib: u32) -> RawCString {
    let mut result = WasmResult::new();
    for m in (1..=n).rev() {
        let mut found = false;
        for a in each_set_exact_no_zero(n, m) {
            if !a.hfoldintervalsumset((ia, ib), n).access(0) {
                info!(result, "Found {}, which gives a zero-free sumset", a);
                info!(result, "(gives:) {}", a.hfoldintervalsumset((ia, ib), n));
                found = true;
                break;
            }
        }
        if found {
            return result.solve(m);
        }
    }
    unreachable!();
}

#[no_mangle]
pub fn tau_restricted(n: u32, h: u32) -> RawCString {
    let mut result = WasmResult::new();
    // Theorem F.88
    if n >= 12 && n % 2 == 0 && (3 <= h) && (h <= n - 1) && (h % 2 == 1) {
        if h == 1 {
            return result.solve(n - 1);
        }
        if (3 <= h) && (h <= n/2 - 2) {
            return result.solve(n / 2);
        }
        if h == n/2 - 1 {
            return result.solve(n / 2 + 1);
        }
        if (n/2 <= h) && (h <= n - 2) {
            return result.solve(h + 1);
        }
        // h = n - 1 (guaranteed)
        return result.solve(n - 1);
    }
    if n == 1 {
        return result.solve(1);
    }
    for m in (1..=n).rev() {
        let mut found = false;
        for a in each_set_exact(n, m) {
            if !a.hfoldrestrictedsumset(h, n).access(0) {
                info!(result, "Found {}, which gives a zero-free sumset", a);
                info!(result, "(gives:) {}", a.hfoldrestrictedsumset(h, n));
                found = true;
                break;
            }
        }
        if found {
            return result.solve(m);
        }
    }
    unreachable!();
}

#[no_mangle]
pub fn tau_restricted_interval(n: u32, ia: u32, ib: u32) -> RawCString {
    let mut result = WasmResult::new();
    for m in (1..=n).rev() {
        let mut found = false;
        for a in each_set_exact(n, m) {
            if !a.hfoldintervalrestrictedsumset((ia, ib), n).access(0) {
                info!(result, "Found {}, which gives a zero-free sumset", a);
                info!(result, "(gives:) {}", a.hfoldintervalrestrictedsumset((ia, ib), n));
                found = true;
                break;
            }
        }
        if found {
            return result.solve(m);
        }
    }
    unreachable!();
}

#[no_mangle]
pub fn tau_signed(n: u32, h: u32) -> RawCString {
    let mut result = WasmResult::new();
    for m in (1..=n).rev() {
        let mut found = false;
        for a in each_set_exact_no_zero(n, m) {
            if !a.hfoldsignedsumset(h, n).access(0) {
                info!(result, "Found {}, which gives a zero-free sumset", a);
                info!(result, "(gives:) {}", a.hfoldsignedsumset(h, n));
                found = true;
                break;
            }
        }
        if found {
            return result.solve(m);
        }
    }
    unreachable!();
}

#[no_mangle]
pub fn tau_signed_interval(n: u32, ia: u32, ib: u32) -> RawCString {
    let mut result = WasmResult::new();
    for m in (1..n).rev() {
        let mut found = false;
        for a in each_set_exact_no_zero(n, m) {
            if !a.hfoldintervalsignedsumset((ia, ib), n).access(0) {
                info!(result, "Found {}, which gives a zero-free sumset", a);
                info!(result, "(gives:) {}", a.hfoldintervalsignedsumset((ia, ib), n));
                found = true;
                break;
            }
        }
        if found {
            return result.solve(m);
        }
    }
    unreachable!();
}

#[no_mangle]
pub fn tau_signed_restricted(n: u32, h: u32) -> RawCString {
    let mut result = WasmResult::new();
    for m in (1..=n).rev() {
        let mut found = false;
        for a in each_set_exact(n, m) {
            if !a.hfoldrestrictedsignedsumset(h, n).access(0) {
                info!(result, "Found {}, which gives a zero-free sumset", a);
                info!(result, "(gives:) {}", a.hfoldrestrictedsignedsumset(h, n));
                found = true;
                break;
            }
        }
        if found {
            return result.solve(m);
        }
    }
    unreachable!();
}

#[no_mangle]
pub fn tau_signed_restricted_interval(n: u32, ia: u32, ib: u32) -> RawCString {
    let mut result = WasmResult::new();
    for m in (1..=n).rev() {
        let mut found = false;
        for a in each_set_exact(n, m) {
            if !a.hfoldintervalrestrictedsignedsumset((ia, ib), n).access(0) {
                info!(result, "Found {}, which gives a zero-free sumset", a);
                info!(result, "(gives:) {}", a.hfoldintervalrestrictedsignedsumset((ia, ib), n));
                found = true;
                break;
            }
        }
        if found {
            return result.solve(m);
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    // Page 297
    // #[test]
    // fn test_tau_restricted() {
    //     let correct_table: Vec<u32> = vec![1, 2, 2, 3, 4, 4, 4, 5, 6, 6, 6, 6, 6, 7, 8, 8, 8, 9];
    //     let actual_table: Vec<u32> = (1..=18).map(|n| {
    //         tau_restricted(n, 3)
    //     }).collect();
    //     assert_eq!(correct_table, actual_table);
    // }
}