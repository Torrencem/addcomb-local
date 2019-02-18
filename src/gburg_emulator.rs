use fastset::*;

#[derive(Debug)]
pub enum TermSize {
    Fixed(u32),
    Vary(u32, u32)
}

#[derive(Debug)]
pub struct GeneratorForm {
    pub order: u32,
    pub m: u32,
    pub h: TermSize,
    pub signed: bool,
    pub repeat: bool,
    pub size_filter: (u32, u32),
    pub terminate: bool
}

#[inline]
fn isin(x: u32, range: (u32, u32)) -> bool {
    let (a, b) = range;
    (x >= a) && (x <= b)
}

// Writes to stdout
pub fn emulate(params: GeneratorForm) {
    let mut count_found = 0;
    let n = params.order;

    // Before we do any emulating, figure out which function we're interested in
    let interest_func: Box<Fn(FastSet, (u32, (u32, u32))) -> FastSet> = match params.h {
            TermSize::Fixed(_) => match (params.signed, params.repeat) {
                (false, true) => Box::new(|a, sp| a.hfoldsumset(sp.0, n)),
                (true, true) => Box::new(|a, sp| a.hfoldsignedsumset(sp.0, n)),
                (false, false) => Box::new(|a, sp| a.hfoldrestrictedsumset(sp.0, n)),
                (true, false) => Box::new(|a, sp| a.hfoldrestrictedsignedsumset(sp.0, n))
            },
            TermSize::Vary(_, _) => match (params.signed, params.repeat) {
                (false, true) => Box::new(|a, sp| a.hfoldintervalsumset(sp.1, n)),
                (true, true) => Box::new(|a, sp| a.hfoldintervalsignedsumset(sp.1, n)),
                (false, false) => Box::new(|a, sp| a.hfoldintervalrestrictedsumset(sp.1, n)),
                (true, false) => Box::new(|a, sp| a.hfoldintervalrestrictedsignedsumset(sp.1, n))
            }
        };
    
    let static_params = match params.h {
        TermSize::Fixed(h) => (h, (0, 0)),
        TermSize::Vary(h1, h2) => (0, (h1, h2))
    };

    let mut found = false;

    for a in each_set_exact(params.order, params.m) {
        // Figure out what the sumset we're interested in is
        let ss = interest_func(a, static_params);
        if isin(ss.size(), params.size_filter) {
            println!("sumset({:?}) = {:?}", a, ss);
            count_found += 1;
            if params.terminate {
                found = true;
                break;
            }
        }
    }

    if !params.terminate {
        println!("\nTotal matches found: {}", count_found);
    } else if !found {
        println!("No sets were found which matched the request!");
    }

    println!("Done!");
}