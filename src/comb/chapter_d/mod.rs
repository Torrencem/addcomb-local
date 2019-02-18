use fastset::*;

pub fn ro(n: u32, m: u32, h: u32) -> u32 {
    let mut smallest_set = empty_set();
    let mut curr_smallest = 0;
    for a in each_set_exact(n, m) {
        let size = a.hfoldsumset(h, n).size();
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!("[ro] Set with smallest sumset: {}", smallest_set);
    info!("[ro] (sumsets to:) {}", smallest_set.hfoldsumset(h, n));
    curr_smallest
}

pub fn ro_interval(n: u32, m: u32, s: u32) -> u32 {
    let mut smallest_set = empty_set();
    let mut curr_smallest = 0;
    for a in each_set_exact(n, m) {
        let size = a.hfoldintervalsumset((0, s), n).size();
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!("[ro] Set with smallest sumset: {}", smallest_set);
    info!("[ro] (sumsets to:) {}", smallest_set.hfoldintervalsumset((0, s), n));
    curr_smallest
}

pub fn ro_signed(n: u32, m: u32, h: u32) -> u32 {
    let mut smallest_set = empty_set();
    let mut curr_smallest = 0;
    for a in each_set_exact(n, m) {
        let size = a.hfoldsignedsumset(h, n).size();
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!("[ro] Set with smallest sumset: {}", smallest_set);
    info!("[ro] (sumsets to:) {}", smallest_set.hfoldsignedsumset(h, n));
    curr_smallest
}


pub fn ro_signed_interval(n: u32, m: u32, s: u32) -> u32 {
    let mut smallest_set = empty_set();
    let mut curr_smallest = 0;
    for a in each_set_exact(n, m) {
        let size = a.hfoldintervalsignedsumset((0, s), n).size();
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!("[ro] Set with smallest sumset: {}", smallest_set);
    info!("[ro] (sumsets to:) {}", smallest_set.hfoldintervalsignedsumset((0, s), n));
    curr_smallest
}

pub fn ro_restricted(n: u32, m: u32, h: u32) -> u32 {
    let mut smallest_set = empty_set();
    let mut curr_smallest = 0;
    for a in each_set_exact(n, m) {
        let size = a.hfoldrestrictedsumset(h, n).size();
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!("[ro] Set with smallest sumset: {}", smallest_set);
    info!("[ro] (sumsets to:) {}", smallest_set.hfoldrestrictedsumset(h, n));
    curr_smallest
}

pub fn ro_restricted_interval(n: u32, m: u32, s: u32) -> u32 {
    let mut smallest_set = empty_set();
    let mut curr_smallest = 0;
    for a in each_set_exact(n, m) {
        let size = a.hfoldintervalrestrictedsumset((0, s), n).size();
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!("[ro] Set with smallest sumset: {}", smallest_set);
    info!("[ro] (sumsets to:) {}", smallest_set.hfoldintervalrestrictedsumset((0, s), n));
    curr_smallest
}

pub fn ro_signed_restricted(n: u32, m: u32, h: u32) -> u32 {
    let mut smallest_set = empty_set();
    let mut curr_smallest = 0;
    for a in each_set_exact(n, m) {
        let size = a.hfoldrestrictedsignedsumset(h, n).size();
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!("[ro] Set with smallest sumset: {}", smallest_set);
    info!("[ro] (sumsets to:) {}", smallest_set.hfoldrestrictedsignedsumset(h, n));
    curr_smallest
}

pub fn ro_signed_restricted_interval(n: u32, m: u32, s: u32) -> u32 {
    let mut smallest_set = empty_set();
    let mut curr_smallest = 0;
    for a in each_set_exact(n, m) {
        let size = a.hfoldintervalrestrictedsignedsumset((0, s), n).size();
        if size < curr_smallest {
            curr_smallest = size;
            smallest_set = a;
        }
    }
    info!("[ro] Set with smallest sumset: {}", smallest_set);
    info!("[ro] (sumsets to:) {}", smallest_set.hfoldintervalrestrictedsignedsumset((0, s), n));
    curr_smallest
}