pub mod fastset;

pub mod comb;
use comb::chapter_a::*;
use comb::chapter_b::*;
use comb::chapter_c::*;
use comb::chapter_d::*;
use comb::chapter_e::*;
use comb::chapter_f::*;

pub mod gburg_emulator;
use gburg_emulator::*;

extern crate rayon;
#[macro_use] extern crate itertools;
extern crate clap;
#[macro_use]
extern crate log;
use log::{Record, Level, Metadata, LevelFilter};

use clap::{Arg, App, SubCommand};

static CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;

struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{}", record.args());
        }
    }

    fn flush(&self) {}
}

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
                                .arg(Arg::with_name("verbose")
                                     .short("v")
                                     .long("verbose")
                                     .help("Print out extra information other than the result, if available for the chosen function"))
                                .arg(Arg::with_name("function")
                                     .short("f")
                                     .long("function")
                                     .value_name("F_NAME")
                                     .help("The function to compute. Supported functions (with interval variants, with s replacing h, where applicable): nu(n, m, h); phi(n, h); sigma(n, h); ro(n, m, h); chi(n, h); tau(n, h)")
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
        if matches.is_present("verbose") {
            log::set_logger(&CONSOLE_LOGGER).unwrap_or_else(|_| println!("Warning: unable to setup logger!\nVerbose output will be disabled"));
            log::set_max_level(LevelFilter::Info);
        }

        let fchoice = matches.value_of("function").unwrap().trim().to_lowercase();
        let argchoice = matches.value_of("arguments").unwrap();
        
        let signed: bool = matches.is_present("signed");
        let restricted: bool = matches.is_present("restricted");
        let interval: bool = matches.is_present("interval");
        
        // Parse fchoice
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
            "ro" => match (interval, signed, restricted) {
                (false, false, false) => Box::new(|a, b, c| ro(a, b, c)),
                (false, true, false)  => Box::new(|a, b, c| ro_signed(a, b, c)),
                (false, false, true)  => Box::new(|a, b, c| ro_restricted(a, b, c)),
                (false, true, true)   => Box::new(|a, b, c| ro_signed_restricted(a, b, c)),
                // Interval functions
                (true, false, false)  => Box::new(|a, b, c| ro_interval(a, b, c)),
                (true, true, false)   => Box::new(|a, b, c| ro_signed_interval(a, b, c)),
                (true, false, true)   => Box::new(|a, b, c| ro_restricted_interval(a, b, c)),
                (true, true, true)    => Box::new(|a, b, c| ro_signed_restricted_interval(a, b, c)),
            },
            "chi" => match (interval, signed, restricted) {
                (false, false, false) => Box::new(|a, b, _c| chi(a, b)),
                (false, true, false)  => Box::new(|a, b, _c| chi_signed(a, b)),
                (false, false, true)  => Box::new(|a, b, _c| chi_restricted(a, b)),
                (false, true, true)   => Box::new(|a, b, _c| chi_signed_restricted(a, b)),
                // Interval functions
                (true, false, false)  => Box::new(|a, b, _c| chi_interval(a, b)),
                (true, true, false)   => Box::new(|a, b, _c| chi_signed_interval(a, b)),
                (true, false, true)   => Box::new(|a, b, _c| chi_restricted_interval(a, b)),
                (true, true, true)    => Box::new(|a, b, _c| chi_signed_restricted_interval(a, b)),
            },
            "tau" => match (interval, signed, restricted) {
                (false, false, false) => Box::new(|a, b, _c| tau(a, b)),
                (false, true, false)  => Box::new(|a, b, _c| tau_signed(a, b)),
                (false, false, true)  => Box::new(|a, b, _c| tau_restricted(a, b)),
                (false, true, true)   => Box::new(|a, b, _c| tau_signed_restricted(a, b)),
                // Interval functions
                (true, false, false)  => Box::new(|a, b, _c| tau_interval(a, b)),
                (true, true, false)   => Box::new(|a, b, _c| tau_signed_interval(a, b)),
                (true, false, true)   => Box::new(|a, b, _c| tau_restricted_interval(a, b)),
                (true, true, true)    => Box::new(|a, b, _c| tau_signed_restricted_interval(a, b)),
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
        if arguments.len() != 3 && fchoice == "ro" {
            panic!("Ro takes 3 arguments, but {} arguments were given: {}", arguments.len(), argchoice);
        }
        if arguments.len() != 2 && fchoice == "phi" {
            panic!("Phi takes 2 arguments, but {} arguments were given: {}", arguments.len(), argchoice);
        }
        if arguments.len() != 2 && fchoice == "chi" {
            panic!("Chi takes 2 arguments, but {} arguments were given: {}", arguments.len(), argchoice);
        }
        if arguments.len() != 2 && fchoice == "sigma" {
            panic!("Sigma takes 2 arguments, but {} arguments were given: {}", arguments.len(), argchoice);
        }
        if arguments.len() != 2 && fchoice == "tau" {
            panic!("Tau takes 2 arguments, but {} arguments were given: {}", arguments.len(), argchoice);
        }

        if fchoice == "nu" || fchoice == "ro" {
            if arguments[1] > arguments[0] {
                panic!("The value of m given cannot be larger than the value of n!");
            }
        }

        let third_arg = if arguments.len() == 2 { 0 } else { arguments[2] };

        let intv_text = format!("[{},{}]", if fchoice == "tau" {1} else {0}, arguments[arguments.len() - 1]);
        let rest_of_args_text: String = argchoice[..argchoice.rfind(",").unwrap()].to_string();
        info!("Computing {}{}{}({},{})...", fchoice, if signed {"+-"} else {""}, if restricted {"^"} else {""}, rest_of_args_text, if interval {intv_text} else {arguments[0].to_string()});

        let computation: u32 = func(arguments[0], arguments[1], third_arg);

        info!("Final result:");
        println!("{}", computation);
    }
}