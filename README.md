# minigrep
Kind of minigrep in Rust. Just for learing some Rust.

## Usage

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
    -a, --from START_AT_LINE_NUMBER
                        start matching at line number
    -z, --until END_AT_LINE_NUMBER
                        match as long as line number is smaller
    -h, --help          print this help menu
```
## Regex

The Regex is bases on the library [regex](https://crates.io/crates/regex). This means the documented Rexex syntax can be uses. Example: given the file `test_regex.txt` has the content

```
2012-03-14, 2013-01-01 and 2014-07-05
```
the command
```bash
./minigrep -s '$m/$d/$y' \
-p "(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})" \
-e -f ~/test_regex.txt 
```
will print
```
03/14/2012, 01/01/2013 and 07/05/2014
```

