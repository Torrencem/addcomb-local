
use std::fmt;
use std::collections::HashSet;
use std::iter::IntoIterator;
use std::iter::Map;

extern crate itertools;
use itertools::Itertools;
use itertools::Combinations;

use std::iter;
use std::cmp::min;

pub struct CombWithReplacement {
    indices: Vec<u32>,
    n: u32,
    r: u32,
    first: bool
}

impl Iterator for CombWithReplacement {
    type Item = Vec<u32>;

    // Based very much on https://docs.python.org/3/library/itertools.html#itertools.combinations
    fn next(&mut self) -> Option<Vec<u32>> {
        if self.first {
            self.first = false;
            return Some(vec![0 ; self.r as usize]);
        }
        let mut found: bool = false;
        let mut found_index: usize = 0;

        for i in (0..self.r).rev() {
            if self.indices[i as usize] != self.n - 1 {
                found_index = i as usize;
                found = true;
                break;
            }
        }
        if !found {
            return None;
        }
        
        self.indices.splice(found_index.., iter::repeat(self.indices[found_index] + 1).take(self.r as usize - found_index));

        Some(self.indices.clone())
    }
}

pub fn combinations_with_replacement(n: u32, r: u32) -> CombWithReplacement {
    CombWithReplacement { indices: vec![0; r as usize], n: n, r: r, first: true }
}

pub struct EachElement<'a> {
    pub curr: Vec<u32>,
    pub mod_v: &'a Vec<u32>,
    pub first: bool
}

impl<'a> Iterator for EachElement<'a> {
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Vec<u32>> {
        if self.first {
            self.first = false;
            return Some(vec![0 ; self.mod_v.len()]);
        }
        let mut indx = 0;
        while self.curr[indx] == self.mod_v[indx] - 1 {
            self.curr[indx] = 0;
            indx += 1;
            if indx == self.curr.len() {
                return None;
            }
        }
        self.curr[indx] += 1;
        Some(self.curr.clone())
    }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct GElem(pub Vec<u32>);

pub fn each_set_exact(size: u32, mod_v: &Vec<u32>) -> EachSetExact {
    EachSetExact { c: (EachElement { curr: vec![0; mod_v.len()], mod_v: &mod_v, first: true })
        .combinations(size as usize) }
}

pub struct EachSetExact<'a> {
    pub c: Combinations<EachElement<'a>>
}

impl<'a> Iterator for EachSetExact<'a> {
    type Item = Vec<GElem>;

    fn next(&mut self) -> Option<Vec<GElem>> {
        let v: Vec<Vec<u32>> = self.c.next()?;
        Some(v.iter().map(|elem| GElem(elem.to_vec())).collect())
    }
}

pub fn each_set_exact_no_zero(size: u32, mod_v: &Vec<u32>) -> EachSetExact {
    EachSetExact { c: (EachElement { curr: vec![0; mod_v.len()], mod_v: &mod_v, first: false })
        .combinations(size as usize) }
}

#[inline]
pub fn mod_sum(x: &GElem, y: &GElem, mod_v: &Vec<u32>) -> GElem {
    let GElem(xc) = x;
    let GElem(yc) = y;
    debug_assert!(xc.len() == yc.len());
    let mut res: Vec<u32> = vec![0; xc.len()];
    for (((zref, xval), yval), mod_val) in res.iter_mut().zip(xc.into_iter()).zip(yc.into_iter()).zip(mod_v) {
        *zref = (xval + yval) % mod_val;
    }
    GElem(res)
}

#[inline]
pub fn elem_sub(x: &GElem, y: &GElem) -> GElem {
    let GElem(xc) = x;
    let GElem(yc) = y;
    debug_assert!(xc.len() == yc.len());
    let mut res: Vec<u32> = vec![0; xc.len()];
    for ((zref, xval), yval) in res.iter_mut().zip(xc.into_iter()).zip(yc.into_iter()) {
        *zref = xval - yval;
    }
    GElem(res)
}

impl fmt::Display for GElem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let GElem(x) = self;
        let asstrs: Vec<String> = x.into_iter().map(|s| s.to_string()).collect();
        let asstr: String = asstrs.join(", ");
        write!(f, "({})", asstr)
    }
}

pub fn hfoldsumset(set: Vec<GElem>, h: u32, mod_v: &Vec<u32>) -> HashSet<GElem> {
    let mut res: HashSet<GElem> = HashSet::new();
    let as_vec: Vec<GElem> = set;
    let n: usize = mod_v.len();
    if as_vec.len() == 0 {
        res.insert(GElem(vec![0; n]));
        return res;
    }

    for indices in combinations_with_replacement(as_vec.len() as u32, h) {
        res.insert(
            indices.into_iter().map(|index| as_vec[index as usize].clone())
                   .fold(GElem(vec![0; n]), |prev, curr| mod_sum(&prev, &curr, &mod_v))
        );
    }
    res
}

