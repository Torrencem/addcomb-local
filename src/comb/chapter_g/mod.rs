use std::cmp;
use fastset::*;

pub fn mu(n: u32, k: u32, l: u32) -> u32 {
    for m in 1..n {
        let mut found = false;
        for a in each_set_exact_no_zero(n, m) {
            let mut k_a = a.hfoldsumset(k, n);
            let l_a = a.hfoldsumset(l, n);
            k_a.intersect(&l_a);
            if k_a.isempty() {
                info!("[mu] For n={}, found {}, which is sum-free", n, a);
                info!("[mu] (k_a = {}, l_a = {})", k_a, l_a);
                found = true;
            }
        }
        if !found {
            return m - 1;
        }
    }
    panic!();
}

pub fn mu_signed(n: u32, k: u32, l: u32) -> u32 {
    for m in 1..n {
        let mut found = false;
        for a in each_set_exact_no_zero(n, m) {
            let mut k_a = a.hfoldsignedsumset(k, n);
            let l_a = a.hfoldsignedsumset(l, n);
            k_a.intersect(&l_a);
            if k_a.isempty() {
                info!("[mu] For n={}, found {}, which is sum-free", n, a);
                info!("[mu] (k_a = {}, l_a = {})", k_a, l_a);
                found = true;
            }
        }
        if !found {
            return m - 1;
        }
    }
    panic!();
}

pub fn mu_restricted(n: u32, k: u32, l: u32) -> u32 {
    let mut lower_bound = 1;
    if l == 1 && (n == k*(k*k - 1)) {
        lower_bound = cmp::max(n/(k + 1) + k - 1, k*k);
    }
    for m in lower_bound..n {
        let mut found = false;
        for a in each_set_exact_no_zero(n, m) {
            let mut k_a = a.hfoldrestrictedsumset(k, n);
            let l_a = a.hfoldrestrictedsumset(l, n);
            k_a.intersect(&l_a);
            if k_a.isempty() {
                info!("[mu] For n={}, found {}, which is sum-free", n, a);
                info!("[mu] (k_a = {}, l_a = {})", k_a, l_a);
                found = true;
            }
        }
        if !found {
            return m - 1;
        }
    }
    panic!();
}

pub fn mu_signed_restricted(n: u32, k: u32, l: u32) -> u32 {
    for m in 1..n {
        let mut found = false;
        for a in each_set_exact_no_zero(n, m) {
            let mut k_a = a.hfoldrestrictedsignedsumset(k, n);
            let l_a = a.hfoldrestrictedsignedsumset(l, n);
            k_a.intersect(&l_a);
            if k_a.isempty() {
                info!("[mu] For n={}, found {}, which is sum-free", n, a);
                info!("[mu] (k_a = {}, l_a = {})", k_a, l_a);
                found = true;
            }
        }
        if !found {
            return m - 1;
        }
    }
    panic!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mu_res() {
        for n in 5..=30 {
            println!("{}: {}", n, mu_restricted(n, 3, 1));
        }
    }
}