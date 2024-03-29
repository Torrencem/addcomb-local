use fastset::*;
use comb::*;

use wasm_result::*;

macro_rules! info {
    ($res_var:ident, $( $arg:tt )+) => {
        $res_var.push_line(format!($($arg)+));
    };
}

#[no_mangle]
pub fn sigma(n: u32, h: u32) -> RawCString {
    let mut result = WasmResult::new();
    for m in (1..n).rev() {
        let expected = choose(m + h - 1, h);
        let mut found = false;
        for a in each_set_exact_zero(n, m) {
            if a.hfoldsumset(h, n).size() == expected {
                info!(result, "for m={}, found a={}", m, a);
                found = true;
                break;
            }
        }
        if found {
            return result.solve(m);
        }
    }
    info!(result, "Found no sets of the required size");
    return result.solve(0);
}

#[no_mangle]
pub fn sigma_interval(n: u32, s: u32) -> RawCString {
    let mut result = WasmResult::new();
    for m in (1..n).rev() {
        let expected = choose(m + s, s);
        let mut found = false;
        for a in each_set_exact_zero(n, m) {
            if a.hfoldintervalsumset((0, s), n).size() == expected {
                info!(result, "for m={}, found a={}", m, a);
                found = true;
                break;
            }
        }
        if found {
            return result.solve(m);
        }
    }
    info!(result, "Found no sets of the required size");
    return result.solve(0);
}

#[no_mangle]
pub fn sigma_signed(n: u32, h: u32) -> RawCString {
    let mut result = WasmResult::new();
    for m in (1..n).rev() {
        let expected = c(h, m);
        let mut found = false;
        for a in each_set_exact_zero(n, m) {
            if a.hfoldsignedsumset(h, n).size() == expected {
                info!(result, "for m={}, found a={}", m, a);
                found = true;
                break;
            }
        }
        if found {
            return result.solve(m);
        }
    }
    info!(result, "Found no sets of the required size");
    return result.solve(0);
}

#[no_mangle]
pub fn sigma_signed_interval(n: u32, s: u32) -> RawCString {
    let mut result = WasmResult::new();
    for m in (1..n).rev() {
        let expected = a(m, s);
        let mut found = false;
        for a in each_set_exact_zero(n, m) {
            if a.hfoldintervalsignedsumset((0, s), n).size() == expected {
                info!(result, "for m={}, found a={}", m, a);
                found = true;
                break;
            }
        }
        if found {
            return result.solve(m);
        }
    }
    info!(result, "Found no sets of the required size");
    return result.solve(0);
}

#[no_mangle]
pub fn sigma_restricted(n: u32, h: u32) -> RawCString {
    let mut result = WasmResult::new();
    for m in (1..n).rev() {
        let expected = choose(m, h);
        let mut found = false;
        for a in each_set_exact_zero(n, m) {
            if a.hfoldrestrictedsumset(h, n).size() == expected {
                info!(result, "for m={}, found a={}", m, a);
                found = true;
                break;
            }
        }
        if found {
            return result.solve(m);
        }
    }
    info!(result, "Found no sets of the required size");
    return result.solve(0);
}

#[no_mangle]
pub fn sigma_restricted_interval(n: u32, s: u32) -> RawCString {
    let mut result = WasmResult::new();
    for m in (1..n).rev() {
        let expected = (0..=cmp::min(s, m)).map(|h| choose(m, h)).sum();
        let mut found = false;
        for a in each_set_exact_zero(n, m) {
            if a.hfoldintervalrestrictedsumset((0, s), n).size() == expected {
                info!(result, "for m={}, found a={}", m, a);
                found = true;
                break;
            }
        }
        if found {
            return result.solve(m);
        }
    }
    info!(result, "Found no sets of the required size");
    return result.solve(0);
}

#[no_mangle]
pub fn sigma_signed_restricted(n: u32, h: u32) -> RawCString {
    let mut result = WasmResult::new();
    for m in (1..n).rev() {
        let expected = choose(m, h)*(2u32).pow(h);
        let mut found = false;
        for a in each_set_exact_zero(n, m) {
            if a.hfoldrestrictedsignedsumset(h, n).size() == expected {
                info!(result, "for m={}, found a={}", m, a);
                found = true;
                break;
            }
        }
        if found {
            return result.solve(m);
        }
    }
    info!(result, "Found no sets of the required size");
    return result.solve(0);
}

#[no_mangle]
pub fn sigma_signed_restricted_interval(n: u32, s: u32) -> RawCString {
    let mut result = WasmResult::new();
    for m in (1..n).rev() {
        let expected = (0..=cmp::min(s, m)).map(|h| choose(m, h)*(2u32).pow(h)).sum();
        let mut found = false;
        for a in each_set_exact_zero(n, m) {
            if a.hfoldintervalrestrictedsumset((0, s), n).size() == expected {
                info!(result, "for m={}, found a={}", m, a);
                found = true;
                break;
            }
        }
        if found {
            return result.solve(m);
        }
    }
    info!(result, "Found no sets of the required size");
    return result.solve(0);
}

#[cfg(test)]
mod tests {
    use super::*;

    // Verify examples according to table on page 153 (details page 154)
    // #[test]
    // pub fn test_sigma() {
    //     for n in 5..10 {
    //         let expected = (((4.0 * (n as f64) - 3.0).sqrt() + 1.0) / 2.0).floor() as u32;

    //         assert!(sigma(n, 2) == expected);
    //     }
    // }
}