use std::cmp;
use fastset::*;
use comb::*;

use wasm_result::*;

macro_rules! info {
    ($res_var:ident, $( $arg:tt )+) => {
        $res_var.push_line(format!($($arg)+));
    };
}

#[no_mangle]
pub fn phi(n: u32, h: u32) -> RawCString {
    if n == 1 {
        return WasmResult::new().solve(1);
    }
    if h == 1 {
        return WasmResult::new().solve(n);
    }
    let mut res = _phi_interval(n, 0, h);
    res.main_result = res.main_result.map(|val| val + 1);
    res.export()
}

#[no_mangle]
pub fn phi_interval(n: u32, ia: u32, ib: u32) -> RawCString {
    _phi_interval(n, ia, ib).export()
}

fn _phi_interval(n: u32, ia: u32, ib: u32) -> WasmResult {
    let mut result = WasmResult::new();
    let mut lower_bound = 1;
    // Proposition B.10
    if (ia, ib).0 == 0 {
        let s = (ia, ib).1;
        lower_bound = cmp::max(1, (((factorial(s) * n) as f32).powf(1f32/(s as f32)).ceil() as i32) - (s as i32)) as u32;
        info!(result, "(Proposition B.10) Using lower bound: {}", lower_bound);
    }

    for m in lower_bound.. {
        for a in each_set_exact(n, m) {
            if a.hfoldintervalsumset((ia, ib), n).isfull(n) {
                info!(result, "Found spanning set: {}", a);
                result.solve(m);
                return result;
            }
        }
    }
    unreachable!();
}

// TODO: Maybe impliment f and g functions on page 132
//(need an upper bound on n though, maybe read paper?)

#[no_mangle]
pub fn phi_signed(n: u32, h: u32) -> RawCString {
    if n == 1 {
        return WasmResult::new().solve(1);
    }
    let mut result = WasmResult::new();
    for m in 2u32.. {
        for a in each_set_exact(n, m) {
            if a.hfoldsignedsumset(h, n).isfull(n) {
                info!(result, "Found spanning set: {}", a);
                return result.solve(m);
            }
        }
    }
    unreachable!();
}

#[no_mangle]
pub fn phi_signed_interval(n: u32, ia: u32, ib: u32) -> RawCString {
    let mut result = WasmResult::new();
    for m in 1u32.. {
        for a in each_set_exact(n, m) {
            if a.hfoldintervalsignedsumset((ia, ib), n).isfull(n) {
                info!(result, "Found spanning set: {}", a);
                return result.solve(m);
            }
        }
    }
    unreachable!();
}

// Not a very researched function... (page 145)
#[no_mangle]
pub fn phi_restricted(n: u32, h: u32) -> RawCString {
    if n == 1 {
        return WasmResult::new().solve(1);
    }
    if h == 1 {
        return WasmResult::new().solve(n);
    }
    let mut result = WasmResult::new();
    for m in 2u32.. {
        for a in each_set_exact(n, m) {
            if a.hfoldrestrictedsumset(h, n).isfull(n) {
                info!(result, "Found spanning set: {}", a);
                return result.solve(m);
            }
        }
    }
    unreachable!();
}

#[no_mangle]
pub fn phi_restricted_interval(n: u32, ia: u32, ib: u32) -> RawCString {
    let mut result = WasmResult::new();
    let mut lower_bound = 1u32;
    // Proposition B.73
    if (ia, ib) == (0, 2) {
        lower_bound = ((((8*n - 7) as f32).sqrt() - 1.0)/2.0).ceil() as u32;
        info!(result, "(Proposition B.73) Using lower bound: {}", lower_bound);
    }
    for m in lower_bound.. {
        for a in each_set_exact(n, m) {
            if a.hfoldintervalrestrictedsumset((ia, ib), n).isfull(n) {
                info!(result, "Found spanning set: {}", a);
                return result.solve(m);
            }
        }
    }
    unreachable!();
}

#[no_mangle]
pub fn phi_signed_restricted(n: u32, h: u32) -> RawCString {
    let mut result = WasmResult::new();
    for m in 2u32.. {
        for a in each_set_exact(n, m) {
            if a.hfoldrestrictedsignedsumset(h, n).isfull(n) {
                info!(result, "Found spanning set: {}", a);
                return result.solve(m);
            }
        }
    }
    unreachable!();
}

#[no_mangle]
pub fn phi_signed_restricted_interval(n: u32, ia: u32, ib: u32) -> RawCString {
    let mut result = WasmResult::new();
    for m in 1u32.. {
        for a in each_set_exact(n, m) {
            if a.hfoldintervalrestrictedsignedsumset((ia, ib), n).isfull(n) {
                info!(result, "Found spanning set: {}", a);
                return result.solve(m);
            }
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    // From table on page 129
    // #[test]
    // pub fn test_phi_interval() {
    //     assert_eq!(phi_interval(17, 0, 2), 5);
    //     assert_eq!(phi_interval(18, 0, 2), 6);
    //     assert_eq!(phi_interval(9, 0, 3),  3);
    //     assert_eq!(phi_interval(8, 0, 3),  2);
    //     assert_eq!(phi_interval(26, 0, 4), 3);
    //     assert_eq!(phi_interval(49, 0, 4), 4);
    // }

    // From table on page 147
    // #[test]
    // pub fn test_phi_restricted_interval() {
    //     assert_eq!(phi_restricted_interval(10, 0, 2), 4);
    //     assert_eq!(phi_restricted_interval(3, 0, 2), 2);
    //     assert_eq!(phi_restricted_interval(5, 0, 3), 3);
    //     assert_eq!(phi_restricted_interval(8, 0, 4), 3);
    //     assert_eq!(phi_restricted_interval(9, 0, 4), 4);
    // }
}