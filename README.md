![test_minigrep](https://github.com/sdoerig/minigrep/workflows/test_minigrep/badge.svg?branch=master&event=push)

# minigrep
Kind of minigrep in Rust. Just for learing some Rust.

```
Usage: ./minigrep -f FILE [options]

Options:
    -p, --pattern PATTERN
                        set pattern to find
    -s, --substitute SUBSTITUTE
                        subsitute pattern with this
    -f, --file FILE     file to search pattern in
    -i, --insensitive   case insensitive matching - not valued in case of a
                        regex pattern
    -e, --regex         interpret pattern as regular expression
    -n, --number        show line numbers of matches
    -r, --recursiv      search FILE recursiv
    -a, --from          start matching at line number
    -z, --until         match as long as line number is smaller
    -h, --help          print this help menu
```
