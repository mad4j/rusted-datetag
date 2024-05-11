# datetag

[![Crate](https://img.shields.io/crates/v/datetag)](https://crates.io/crates/datetag)
![Crates.io](https://img.shields.io/crates/l/datetag)

![icon](icon.png)

```text
Display a customizable date tag (e.g. TEST_202404, 2024-04-03_rel, 2024.04.03)

Usage: datetag.exe [OPTIONS] [DATE]

Arguments:
  [DATE]
          Reference date, using today is not specified (e.g. 'yyyymmdd', 'yyyymm', 'yyyy', allowed field separators: '.-/:')

Options:
  -t, --tag-type <TAG_TYPE>
          Tag type [d | m | y | daily | monthly | yearly]

          [default: m]

          Possible values:
          - y
          - yearly:  yearly tags (e.g. 2022)
          - w
          - weekly:  weekly ISO 8601 tags (e.g. 202234)
          - m
          - monthly: monthly tags (e.g. 202212)
          - d
          - daily:   daily tags (e.g. 20221230)

  -s, --style <STYLE>
          Date tag style

          [default: plain]

          Possible values:
          - plain: yyyymmdd
          - dot:   yyyy.mm.dd
          - slash: yyyy/mm/dd
          - colon: yyyy:mm:dd
          - dash:  yyyy-mm-dd

  -p, --prefix <PREFIX>
          Tag prefix (e.g. 'LAB_202404')

  -x, --suffix <SUFFIX>
          Tag suffix (e.g. '202404_rel')

  -o, --offset <OFFSET>
          Date offset (offset unit depends on -t value)

          [default: 0]

  -f, --file <FILE>
          Use provided file modification date as reference

  -r, --repeat <REPEAT>
          Generate more date tags

  -n, --new-line
          Append an end-of-line to each generated tag

      --format <FORMAT>
          Custom date reference format string, override --style value

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

Examples:
    $ datetag 20240312 --offset 22 --prefix 'TEST_' --tag-type daily
    TEST_20240403

    $ datetag 20240312 -o 22 -p 'TEST_' -td
    TEST_20240403

    $ datetag 20240312 -o 2 -r3 -td -s dot
    2024.03.12
    2024.03.14
    2024.03.16

Notes:
    Argument '--format' use string format from:
    https://docs.rs/chrono/latest/chrono/format/strftime/index.html
```
