# csloc
count substantial lines of code

    csloc 1.0
    lemonxah <lemon.xah@gmail.com>
    Count Substantial Lines of Code

    USAGE:
        cloc [FLAGS] [OPTIONS] <path> --filetype <filetype>

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information
        -v               Sets the level of verbosity. Use -v for a level 1 or -v -v / -vv for level 2.

    OPTIONS:
        -c, --charcount <count>      Counts lines with at least X non whitespace characters.
        -t, --filetype <filetype>    Sets filetype of source files. rs (rust), cpp (c++)
        -i, --ignore <ignore>...     Sets ignored folder pattern. target (rust). Can set multiple -i

    ARGS:
        <path>    Sets path for line counting

