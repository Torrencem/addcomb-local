pub mod fastset;

pub mod comb;
use comb::chapter_a::*;
use comb::chapter_b::*;
use comb::chapter_c::*;
use comb::chapter_d::*;
use comb::chapter_e::*;
use comb::chapter_f::*;
use comb::chapter_g::*;

pub mod gburg_emulator;
use gburg_emulator::*;

extern crate rayon;
#[macro_use] extern crate itertools;
#[macro_use] extern crate lazy_static;
extern crate clap;
#[macro_use]
extern crate log;
use log::{Record, Level, Metadata, LevelFilter};
extern crate regex;

use regex::Regex;
use regex::Captures;

use clap::{Arg, App, SubCommand};

use std::fmt::Display;
use std::process::exit;

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

lazy_static! {
    static ref THREE_ARG_REG: Regex        = Regex::new(r"^(\d+),(\d+),(\d+)$").unwrap();
    static ref THREE_ARG_INTV_REG: Regex   = Regex::new(r"^(\d+),(\d+),\[(\d+),(\d+)\]$").unwrap();

    static ref TWO_ARG_REG: Regex          = Regex::new(r"^(\d+),(\d+)$").unwrap();
    static ref TWO_ARG_INTV_REG: Regex     = Regex::new(r"^(\d+),\[(\d+),(\d+)\]$").unwrap();

    static ref MU_REG: Regex               = Regex::new(r"^(\d+),(\d+),(\d+)$").unwrap();
}

static THREE_ARG_ERR: &str      = &" expects three arguments, like 30,20,10";
static THREE_ARG_INTV_ERR: &str = &" expects two regular arguments, and an interval, like 30,20,[0,4]";

static TWO_ARG_ERR: &str        = &" expects two arguments, like 30,20";
static TWO_ARG_INTV_ERR: &str   = &" expects one regular argument, and an interval, like 30,[0,4]";

static MU_ARG_ERR: &str         = &"mu expects three arguments formatted as n,k,l (e.g. 20,1,3)";

static I_TEXT: &str = &"Additive Combinatorics calculator v0.2";

fn from_capt(m: &Captures, i: usize) -> u32 {
    m.get(i)
    .map_or(0, |m| m.as_str().parse::<u32>().unwrap())
}

fn graceful_exit<D: Display>(message: D) -> ! {
    eprintln!("{}", message);
    exit(1)
}

