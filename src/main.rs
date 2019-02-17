extern crate addcomb;

pub mod fastset;

pub mod comb;
use comb::chapter_a::*;
use comb::chapter_b::*;
use comb::chapter_c::*;

pub mod gburg_emulator;
use gburg_emulator::*;

extern crate rayon;
#[macro_use] extern crate itertools;
extern crate clap;

use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("Additive Combinatorics")
                    .version("0.1")
                    .author("Matt Torrence <torrma01@gettysburg.edu>")
                    .about("Compute combinatoric functions and sumset problems")
                    .subcommand(SubCommand::with_name("emulate")
                                .about("Compute sumsets similarly to www.addcomb.gettysburg.edu")
                                .version("1.0")
                                .author("Matt Torrence <torrma01@gettysburg.edu>")
                                .arg(Arg::with_name("order")
                                     .short("n")
                                     .long("order")
                                     .value_name("ORDER")
                                     .help("The order of the group Z_n to work with")
                                     .required(true)
                                     .takes_value(true))
                                .arg(Arg::with_name("subset_size")
                                     .short("m")
                                     .long("subset_size")
                                     .value_name("SIZE")
                                     .help("The size of the subsets of interest")
                                     .required(true)
                                     .takes_value(true))
                                .arg(Arg::with_name("h_folds")
                                     .short("h")
                                     .long("h_folds")
                                     .value_name("H_VALUES")
                                     .help("Either a single h value, or a range \"a,b\" or h values of interest")
                                     .required(true)
                                     .takes_value(true))
                                .arg(Arg::with_name("signed")
                                     .short("s")
                                     .long("signed")
                                     .help("Allow negative terms to be added"))
                                .arg(Arg::with_name("restricted")
                                     .short("r")
                                     .long("restricted")
                                     .help("Make all terms distinct"))
                                .arg(Arg::with_name("sizes")
                                     .short("z")
                                     .long("sizes")
                                     .help("Find sumsets with sizes in a range \"a,b\"")
                                     .required(true)
                                     .takes_value(true))
                                .arg(Arg::with_name("all")
                                    .short("v")
                                    .long("all")
                                    .help("Find all sumsets, as opposed to stopping when the first is found")))
                    .subcommand(SubCommand::with_name("compute")
                                .about("Compute the value of a combinatoric function found in the notation section of the book")
                                .version("0.1")
                                .author("Matt Torrence <torrma01@gettysburg.edu>")
                                .arg(Arg::with_name("function")
                                     .short("f")
                                     .long("function")
                                     .value_name("F_NAME")
                                     .help("The function to compute. Supported functions (with interval variants, with s replacing h, where applicable): nu(n, m, h); phi(n, h); sigma(n, h)")
                                     .required(true)
                                     .takes_value(true))
                                .arg(Arg::with_name("arguments")
                                    .short("a")
                                    .long("args")
                                    .value_name("VALUES")
                                    .help("Comma-seperated values of the function to compute (Example: 10,20)")
                                    .required(true)
                                    .takes_value(true))
                                .arg(Arg::with_name("signed")
                                     .short("s")
                                     .long("signed")
                                     .help("Allow positive and negative values in the sumset"))
                                .arg(Arg::with_name("restricted")
                                     .short("r")
                                     .long("restricted")
                                     .help("Restrict the coefficients to |lambda| = 1 in the sumset"))
                                .arg(Arg::with_name("interval")
                                     .short("i")
                                     .long("interval")
                                     .help("Use [0, s]A instead of hA in the sumset (allowed with other flags)")))
                    .get_matches();
    
    // Emulation handler
    if let Some(matches) = matches.subcommand_matches("emulate") {
        // Build the generator form
        let order = matches.value_of("order").unwrap().parse().unwrap_or_else(|invalid_num| panic!("Invalid numeric argument for order: {}", invalid_num));
        let m = matches.value_of("subset_size").unwrap().parse().unwrap_or_else(|invalid_num| panic!("Invalid numeric argument for subset_size: {}", invalid_num));

        let signed: bool = matches.is_present("signed");
        let repeat: bool = !matches.is_present("restricted");
        let terminate: bool = !matches.is_present("all");

        let size_range_s: Vec<u32> = matches.value_of("sizes").unwrap()
            .split(",")
            .map(|num| num.parse().unwrap_or_else(|invalid_num| panic!("Invalid numeric argument for size bound: {}", invalid_num)))
            .collect();
        if size_range_s.len() != 2 {
            panic!("Expected sizes to be of the form a,b: {}", matches.value_of("sizes").unwrap());
        }
        let size_range: (u32, u32) = (size_range_s[0], size_range_s[1]);

        let h_folds = matches.value_of("h_folds").unwrap();
        let h: TermSize = if let Ok(x) = h_folds.parse() {
            TermSize::Fixed(x)
        } else {
            let h_folds_s: Vec<u32> = h_folds
            .split(",")
            .map(|num| num.parse().unwrap_or_else(|invalid_num| panic!("Invalid numeric argument for h_folds: {}", invalid_num)))
            .collect();
            if h_folds_s.len() != 2 {
                panic!("Expected h_folds to be of the form a,b: {}", h_folds);
            }
            TermSize::Vary(h_folds_s[0], h_folds_s[1])
        };

        let gen_form = GeneratorForm {
            order: order,
            m: m,
            h: h,
            signed: signed,
            repeat: repeat,
            size_filter: size_range,
            terminate: terminate,
        };

        // Emulate the requested form
        emulate(gen_form);
    }

    if let Some(matches) = matches.subcommand_matches("compute") {
        let fchoice = matches.value_of("function").unwrap().trim().to_lowercase();
        let argchoice = matches.value_of("arguments").unwrap();
        
        let signed: bool = matches.is_present("signed");
        let restricted: bool = matches.is_present("restricted");
        let interval: bool = matches.is_present("interval");
        
        // Parse fchoice
        // TODO: Maybe support the actual greek letters as well?
        // Ignores the last argument if it's not necessary
        let func: Box<Fn(u32, u32, u32) -> u32> = match fchoice.as_ref()
        {
            "nu" => match (interval, signed, restricted) {
                (false, false, false) => Box::new(|a, b, c| nu(a, b, c)),
                (false, true, false)  => Box::new(|a, b, c| nu_signed(a, b, c)),
                (false, false, true)  => Box::new(|a, b, c| nu_restricted(a, b, c)),
                (false, true, true)   => Box::new(|a, b, c| nu_signed_restricted(a, b, c)),
                // Interval functions
                (true, false, false)  => Box::new(|a, b, c| nu_interval(a, b, c)),
                (true, true, false)   => Box::new(|a, b, c| nu_signed_interval(a, b, c)),
                (true, false, true)   => Box::new(|a, b, c| nu_restricted_interval(a, b, c)),
                (true, true, true)    => Box::new(|a, b, c| nu_signed_restricted_interval(a, b, c)),
            },
            "phi" => match (interval, signed, restricted) {
                (false, false, false) => Box::new(|a, b, _c| phi(a, b)),
                (false, true, false)  => Box::new(|a, b, _c| phi_signed(a, b)),
                (false, false, true)  => Box::new(|a, b, _c| phi_restricted(a, b)),
                (false, true, true)   => Box::new(|a, b, _c| phi_signed_restricted(a, b)),
                // Interval functions
                (true, false, false)  => Box::new(|a, b, _c| phi_interval(a, b)),
                (true, true, false)   => Box::new(|a, b, _c| phi_signed_interval(a, b)),
                (true, false, true)   => Box::new(|a, b, _c| phi_restricted_interval(a, b)),
                (true, true, true)    => Box::new(|a, b, _c| phi_signed_restricted_interval(a, b)),
            },
            "sigma" => match (interval, signed, restricted) {
                (false, false, false) => Box::new(|a, b, _c| sigma(a, b)),
                (false, true, false)  => Box::new(|a, b, _c| sigma_signed(a, b)),
                (false, false, true)  => Box::new(|a, b, _c| sigma_restricted(a, b)),
                (false, true, true)   => Box::new(|a, b, _c| sigma_signed_restricted(a, b)),
                // Interval functions
                (true, false, false)  => Box::new(|a, b, _c| sigma_interval(a, b)),
                (true, true, false)   => Box::new(|a, b, _c| sigma_signed_interval(a, b)),
                (true, false, true)   => Box::new(|a, b, _c| sigma_restricted_interval(a, b)),
                (true, true, true)    => Box::new(|a, b, _c| sigma_signed_restricted_interval(a, b)),
            },
            x => panic!("Unsupported or unrecognized function: {}", x)
        };

        let arguments: Vec<u32> = argchoice
            .split(",")
            .map(|num| num.parse().unwrap_or_else(|invalid_num| panic!("Invalid numeric argument to function: {}", invalid_num)))
            .collect();
        
        if arguments.len() > 3 {
            panic!("More than 3 arguments given to the function (note, they should be comma-seperated with no spaces): {}", argchoice);
        }

        // Special function cases:
        if arguments.len() != 3 && fchoice == "nu" {
            panic!("Nu takes 3 arguments, but {} arguments were given: {}", arguments.len(), argchoice);
        }
        if arguments.len() != 2 && fchoice == "phi" {
            panic!("Phi takes 2 arguments, but {} arguments were given: {}", arguments.len(), argchoice);
        }
        if arguments.len() != 2 && fchoice == "sigma" {
            panic!("Sigma takes 2 arguments, but {} arguments were given: {}", arguments.len(), argchoice);
        }

        let third_arg = if arguments.len() == 2 { 0 } else { arguments[2] };

        let computation: u32 = func(arguments[0], arguments[1], third_arg);

        println!("{}", computation);
    }
}


#[cfg(test)]
mod tests {
    use fastset::*;
    
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