<img src="./.assets/christmas_ferris.png" width="164">

# 🎄 Advent of Code 2015

Solutions for [Advent of Code](https://adventofcode.com/2015) in [Rust](https://www.rust-lang.org/).

<!--- advent_readme_stars table --->
## 2015 Results

| Day | Part 1 | Part 2 |
| :---: | :---: | :---: |
| [Day 1](https://adventofcode.com/2015/day/1) | ⭐ | ⭐ |
| [Day 2](https://adventofcode.com/2015/day/2) | ⭐ | ⭐ |
| [Day 3](https://adventofcode.com/2015/day/3) | ⭐ |   |
<!--- advent_readme_stars table --->

<!--- benchmarking table --->

---

## Usage

### Scaffold a day

```sh
cargo scaffold 1 --download   # Create scaffolding for day 1 and download puzzle input
cargo scaffold 1              # Create scaffolding for day 1
cargo download 1              # Download puzzle input for day 1
cargo read 1                  # Read puzzle description for day 1
```

### Testing

```sh
cargo test
cargo test --bin 01
cargo test --bin 01 part_one
```

### Use VS Code to debug your code

1. Install [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) and [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb)
2. Set breakpoints in your code
3. Click _Debug_ next to the unit test or the _main_ function
4. The debugger will halt your program at the specific line and allow you to inspect the local stack

### Run solutions for a day

```sh
cargo solve 1              # Run day 1 solution against real puzzle inputs
cargo solve 1 2            # Run day 1, part 2 solution against real puzzle inputs
cargo solve 1 2 --release  # Run day 1, part 2 solution against real puzzle inputs with an optimized build
cargo all                  # Run all solutions sequentially
cargo all --release        # Run all solutions sequentially with optimized build
cargo solve 1 --submit 2   # Submit day 1, part 2 solution
```

### Benchmarking

```sh
cargo time 1              # Run day 1 10-10K times (depending on execution time) and print the average runtime
cargo time --all          # Print the average runtime for all days
cargo solve 1 --dhat      # Analyze day 1 heap allocations with DHAT (https://valgrind.org/docs/manual/dh-manual.html)
cargo time 1 --store      # Add day 1 timings to the readme
cargo time --all --store  # Add all timings to the readme
```

The `--dhat` command will output some basic stats to the command-line and generate a `dhat-heap.json` report in the repo root directory. You can pass the report a tool like [dh-view](https://nnethercote.github.io/dh_view/dh_view.html) to view a detailed breakdown of heap allocations.

### Linting and formatting

```sh
cargo fmt
cargo clippy
```

## Common pitfalls

-  **Integer overflows:** This template uses 32-bit integers by default because it is generally faster - for example when packed in large arrays or structs - than using 64-bit integers everywhere. For some problems, solutions for real input might exceed 32-bit integer space. While this is checked and panics in `debug` mode, integers [wrap](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow) in `release` mode, leading to wrong output when running your solution.

## Session cookie

1. The session cookie might expire after a while (~1 month) which causes the downloads to fail. To fix this issue, refresh the `.adventofcode.session` file.
2. The session cookie might expire after a while (~1 month) which causes the automated workflow to fail. To fix this issue, refresh the AOC_SESSION secret.
2. Create the file `$HOME/.adventofcode.session` and paste your session cookie into it. To retrieve the session cookie, press F12 anywhere on the Advent of Code website to open your browser developer tools. Look in _Cookies_ under the _Application_ or _Storage_ tab, and copy out the `session` cookie value. [^1]
