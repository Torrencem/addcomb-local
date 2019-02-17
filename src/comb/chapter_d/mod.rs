use fastset::*;

pub fn ro(n: u32, m: u32, h: u32) -> u32 {
    let mut curr_smallest = n;
    for a in each_set_exact(n, m) {
        let size = a.hfoldsumset(h, n).size();
        if size < curr_smallest {
            curr_smallest = size;
        }
    }
    curr_smallest
}

pub fn ro_interval(n: u32, m: u32, s: u32) -> u32 {
    let mut curr_smallest = n;
    for a in each_set_exact(n, m) {
        let size = a.hfoldintervalsumset((0, s), n).size();
        if size < curr_smallest {
            curr_smallest = size;
        }
    }
    curr_smallest
}

pub fn ro_signed(n: u32, m: u32, h: u32) -> u32 {
    let mut curr_smallest = n;
    for a in each_set_exact(n, m) {
        let size = a.hfoldsignedsumset(h, n).size();
        if size < curr_smallest {
            curr_smallest = size;
        }
    }
    curr_smallest
}

pub fn ro_signed_interval(n: u32, m: u32, s: u32) -> u32 {
    let mut curr_smallest = n;
    for a in each_set_exact(n, m) {
        let size = a.hfoldintervalsignedsumset((0, s), n).size();
        if size < curr_smallest {
            curr_smallest = size;
        }
    }
    curr_smallest
}

pub fn ro_restricted(n: u32, m: u32, h: u32) -> u32 {
    let mut curr_smallest = n;
    for a in each_set_exact(n, m) {
        let size = a.hfoldrestrictedsumset(h, n).size();
        if size < curr_smallest {
            curr_smallest = size;
        }
    }
    curr_smallest
}

pub fn ro_restricted_interval(n: u32, m: u32, s: u32) -> u32 {
    let mut curr_smallest = n;
    for a in each_set_exact(n, m) {
        let size = a.hfoldintervalrestrictedsumset((0, s), n).size();
        if size < curr_smallest {
            curr_smallest = size;
        }
    }
    curr_smallest
}

pub fn ro_signed_restricted(n: u32, m: u32, h: u32) -> u32 {
    let mut curr_smallest = n;
    for a in each_set_exact(n, m) {
        let size = a.hfoldrestrictedsignedsumset(h, n).size();
        if size < curr_smallest {
            curr_smallest = size;
        }
    }
    curr_smallest
}

pub fn ro_signed_restricted_interval(n: u32, m: u32, s: u32) -> u32 {
    let mut curr_smallest = n;
    for a in each_set_exact(n, m) {
        let size = a.hfoldintervalrestrictedsignedsumset((0, s), n).size();
        if size < curr_smallest {
            curr_smallest = size;
        }
    }
    curr_smallest
}