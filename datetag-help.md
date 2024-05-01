# Command-Line Help for `datetag`

This document contains the help content for the `datetag` command-line program.

**Command Overview:**

* [`datetag`↴](#datetag)

## `datetag`

Display a customizable date tag (e.g. TEST_202404)

EXAMPLES:
 
    $ datetag --offset 22 --date 20220312 --prefix 'TEST_' --tag-type daily
    TEST_20220403

    $ datetag -o 22 -d 20220312 -p 'TEST_' -t d
    TEST_20220403

**Usage:** `datetag [OPTIONS]`

###### **Options:**

* `-t`, `--tag-type <TAG_TYPE>` — Tag type [d | m | y | daily | monthly | yearly]

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

* `-s`, `--style <STYLE>` — Date tag style

  Default value: `plain`

  Possible values:
  - `plain`:
    yyyymmdd
  - `dot`:
    yyyy.mm.dd
  - `slash`:
    yyyy/mm/dd
  - `colon`:
    yyyy:mm:dd
  - `dash`:
    yyyy-mm-dd

* `-p`, `--prefix <PREFIX>` — Tag prefix (e.g. 'LAB_202404')
* `-x`, `--suffix <SUFFIX>` — Tag suffix (e.g. '202404_rel')
* `-d`, `--date <DATE>` — Date value (one of 'yyyymmdd', 'yyyymm', 'yyyy')
* `-o`, `--offset <OFFSET>` — Date offset (offset unit depends on -t value)

  Default value: `0`
* `-r`, `--repeat <REPEAT>` — Generate more date tags

  Default value: `1`
* `-n`, `--new-line` — Append an end-of-line to each generated tag

  Default value: `false`

  Possible values: `true`, `false`

* `--markdown-help` — Print this help as markdown document

  Possible values: `true`, `false`




<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

