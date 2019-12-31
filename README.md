# minigrep
Kind of minigrep in rust

Usage: ./minigrep -f FILE [options]

Options:
    -p, --pattern PATTERN
                        set pattern to finde
    -s, --substitute SUBSTITUTE
                        subsitute pattern with this
    -f, --file FILE     file to search pattern in
    -i, --insensitive   case insensitive matching - not valued in case of a
                        regex pattern
    -e, --regex         interpret pattern as regular expression
    -h, --help          print this help menu

