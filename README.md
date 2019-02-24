# Additive Combinatorics

A compilation of useful functions and tools based on this [additive combinatorics book](https://arxiv.org/pdf/1705.07444.pdf).

To use this project, download a binary for your system from the [releases page](https://github.com/torrencem/addcomb/releases). Use the executable from the command line. A helpful place to start is ``./addcomb.exe compute --help``:

    addcomb-compute 0.2
    Matt Torrence <torrma01@gettysburg.edu>
    Compute the value of a combinatoric function found in the notation section of the book

    USAGE:
        addcomb compute [FLAGS] --args <VALUES> --function <F_NAME>

    FLAGS:
        -h, --help          Prints help information
        -i, --interval      Use [a, b]A instead of hA in the sumset (allowed with other flags)
        -r, --restricted    Restrict the coefficients to |lambda| = 1 in the sumset
        -s, --signed        Allow positive and negative values in the sumset
        -V, --version       Prints version information
        -v, --verbose       Print out extra information other than the result, if available for the chosen function

    OPTIONS:
        -a, --args <VALUES>        Comma-seperated values of the function to compute (Example: 20,10,[0,2] or 20,10,3)
        -f, --function <F_NAME>    The function to compute. Supported functions (with an interval replacing h, where
                                applicable): nu(n, m, h); phi(n, h); sigma(n, h); rho(n, m, h); chi(n, h); tau(n, h);
                                mu(n, k, l)



If you want to build this yourself, it's very easy. [Install rust & cargo](https://www.rust-lang.org/tools/install) on your system. In the command line, traverse to the directory of this project, then use ``cargo build --release`` to build the project. Then, a binary should appear in ``./target/release/``, which you can run.
