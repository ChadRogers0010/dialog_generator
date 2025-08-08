# Command-Line Help for `dialog`

This document contains the help content for the `dialog` command-line program.

**Command Overview:**

* [`dialog`↴](#dialog)
* [`dialog csv`↴](#dialog-csv)
* [`dialog build`↴](#dialog-build)
* [`dialog build-c`↴](#dialog-build-c)
* [`dialog test`↴](#dialog-test)

## `dialog`

**Usage:** `dialog [OPTIONS] [COMMAND]`

###### **Subcommands:**

* `csv` — Generate a test csv
* `build` — build the Dialog_lib::query()
* `build-c` — build the c query
* `test` — Test a build

###### **Options:**

* `-c <CSV_PATH>`

  Default value: `./test.csv`



## `dialog csv`

Generate a test csv

**Usage:** `dialog csv -s <STATEMENTS> -p <PREDICATES>`

###### **Options:**

* `-s <STATEMENTS>`
* `-p <PREDICATES>`



## `dialog build`

build the Dialog_lib::query()

**Usage:** `dialog build [OPTIONS]`

###### **Options:**

* `-l <LINES_PER_MODULE>`

  Default value: `100`



## `dialog build-c`

build the c query

**Usage:** `dialog build-c [OPTIONS]`

###### **Options:**

* `-l <LINES_PER_MODULE>`

  Default value: `100`



## `dialog test`

Test a build

**Usage:** `dialog test [OPTIONS]`

###### **Options:**

* `-c <COUNT>` — Number of times to run the test

  Default value: `1`
* `-r` — All responses succeed
* `-m` — Multithread with Rayon
* `-f` — Rayon's into_par_iter().map().collect()
* `-a` — Test every test case



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

Usage: dialog [OPTIONS] [COMMAND]

Commands:
  csv      Generate a test csv
  build    build the Dialog_lib::query()
  build-c  build the c query
  test     Test a build
  help     Print this message or the help of the given subcommand(s)

Options:
  -c <CSV_PATH>      [default: ./test.csv]
  -h, --help         Print help
