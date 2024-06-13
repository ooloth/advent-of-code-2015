<img src="./.assets/christmas_ferris.png" width="164">

# 🎄 Advent of Code 2015

Solutions for [Advent of Code](https://adventofcode.com/2015) in [Rust](https://www.rust-lang.org/).

<!--- advent_readme_stars table --->
## 2015 Results

| Day | Part 1 | Part 2 |
| :---: | :---: | :---: |
| [Day 1](https://adventofcode.com/2015/day/1) | ⭐ | ⭐ |
| [Day 2](https://adventofcode.com/2015/day/2) | ⭐ | ⭐ |
| [Day 3](https://adventofcode.com/2015/day/3) | ⭐ | ⭐ |
| [Day 4](https://adventofcode.com/2015/day/4) | ⭐ | ⭐ |
| [Day 5](https://adventofcode.com/2015/day/5) | ⭐ | ⭐ |
| [Day 6](https://adventofcode.com/2015/day/6) | ⭐ | ⭐ |
<!--- advent_readme_stars table --->

<!--- benchmarking table --->

---

## Usage

### Scaffolding files and downloading puzzle input

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

### Debugging

1. Install the `rust-analyzer` and `CodeLLDB` extensions
2. Set breakpoints in your code
3. Click _Debug_ next to the unit test or the _main_ function
4. The debugger will halt your program at the specific line and allow you to inspect the local stack

### Running and submitting

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

## Session cookie for AOC CLI

1. Copy the `session` 🍪 from the [AOC website](https://adventofcode.com) (you'll find it under _Cookies_ in the _Application_ or _Storage_ tab)
2. Create a `$HOME/.adventofcode.session` file and paste the 🍪 there
3. Create an `AOC_SESSION` repo secret and paste the 🍪 there as well (Settings -> Secrets -> New repository secret)
4. When the 🍪 expires, repeat steps 1-3, replacing the old value with the new one

## Common pitfalls

- **Whitespace in input:** Make sure the input file has no leading or trailing whitespace, including no newline at the end of the file.
- **Integer overflows:** This template uses 32-bit integers by default because it is generally faster - for example when packed in large arrays or structs - than using 64-bit integers everywhere. For some problems, solutions for real input might exceed 32-bit integer space. While this is checked and panics in `debug` mode, integers [wrap](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow) in `release` mode, leading to wrong output when running your solution.
