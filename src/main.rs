pub mod fastset;

pub mod comb;
use comb::chapter_a::*;
use comb::chapter_b::*;
use comb::chapter_c::*;
use comb::chapter_d::*;
use comb::chapter_e::*;
use comb::chapter_f::*;
use comb::chapter_g::*;

use comb::exact_copy_gen;
// #[macro_use] extern crate itertools;
#[macro_use] extern crate lazy_static;

// For exactset
extern crate itertools;
pub mod exactset;

mod wasm_result;

fn main() {
    // std::env::set_var("RUST_BACKTRACE", "1");
    // println!("Initialized rust code");
}