fn main() {
    let matches = App::new("Additive Combinatorics")
                    .version("0.2")
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
                                     .help("The function to compute. Supported functions (with an interval replacing h, where applicable): nu(n, m, h); phi(n, h); sigma(n, h); rho(n, m, h); chi(n, h); tau(n, h); mu(n, k, l)")
                                     .required(true)
                                     .takes_value(true))
                                .arg(Arg::with_name("arguments")
                                    .short("a")
                                    .long("args")
                                    .value_name("VALUES")
                                    .help("Comma-seperated values of the function to compute (Example: 20,10,[0,2] or 20,10,3)")
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
                                     .help("Use [a, b]A instead of hA in the sumset (allowed with other flags)")))
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

        info!("{}", I_TEXT);

        let fchoice = matches.value_of("function").unwrap().trim().to_lowercase();
        let argchoice = matches.value_of("arguments").unwrap();
        
        let signed: bool = matches.is_present("signed");
        let restricted: bool = matches.is_present("restricted");
        let interval: bool = matches.is_present("interval");
        
        // Parse fchoice
        // Ignores the last argument if it's not necessary
        let func: Box<Fn(u32, u32, u32, u32) -> u32> = match fchoice.as_ref()
        {
            "nu" => match (interval, signed, restricted) {
                (false, false, false) => Box::new(|a, b, c, _d| nu(a, b, c)),
                (false, true, false)  => Box::new(|a, b, c, _d| nu_signed(a, b, c)),
                (false, false, true)  => Box::new(|a, b, c, _d| nu_restricted(a, b, c)),
                (false, true, true)   => Box::new(|a, b, c, _d| nu_signed_restricted(a, b, c)),
                // Interval functions
                (true, false, false)  => Box::new(|a, b, c, d| nu_interval(a, b, (c, d))),
                (true, true, false)   => Box::new(|a, b, c, d| nu_signed_interval(a, b, (c, d))),
                (true, false, true)   => Box::new(|a, b, c, d| nu_restricted_interval(a, b, (c, d))),
                (true, true, true)    => Box::new(|a, b, c, d| nu_signed_restricted_interval(a, b, (c, d))),
            },
            "phi" => match (interval, signed, restricted) {
                (false, false, false) => Box::new(|a, b, _c, _d| phi(a, b)),
                (false, true, false)  => Box::new(|a, b, _c, _d| phi_signed(a, b)),
                (false, false, true)  => Box::new(|a, b, _c, _d| phi_restricted(a, b)),
                (false, true, true)   => Box::new(|a, b, _c, _d| phi_signed_restricted(a, b)),
                // Interval functions
                (true, false, false)  => Box::new(|a, b, c, _d| phi_interval(a, (b, c))),
                (true, true, false)   => Box::new(|a, b, c, _d| phi_signed_interval(a, (b, c))),
                (true, false, true)   => Box::new(|a, b, c, _d| phi_restricted_interval(a, (b, c))),
                (true, true, true)    => Box::new(|a, b, c, _d| phi_signed_restricted_interval(a, (b, c))),
            },
            "sigma" => match (interval, signed, restricted) {
                (false, false, false) => Box::new(|a, b, _c, _d| sigma(a, b)),
                (false, true, false)  => Box::new(|a, b, _c, _d| sigma_signed(a, b)),
                (false, false, true)  => Box::new(|a, b, _c, _d| sigma_restricted(a, b)),
                (false, true, true)   => Box::new(|a, b, _c, _d| sigma_signed_restricted(a, b)),
                // Interval functions
                (true, false, false)  => Box::new(|a, b, _c, _d| sigma_interval(a, b)),
                (true, true, false)   => Box::new(|a, b, _c, _d| sigma_signed_interval(a, b)),
                (true, false, true)   => Box::new(|a, b, _c, _d| sigma_restricted_interval(a, b)),
                (true, true, true)    => Box::new(|a, b, _c, _d| sigma_signed_restricted_interval(a, b)),
            },
            "rho" => match (interval, signed, restricted) {
                (false, false, false) => Box::new(|a, b, c, _d| rho(a, b, c)),
                (false, true, false)  => Box::new(|a, b, c, _d| rho_signed(a, b, c)),
                (false, false, true)  => Box::new(|a, b, c, _d| rho_restricted(a, b, c)),
                (false, true, true)   => Box::new(|a, b, c, _d| rho_signed_restricted(a, b, c)),
                // Interval functions
                (true, false, false)  => Box::new(|a, b, c, d| rho_interval(a, b, (c, d) )),
                (true, true, false)   => Box::new(|a, b, c, d| rho_signed_interval(a, b, (c, d))),
                (true, false, true)   => Box::new(|a, b, c, d| rho_restricted_interval(a, b, (c, d))),
                (true, true, true)    => Box::new(|a, b, c, d| rho_signed_restricted_interval(a, b, (c, d))),
            },
            "chi" => match (interval, signed, restricted) {
                (false, false, false) => Box::new(|a, b, _c, _d| chi(a, b)),
                (false, true, false)  => Box::new(|a, b, _c, _d| chi_signed(a, b)),
                (false, false, true)  => Box::new(|a, b, _c, _d| chi_restricted(a, b)),
                (false, true, true)   => Box::new(|a, b, _c, _d| chi_signed_restricted(a, b)),
                // Interval functions
                (true, false, false)  => Box::new(|a, b, c, _d| chi_interval(a, (b, c))),
                (true, true, false)   => Box::new(|a, b, c, _d| chi_signed_interval(a, (b, c))),
                (true, false, true)   => Box::new(|a, b, c, _d| chi_restricted_interval(a, (b, c))),
                (true, true, true)    => Box::new(|a, b, c, _d| chi_signed_restricted_interval(a, (b, c))),
            },
            "tau" => match (interval, signed, restricted) {
                (false, false, false) => Box::new(|a, b, _c, _d| tau(a, b)),
                (false, true, false)  => Box::new(|a, b, _c, _d| tau_signed(a, b)),
                (false, false, true)  => Box::new(|a, b, _c, _d| tau_restricted(a, b)),
                (false, true, true)   => Box::new(|a, b, _c, _d| tau_signed_restricted(a, b)),
                // Interval functions
                (true, false, false)  => Box::new(|a, b, c, _d| tau_interval(a, (b, c))),
                (true, true, false)   => Box::new(|a, b, c, _d| tau_signed_interval(a, (b, c))),
                (true, false, true)   => Box::new(|a, b, c, _d| tau_restricted_interval(a, (b, c))),
                (true, true, true)    => Box::new(|a, b, c, _d| tau_signed_restricted_interval(a, (b, c))),
            },
            "mu" => match(signed, restricted) {
                (false, false) => Box::new(|a, b, c, _d| mu(a, b, c)),
                (false, true)  => Box::new(|a, b, c, _d| mu_restricted(a, b, c)),
                (true, false)  => Box::new(|a, b, c, _d| mu_signed(a, b, c)),
                (true, true)   => Box::new(|a, b, c, _d| mu_signed_restricted(a, b, c))
            },
            x => panic!("Unsupported or unrecognized function: {}", x)
        };

        let arguments: (u32, u32, u32, u32) = match fchoice.as_ref() {
            "nu" => match interval {
                false => {
                    let ref args = THREE_ARG_REG.captures(argchoice)
                        .unwrap_or_else(|| graceful_exit(format!("{}{}", "nu", THREE_ARG_ERR)));
                    (from_capt(args, 1),
                     from_capt(args, 2),
                     from_capt(args, 3),
                     from_capt(args, 4))
                },
                true => {
                    let ref args = THREE_ARG_INTV_REG.captures(argchoice)
                        .unwrap_or_else(|| graceful_exit(format!("{}{}", "nu interval", THREE_ARG_INTV_ERR)));
                    (from_capt(args, 1),
                     from_capt(args, 2),
                     from_capt(args, 3),
                     from_capt(args, 4))
                }
            },
            "phi" => match interval {
                false => {
                    let ref args = TWO_ARG_REG.captures(argchoice)
                        .unwrap_or_else(|| graceful_exit(format!("{}{}", "phi", TWO_ARG_ERR)));
                    (from_capt(args, 1),
                     from_capt(args, 2),
                     from_capt(args, 3),
                     from_capt(args, 4))
                },
                true => {
                    let ref args = TWO_ARG_INTV_REG.captures(argchoice)
                        .unwrap_or_else(|| graceful_exit(format!("{}{}", "phi interval", TWO_ARG_ERR)));
                    (from_capt(args, 1),
                     from_capt(args, 2),
                     from_capt(args, 3),
                     from_capt(args, 4))
                }
            },
            "sigma" => {
                    let ref args = TWO_ARG_REG.captures(argchoice)
                        .unwrap_or_else(|| graceful_exit(format!("{}{}", "sigma (and sigma interval)", TWO_ARG_ERR)));
                    (from_capt(args, 1),
                     from_capt(args, 2),
                     from_capt(args, 3),
                     from_capt(args, 4))
            },
            "rho" => match interval {
                false => {
                    let ref args = THREE_ARG_REG.captures(argchoice)
                        .unwrap_or_else(|| graceful_exit(format!("{}{}", "rho", THREE_ARG_ERR)));
                    (from_capt(args, 1),
                     from_capt(args, 2),
                     from_capt(args, 3),
                     from_capt(args, 4))
                },
                true => {
                    let ref args = THREE_ARG_INTV_REG.captures(argchoice)
                        .unwrap_or_else(|| graceful_exit(format!("{}{}", "rho interval", THREE_ARG_INTV_ERR)));
                    (from_capt(args, 1),
                     from_capt(args, 2),
                     from_capt(args, 3),
                     from_capt(args, 4))
                }
            },
            "chi" => match interval {
                false => {
                    let ref args = THREE_ARG_REG.captures(argchoice)
                        .unwrap_or_else(|| graceful_exit(format!("{}{}", "chi", THREE_ARG_ERR)));
                    (from_capt(args, 1),
                     from_capt(args, 2),
                     from_capt(args, 3),
                     from_capt(args, 4))
                },
                true => {
                    let ref args = THREE_ARG_INTV_REG.captures(argchoice)
                        .unwrap_or_else(|| graceful_exit(format!("{}{}", "chi interval", THREE_ARG_INTV_ERR)));
                    (from_capt(args, 1),
                     from_capt(args, 2),
                     from_capt(args, 3),
                     from_capt(args, 4))
                }
            },
            "tau" => match interval {
                false => {
                    let ref args = TWO_ARG_REG.captures(argchoice)
                        .unwrap_or_else(|| graceful_exit(format!("{}{}", "tau", TWO_ARG_ERR)));
                    (from_capt(args, 1),
                     from_capt(args, 2),
                     from_capt(args, 3),
                     from_capt(args, 4))
                },
                true => {
                    let ref args = TWO_ARG_INTV_REG.captures(argchoice)
                        .unwrap_or_else(|| graceful_exit(format!("{}{}", "tau interval", TWO_ARG_INTV_ERR)));
                    (from_capt(args, 1),
                     from_capt(args, 2),
                     from_capt(args, 3),
                     from_capt(args, 4))
                }
            },
            "mu" => match interval {
                false => {
                    let ref args = MU_REG.captures(argchoice)
                        .unwrap_or_else(|| graceful_exit(MU_ARG_ERR));
                    (from_capt(args, 1),
                     from_capt(args, 2),
                     from_capt(args, 3),
                     from_capt(args, 4))
                },
                true => panic!("Mu with interval not yet supported")
            },
            _ => unreachable!()
        };

        info!("Computing {}{}{}({})...", fchoice, if signed {"+-"} else {""}, if restricted {"^"} else {""}, argchoice);

        let computation: u32 = func(arguments.0, arguments.1, arguments.2, arguments.3);

        info!("Final result:");
        println!("{}", computation);
    }
}