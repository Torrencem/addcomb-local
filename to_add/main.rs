use std;

pub mod lib;
use lib::*;

use std::ffi::CString;
use std::os::raw::c_char;

fn main() {
}

fn exp_str(s: String) -> *mut c_char {
    let s = CString::new(s).unwrap();
    s.into_raw()
}

#[no_mangle]
pub fn nu_rs(gname: &[u32], m: u32, h: u32) -> *mut c_char {
    let mod_v = gname.to_vec();
    let mut record = 0;
    for a in each_set_exact(m, &mod_v) {
        let interest_set = hfoldrestrictedsignedsumset(a, h, &mod_v);
        if interest_set.len() > record {
            record = interest_set.len();
        }
    }
    return exp_str(record.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nrs() {
        unsafe {
            println!("{:?}", CString::from_raw(nu_rs(&[30], 4, 2)));
        }
    }
}