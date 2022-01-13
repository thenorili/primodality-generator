# primodality_generator-1.1
Takes primes as input to generate primodal .scl files.

**Primodality** is a new category of digital harmony space conceived of by [Zheanna Erose](https://www.youtube.com/channel/UC--VosYH0BHISbb4SFO9rQA).

A unique primodal scale exists for every conceivable prime. *n*-primodality is the set of all rational numbers where the denominator is *n* and the numerator *a* is greater than or equal to *n*. Particularly of interest is the subset where n ≤ a < 8n, which always contains a just fifth. This is the range for which this program produces a .scl file.

This program is run via command line. It accepts primes greater than 2 that fit in an unsigned 32-bit integer. I've produced a portable linux binary which can be found in the builds directory. If you have any questions, reach out on [twitter](https://www.twitter.com/thenorili)!

## Documentation:

Generates just intonation .scl files for a given integer denominator.

USAGE:

    primodality_generator-linux-1.1 [FLAGS] [OPTIONS] [input]

FLAGS:

    -h, --help       Prints help information
    
    -p, --poly       Permits nonprime denominators.
    
    -V, --version    Prints version information

OPTIONS:

    -f, --from <from>    Sets the minimum numerator as a factor of n.
    
                         Valid values are 0-254. Default value is 1. [default: 1]
                         
    -t, --to <to>        Sets the maximum numerator as a factor of n.
    
                         Valid values are 1-255. Default value is 8. [default: 8]

ARGS:

    <input>    Sets denominator directly, skipping the menu.
