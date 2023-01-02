# datetag

[![Crate](https://img.shields.io/crates/v/datetag)](https://crates.io/crates/datetag)
![Crates.io](https://img.shields.io/crates/l/datetag)

``` bash
display a customizable date tag (e.g. TEST_202110)

EXAMPLES:

    $ datetag --offset 22 --date 20220312 --prefix 'TEST_' --tag-type daily
    TEST_20220403

    $ datetag -o 22 -d 20220312 -p 'TEST_' -t d
    TEST_20220403

Usage: datetag.exe [OPTIONS]

Options:
  -t, --tag-type <TAG_TYPE>
          tag type [d | m | y | daily | monthly | yearly]

          [default: m]

          Possible values:
          - y
          - yearly:  yearly tags (e.g. 2022)
          - m
          - monthly: monthly tags (e.g. 202212)
          - d
          - daily:   daily tags (e.g. 20221230)

  -p, --prefix <PREFIX>
          tag prefix

  -d, --date <DATE>
          date tag value (one of 'yyyymmdd', 'yyyymm', 'yyyy')

  -o, --offset <OFFSET>
          date tag offset

          [default: 0]

  -h, --help
          Print help information (use `-h` for a summary)

  -V, --version
          Print version information
```
