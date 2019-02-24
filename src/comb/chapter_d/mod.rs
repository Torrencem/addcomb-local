use fastset::*;

pub fn rho(n: u32, m: u32, h: u32) -> u32 {
    let mut smallest_set = empty_set();
    let mut curr_smallest = n;
    for a in each_set_exact(n, m) {
        let size = a.hfoldsumset(h, n).size();
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!("[rho] Set with smallest sumset: {}", smallest_set);
    info!("[rho] (sumsets to:) {}", smallest_set.hfoldsumset(h, n));
    curr_smallest
}

pub fn rho_interval(n: u32, m: u32, intv: (u32, u32)) -> u32 {
    let mut smallest_set = empty_set();
    let mut curr_smallest = n;
    for a in each_set_exact(n, m) {
        let size = a.hfoldintervalsumset(intv, n).size();
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!("[rho] Set with smallest sumset: {}", smallest_set);
    info!("[rho] (sumsets to:) {}", smallest_set.hfoldintervalsumset(intv, n));
    curr_smallest
}

pub fn rho_signed(n: u32, m: u32, h: u32) -> u32 {
    let mut smallest_set = empty_set();
    let mut curr_smallest = n;
    for a in each_set_exact(n, m) {
        let size = a.hfoldsignedsumset(h, n).size();
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!("[rho] Set with smallest sumset: {}", smallest_set);
    info!("[rho] (sumsets to:) {}", smallest_set.hfoldsignedsumset(h, n));
    curr_smallest
}


pub fn rho_signed_interval(n: u32, m: u32, intv: (u32, u32)) -> u32 {
    let mut smallest_set = empty_set();
    let mut curr_smallest = n;
    for a in each_set_exact(n, m) {
        let size = a.hfoldintervalsignedsumset(intv, n).size();
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!("[rho] Set with smallest sumset: {}", smallest_set);
    info!("[rho] (sumsets to:) {}", smallest_set.hfoldintervalsignedsumset(intv, n));
    curr_smallest
}

pub fn rho_restricted(n: u32, m: u32, h: u32) -> u32 {
    let mut smallest_set = empty_set();
    let mut curr_smallest = n;
    for a in each_set_exact(n, m) {
        let size = a.hfoldrestrictedsumset(h, n).size();
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!("[rho] Set with smallest sumset: {}", smallest_set);
    info!("[rho] (sumsets to:) {}", smallest_set.hfoldrestrictedsumset(h, n));
    curr_smallest
}

pub fn rho_restricted_interval(n: u32, m: u32, intv: (u32, u32)) -> u32 {
    let mut smallest_set = empty_set();
    let mut curr_smallest = n;
    for a in each_set_exact(n, m) {
        let size = a.hfoldintervalrestrictedsumset(intv, n).size();
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!("[rho] Set with smallest sumset: {}", smallest_set);
    info!("[rho] (sumsets to:) {}", smallest_set.hfoldintervalrestrictedsumset(intv, n));
    curr_smallest
}

pub fn rho_signed_restricted(n: u32, m: u32, h: u32) -> u32 {
    let mut smallest_set = empty_set();
    let mut curr_smallest = n;
    for a in each_set_exact(n, m) {
        let size = a.hfoldrestrictedsignedsumset(h, n).size();
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!("[rho] Set with smallest sumset: {}", smallest_set);
    info!("[rho] (sumsets to:) {}", smallest_set.hfoldrestrictedsignedsumset(h, n));
    curr_smallest
}

pub fn rho_signed_restricted_interval(n: u32, m: u32, intv: (u32, u32)) -> u32 {
    let mut smallest_set = empty_set();
    let mut curr_smallest = n;
    for a in each_set_exact(n, m) {
        let size = a.hfoldintervalrestrictedsignedsumset(intv, n).size();
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!("[rho] Set with smallest sumset: {}", smallest_set);
    info!("[rho] (sumsets to:) {}", smallest_set.hfoldintervalrestrictedsignedsumset(intv, n));
    curr_smallest
}