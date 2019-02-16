extern crate addcomb;

pub mod fastset;
use fastset::*;

pub mod comb;
use comb::chapter_a::nu_exceptions;

extern crate rayon;


#[macro_use] extern crate itertools;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sets() {
        let mut s = FastSet::from(&[2, 4, 10][..]);
        s.cycle(5, 11);
        println!("{:?}", s);
    }

    #[test]
    fn test_sumsets() {
        let s1 = FastSet::from(&[1, 3][..]);
        let s2 = FastSet::from(&[2, 4, 5][..]);
        let s3 = s1.simplesumset(&s2, 10);
        println!("{:?}", s3);
    }

    #[test]
    fn test_hfolds() {
        let s1 = FastSet::from(&[2, 3][..]);
        for iter in 0..=12 {
            println!("{}A = {:?}", iter, s1.hfoldsumset(iter, 13));
        }
        println!("");

        assert!(!s1.hfoldsumset(11, 13).isfull(12));
        assert!(s1.hfoldsumset(12, 13).isfull(12));
        // TODO: Maybe more tests of off-by-one isfulls?
    }

    #[test]
    fn test_iterators() {
        for a in each_set_exact_zero(6, 3) {
            println!("{:?}", a);
        }
    }

    #[test]
    fn test_multipurpose() {
        // Page 133
        // for n in 2..=21 {
        //     println!("n: {}, exceptions: {}", n, nu_exceptions(n));
        // }
        for a in each_set_exact(50, 5) {
            if a.hfoldsumset(3, 50).size() == 20 {
                println!("{:?}", a);
            }
        }
    }

    #[test]
    fn test_2() {
        println!("{:?}", FastSet::from(&[1,3,8][..]).hfoldsumset(2, 20));
    }

    #[test]
    fn test_phi() {
        for h in 1..10 {
            for n in 2..=9 {
                let mut found = false;
                for a in each_set_exact(10, n) {
                    if a.hfoldsumset(h, 10).isfull(10) {
                        println!("A: {:?}, h: {}, n: {}", a, h, n);
                        found = true;
                        break;
                    }
                }
                if found {
                    break;
                }
            }
        }
    }
}


fn main() {
    for n in 2..=30 {
        println!("n: {}, exceptions: {}", n, nu_exceptions(n));
    }
}