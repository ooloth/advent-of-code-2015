<img src="./.assets/christmas_ferris.png" width="164">

# 🎄 Advent of Code 2015

Solutions for [Advent of Code](https://adventofcode.com/2015) in [Rust](https://www.rust-lang.org/).

<!--- advent_readme_stars table --->
## 2015 Results

| Day | Part 1 | Part 2 |
| :---: | :---: | :---: |
| [Day 1](https://adventofcode.com/2015/day/1) | ⭐ | ⭐ |
<!--- advent_readme_stars table --->

<!--- benchmarking table --->

---

## Usage

## Scaffold a day

You can automatically download puzzle input and description by either appending the `--download` flag to `scaffold` (e.g. `cargo scaffold 4 --download`) or with the separate `download` command:

```sh
# example: `cargo scaffold 1 --download`
cargo scaffold <day>
cargo scaffold <day> --download
cargo download <day>
```

> [!TIP]
> If a day has multiple example inputs, you can use the `read_file_part()` helper in your tests instead of `read_file()`. If this e.g. applies to day 1, you can create a second example file `01-2.txt` and invoke the helper like `let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));`. This supports an arbitrary number of example files.

## Read puzzle description

```sh
# example: `cargo read 1`
cargo read <day>
```

## Linting and formatting

```sh
cargo fmt
cargo clippy
```

## Testing

```sh
cargo test

# example: cargo test --bin 01
cargo test --bin <day>

# example: cargo test --bin 01 part_one
cargo test --bin <day> <part>
```

## Benchmarking

You can benchmark one day or all days and store timings in the readme. The runner will run your code between 10 and 10,000 times, depending on execution time of first execution, and print the average.

```sh
# example: `cargo time 8 --store`
cargo time <day> --store
cargo time --all --store
```

## Run solutions for a day

If you are not only interested in the runtime of your solution, but also its heap allocation profile, you can use the template's [DHAT](https://valgrind.org/docs/manual/dh-manual.html) integration to analyze it. In order to activate DHAT, call the `solve` command with the `--dhat` flag.

```sh
# example: `cargo solve 01`
# example: `cargo solve 1 --dhat`
cargo solve <day>
cargo solve <day> <part>
cargo solve <day> --dhat
cargo solve <day> --submit <part>
```

The `solve` command runs your solution against real puzzle inputs. To run an optimized build of your code, append the `--release` flag as with any other rust program.

## Submitting solutions

Append the `--submit <part>` option to the `solve` command to submit your solution for checking.

```sh
# Run all solutions sequentially
cargo all

# Run all solutions sequentially with optimized build
cargo all --release
```

## Optional template features

The command will output some basic stats to the command-line and generate a `dhat-heap.json` report in the repo root directory.

You can pass the report a tool like [dh-view](https://nnethercote.github.io/dh_view/dh_view.html) to view a detailed breakdown of heap allocations.

### Use VS Code to debug your code

1. Install [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) and [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb).
2. Set breakpoints in your code. [^3]
3. Click _Debug_ next to the unit test or the _main_ function. [^4]
4. The debugger will halt your program at the specific line and allow you to inspect the local stack. [^5]

## Common pitfalls

-  **Integer overflows:** This template uses 32-bit integers by default because it is generally faster - for example when packed in large arrays or structs - than using 64-bit integers everywhere. For some problems, solutions for real input might exceed 32-bit integer space. While this is checked and panics in `debug` mode, integers [wrap](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow) in `release` mode, leading to wrong output when running your solution.

## Session cookie

1. The session cookie might expire after a while (~1 month) which causes the downloads to fail. To fix this issue, refresh the `.adventofcode.session` file.
2. The session cookie might expire after a while (~1 month) which causes the automated workflow to fail. To fix this issue, refresh the AOC_SESSION secret.
2. Create the file `$HOME/.adventofcode.session` and paste your session cookie into it. To retrieve the session cookie, press F12 anywhere on the Advent of Code website to open your browser developer tools. Look in _Cookies_ under the _Application_ or _Storage_ tab, and copy out the `session` cookie value. [^1]
