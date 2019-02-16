use fastset::*;
use comb::*;

pub fn sigma(n: u8, h: u32) -> u32 {
    for m in 2.. {
        let expected = choose(m + h - 1, h);
        let mut found = false;
        for a in each_set_exact_zero(n as u32, m as u32) {
            if a.hfoldsumset(h as u8, n).size() == expected as u8 {
                found = true;
                break;
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

    // Verify examples according to table on page 153 (details page 154)
    #[test]
    pub fn test_sigma() {
        for n in 30..38 {
            let expected = (((4.0 * (n as f64) - 3.0).sqrt() + 1.0) / 2.0).floor() as u32;

            if sigma(n, 2) != expected {
                println!("s({}) = {} != {}", n, sigma(n, 2), expected);
            }
        }
    }
}