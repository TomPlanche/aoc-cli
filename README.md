# Advent of code CLI.

> Poorer, less cool and less complete version of [fspoettel's aoc](https://github.com/fspoettel/advent-of-code-rust#configure-aoc-cli-integration) CLI. I just wanted to try out Rust.

## Motivation
I wanted a simple CLI to create the files and folders for the advent of code challenges. I also wanted to learn rust so I'll try to do the most of AOC in rust.

The file structure looks like this:
```text
.
├── src/
│   ├── main.rs
│   ├── bin/
│   │   ├── aoc_cli
│   │   ├── download_input.rs
│   │   └── solve.rs
│   ├── year_2015/
│   │   ├── day_01.rs
│   │   ├── ...
│   │   └── day_25.rs
│   └── year_n/
└── data/
    ├── year_2015/
    │   ├──puzzles/
    │   │   ├── day_01.md
    │   │   ├── ...
    │   │   └── day_25.md
    │   └── inputs/
    │       ├── day_01.txt
    │       ├── ...
    │       └── day_25.txt
    └── year_n/
```

## Usage
### Init
Init the project files and folders. This will create the skeleton of the project for the given year (default is 2015).
```bash
aoc init [year]
```

### Create
Create the files for the given day. This will create the files for the given day in the `src/year_n/day_n.rs` and `data/year_n/puzzles/day_n.md` and `data/year_n/inputs/day_n.txt` files.
```bash
aoc create [day] [year]
```
