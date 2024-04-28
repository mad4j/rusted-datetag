# datetag

[![Crate](https://img.shields.io/crates/v/datetag)](https://crates.io/crates/datetag)
![Crates.io](https://img.shields.io/crates/l/datetag)

```text
Display a customizable date tag (e.g. TEST_202404)

EXAMPLES:

    $ datetag --offset 22 --date 20220312 --prefix 'TEST_' --tag-type daily
    TEST_20220403

    $ datetag -o 22 -d 20220312 -p 'TEST_' -t d
    TEST_20220403

Usage: datetag.exe [OPTIONS]

Options:
  -t, --tag-type <TAG_TYPE>
          Tag type [d | m | y | daily | monthly | yearly]

          [default: m]

          Possible values:
          - y
          - yearly:  yearly tags (e.g. 2022)
          - m
          - monthly: monthly tags (e.g. 202212)
          - d
          - daily:   daily tags (e.g. 20221230)

  -p, --prefix <PREFIX>
          Tag prefix (e.g. 'LAB_202404)

  -s, --suffix <SUFFIX>
          Tag suffix (e.g. '202404_rel)

  -d, --date <DATE>
          Date value (one of 'yyyymmdd', 'yyyymm', 'yyyy')

  -o, --offset <OFFSET>
          Date offset (offset unit depends on -t value)

          [default: 0]

  -r, --repeat <REPEAT>
          Generate more date tags

          [default: 1]

  -n, --new-line
          Append an end-of-line to each generated tag

      --markdown-help
          Print this help as markdown document

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
