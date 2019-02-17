# Additive Combinatorics

A compilation of useful functions and tools based on this [additive combinatorics book](https://arxiv.org/pdf/1705.07444.pdf).

To use: [install rust & cargo](https://www.rust-lang.org/tools/install) on your system. In the command line, traverse to the directory of this project, then use ``cargo build --release`` to build the project. Then, a binary should appear in ``./target/release/``, which you can run. A helpful place to start is ``./addcomb compute --help``:

        USAGE:
            addcomb compute [FLAGS] --args <VALUES> --function <F_NAME>

        FLAGS:
            -h, --help          Prints help information
            -i, --interval      Use [0, s]A instead of hA in the sumset (allowed with other flags)
            -r, --restricted    Restrict the coefficients to |lambda| = 1 in the sumset
            -s, --signed        Allow positive and negative values in the sumset
            -V, --version       Prints version information

        OPTIONS:
            -a, --args <VALUES>        Comma-seperated values of the function to compute (Example: 10,20)
            -f, --function <F_NAME>    The function to compute. Supported functions (with interval variants, with s replacing h,
                                       where applicable): nu(n, m, h); phi(n, h); sigma(n, h); ro(n, m, h); chi(n, h); tau(n, h)
