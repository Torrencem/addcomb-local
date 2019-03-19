use fastset::*;
use wasm_result::*;

macro_rules! info {
    ($res_var:ident, $( $arg:tt )+) => {
        $res_var.push_line(format!($($arg)+));
    };
}

#[no_mangle]
pub fn chi(n: u32, h: u32) -> RawCString {
   let mut result = WasmResult::new();
   for m in 1.. {
        let mut found = false;
        for a in each_set_exact(n, m) {
            if !a.hfoldsumset(h, n).isfull(n) {
                info!(result, "[chi] For m={}, found {}, which doesn't give a full sumset", m, a);
                info!(result, "[chi] (gives:) {}", a.hfoldsumset(h, n));
                found = true;
                break;
            }
        }
        if !found {
            return result.solve(m);
        }
    }
    unreachable!();
}

#[no_mangle]
pub fn chi_interval(n: u32, ia: u32, ib: u32) -> RawCString {
   let mut result = WasmResult::new();
   for m in 1.. {
        let mut found = false;
        for a in each_set_exact(n, m) {
            if !a.hfoldintervalsumset((ia, ib), n).isfull(n) {
                info!(result, "[chi] For m={}, found {}, which doesn't give a full sumset", m, a);
                info!(result, "[chi] (gives:) {}", a.hfoldintervalsumset((ia, ib), n));
                found = true;
                break;
            }
        }
        if !found {
            return result.solve(m);
        }
    }
    unreachable!();
}

#[no_mangle]
pub fn chi_signed(n: u32, h: u32) -> RawCString {
   let mut result = WasmResult::new();
   for m in 1.. {
        let mut found = false;
        for a in each_set_exact(n, m) {
            if !a.hfoldsignedsumset(h, n).isfull(n) {
                info!(result, "[chi] For m={}, found {}, which doesn't give a full sumset", m, a);
                info!(result, "[chi] (gives:) {}", a.hfoldsignedsumset(h, n));
                found = true;
                break;
            }
        }
        if !found {
            return result.solve(m);
        }
    }
    unreachable!();
}

#[no_mangle]
pub fn chi_signed_interval(n: u32, ia: u32, ib: u32) -> RawCString {
   let mut result = WasmResult::new();
   for m in 1.. {
        let mut found = false;
        for a in each_set_exact(n, m) {
            if !a.hfoldintervalsignedsumset((ia, ib), n).isfull(n) {
                info!(result, "[chi] For m={}, found {}, which doesn't give a full sumset", m, a);
                info!(result, "[chi] (gives:) {}", a.hfoldintervalsignedsumset((ia, ib), n));
                found = true;
                break;
            }
        }
        if !found {
            return result.solve(m);
        }
    }
    unreachable!();
}

#[no_mangle]
pub fn chi_restricted(n: u32, h: u32) -> RawCString {
   let mut result = WasmResult::new();
   for m in 1.. {
        let mut found = false;
        for a in each_set_exact(n, m) {
            if !a.hfoldrestrictedsumset(h, n).isfull(n) {
                info!(result, "[chi] For m={}, found {}, which doesn't give a full sumset", m, a);
                info!(result, "[chi] (gives:) {}", a.hfoldrestrictedsumset(h, n));
                found = true;
                break;
            }
        }
        if !found {
            return result.solve(m);
        }
    }
    unreachable!();
}

#[no_mangle]
pub fn chi_restricted_interval(n: u32, ia: u32, ib: u32) -> RawCString {
   let mut result = WasmResult::new();
   for m in 1.. {
        let mut found = false;
        for a in each_set_exact(n, m) {
            if !a.hfoldintervalrestrictedsumset((ia, ib), n).isfull(n) {
                info!(result, "[chi] For m={}, found {}, which doesn't give a full sumset", m, a);
                info!(result, "[chi] (gives:) {}", a.hfoldintervalrestrictedsumset((ia, ib), n));
                found = true;
                break;
            }
        }
        if !found {
            return result.solve(m);
        }
    }
    unreachable!();
}

#[no_mangle]
pub fn chi_signed_restricted(n: u32, h: u32) -> RawCString {
   let mut result = WasmResult::new();
   for m in 1.. {
        let mut found = false;
        for a in each_set_exact(n, m) {
            if !a.hfoldrestrictedsignedsumset(h, n).isfull(n) {
                info!(result, "[chi] For m={}, found {}, which doesn't give a full sumset", m, a);
                info!(result, "[chi] (gives:) {}", a.hfoldrestrictedsignedsumset(h, n));
                found = true;
                break;
            }
        }
        if !found {
            return result.solve(m);
        }
    }
    unreachable!();
}

#[no_mangle]
pub fn chi_signed_restricted_interval(n: u32, ia: u32, ib: u32) -> RawCString {
   let mut result = WasmResult::new();
   for m in 1.. {
        let mut found = false;
        for a in each_set_exact(n, m) {
            if !a.hfoldintervalrestrictedsignedsumset((ia, ib), n).isfull(n) {
                info!(result, "[chi] For m={}, found {}, which doesn't give a full sumset", m, a);
                info!(result, "[chi] (gives:) {}", a.hfoldintervalrestrictedsignedsumset((ia, ib), n));
                found = true;
                break;
            }
        }
        if !found {
            return result.solve(m);
        }
    }
    unreachable!();
}