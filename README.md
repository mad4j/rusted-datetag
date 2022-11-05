# datetag

[![Crate](https://img.shields.io/crates/v/datetag)](https://crates.io/crates/datetag)
![Crates.io](https://img.shields.io/crates/l/datetag)

``` bash
datetag 0.1.3
display a customizable date tag (e.g. TEST_202110)

EXAMPLES:

    $ datetag --offset 22 --date 20220312 --prefix 'TEST_' --tag-type daily
    TEST_20220403

    $ datetag -o 22 -d 20220312 -p 'TEST_' -t d
    TEST_20220403

USAGE:
    datetag.exe [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --date <date>            date tag value (one of 'yyyymmdd', 'yyyymm', 'yyyy')
    -o, --offset <offset>        date tag offset [default: 0]
    -p, --prefix <prefix>        tag prefix
    -t, --tag-type <tag-type>    tag type [d | m | y | daily | monthly | yearly] [default: m]
```
