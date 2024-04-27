# Command-Line Help for `datetag`

This document contains the help content for the `datetag` command-line program.

**Command Overview:**

* [`datetag`↴](#datetag)

## `datetag`

display a customizable date tag (e.g. TEST_202110)

EXAMPLES:
 
    $ datetag --offset 22 --date 20220312 --prefix 'TEST_' --tag-type daily
    TEST_20220403

    $ datetag -o 22 -d 20220312 -p 'TEST_' -t d
    TEST_20220403

**Usage:** `datetag [OPTIONS]`

###### **Options:**

* `-t`, `--tag-type <TAG_TYPE>` — tag type [d | m | y | daily | monthly | yearly]

  Default value: `m`

  Possible values:
  - `y`
  - `yearly`:
    yearly tags (e.g. 2022)
  - `m`
  - `monthly`:
    monthly tags (e.g. 202212)
  - `d`
  - `daily`:
    daily tags (e.g. 20221230)

* `-p`, `--prefix <PREFIX>` — tag prefix
* `-d`, `--date <DATE>` — date tag value (one of 'yyyymmdd', 'yyyymm', 'yyyy')
* `-o`, `--offset <OFFSET>` — date tag offset

  Default value: `0`
* `-r`, `--repeat <REPEAT>` — generate more date tags

  Default value: `1`
* `-n`, `--new-line` — append an end-of-line

  Default value: `false`

  Possible values: `true`, `false`

* `--markdown-help` — print this help as markdown document

  Possible values: `true`, `false`




<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