pub fn hfoldsignedsumset(set: Vec<GElem>, h: u32, mod_v: &Vec<u32>) -> HashSet<GElem> {
    let mut res: HashSet<GElem> = HashSet::new();
    let as_vec: Vec<GElem> = set;
    let n: usize = mod_v.len();
    if as_vec.len() == 0 {
        res.insert(GElem(vec![0; n]));
        return res;
    }

    for indices in combinations_with_replacement(as_vec.len() as u32, h) {
        let mut coeffs: Vec<u32> = vec![1; indices.len()];
        loop {
            println!("ids: {:?}\ncoeffs: {:?}", &indices, &coeffs);
            res.insert(
            indices.clone().into_iter().map(|index| as_vec[index as usize].clone())
                   .enumerate()
                   .fold((0, GElem(vec![0; n])), |prev, curr| {
                        let (index, elem) = curr;
                        let (_, prev_elem) = prev;
                        if coeffs[index] == 0 {
                            (0, mod_sum(&prev_elem, &elem_sub(&GElem((*mod_v).to_vec()), &elem), &mod_v))
                        } else {
                            (0, mod_sum(&prev_elem, &elem, &mod_v))
                        }
                   }).1
            );
            let mut found_index: usize = 0;
            let mut found = true;
            while coeffs[found_index] == 0 {
                if found_index == indices.len() - 1 {
                    found = false;
                    break;
                }
                coeffs[found_index] = 1;
                found_index += 1;
            }
            if !found {
                break;
            } else {
                // Fill consecutive indices which have the same value
                let val_at = indices[found_index];
                let mut indx = found_index;
                while indx < indices.len() && indices[indx] == val_at {
                    coeffs[indx] = 0;
                    indx += 1;
                }
            }
        }
    }
    res
}

pub fn hfoldrestrictedsumset(set: Vec<GElem>, h: u32, mod_v: &Vec<u32>) -> HashSet<GElem> {
    let mut res: HashSet<GElem> = HashSet::new();
    let as_vec: Vec<GElem> = set;
    let n: usize = mod_v.len();
    if as_vec.len() == 0 {
        res.insert(GElem(vec![0; n]));
        return res;
    }

    for indices in (0..as_vec.len() as u32).combinations(h as usize) {
        res.insert(
            indices.into_iter().map(|index| as_vec[index as usize].clone())
                   .fold(GElem(vec![0; n]), |prev, curr| mod_sum(&prev, &curr, &mod_v))
        );
    }
    res
}

pub fn hfoldrestrictedsignedsumset(set: Vec<GElem>, h: u32, mod_v: &Vec<u32>) -> HashSet<GElem> {
    let mut res: HashSet<GElem> = HashSet::new();
    let as_vec: Vec<GElem> = set;
    let n: usize = mod_v.len();
    if as_vec.len() == 0 {
        res.insert(GElem(vec![0; n]));
        return res;
    }

    for indices in (0..as_vec.len() as u32).combinations(h as usize) {
        let mut coeffs: Vec<u32> = vec![1; indices.len()];
        loop {
            res.insert(
            indices.clone().into_iter().map(|index| as_vec[index as usize].clone())
                   .enumerate()
                   .fold((0, GElem(vec![0; n])), |prev, curr| {
                        let (index, elem) = curr;
                        let (_, prev_elem) = prev;
                        if coeffs[index] == 0 {
                            (0, mod_sum(&prev_elem, &elem_sub(&GElem((*mod_v).to_vec()), &elem), &mod_v))
                        } else {
                            (0, mod_sum(&prev_elem, &elem, &mod_v))
                        }
                   }).1
            );
            let mut found_index: usize = 0;
            let mut found = true;
            while coeffs[found_index] == 0 {
                if found_index == indices.len() - 1 {
                    found = false;
                    break;
                }
                coeffs[found_index] = 1;
                found_index += 1;
            }
            if !found {
                break;
            } else {
                coeffs[found_index] = 0;
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ms() {
        let x = GElem(vec![1, 2, 3]);
        let y = GElem(vec![10, 2, 3]);
        println!("{}", x);

        let m = vec![5, 3, 3];

        println!("{}", mod_sum(&x, &y, &m));
    }

    #[test]
    fn test_combs() {
        for v in each_set_exact(3, &vec![5, 3]) {
            println!("{:?}", v);
        }
    }
}