extern crate addcomb_rust;
use addcomb_rust::nu_exceptions;

fn main() {
    for n in 2..=30 {
        println!("n: {}, exceptions: {}", n, nu_exceptions(n));
    }
}