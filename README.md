# datetag

[![Crate](https://img.shields.io/crates/v/datetag)](https://crates.io/crates/datetag)
![Crates.io](https://img.shields.io/crates/l/datetag)

``` bash
datetag 0.1.2
display a customizable date tag (e.g. TEST_202110)

EXAMPLES:

    $ datetag --add 22 --date 20220312 --prefix 'TEST_' --tag-type daily
    TEST_20220403

    $ datetag -a 22 -d 20220312 -p 'TEST_' -t d
    TEST_20220403

USAGE:
    datetag.exe [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --add <add>              date tag positive offset
    -d, --date <date>            date tag value
    -p, --prefix <prefix>        tag prefix
    -s, --sub <sub>              date tag negative offset
    -t, --tag-type <tag-type>    tag type [d | m | y | daily | monthly | yearly] [default: m]
```
