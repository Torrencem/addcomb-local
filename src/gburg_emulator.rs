
use fastset::*;
use comb::*;

pub enum TermSize {
    Fixed(u32),
    Vary(u32, u32)
}

pub struct GeneratorForm {
    order: u32,
    m: u32,
    h: TermSize,
    signed: bool,
    repeat: bool,
    size_filter: (u32, u32),
    terminate: bool
}

#[inline]
fn isin(x: u32, range: (u32, u32)) {
    let (a, b) = range;
    (x >= a) && (x <= b)
}

// Writes to stdout
// Will split out to 4 versions based on
// Signed vs Unsigned, repeat vs no-repeat
pub fn emulate(params: GeneratorForm) {
    match (params.signed, params.repeat) {
        (false, false) => _emulate_restricted(params),
        (true, false) => _emulate_signed_restricted(params),
        (false, true) => _emulate(params),
        (true, true) => _emulate_signed(params)
    }
}

fn _emulate(params: GeneratorForm) {
    for a in each_set_exact(params.m) {
        let ss = match params.h {
            Fixed(m) => a.hfoldsumset(m);
            
        }
    }
}