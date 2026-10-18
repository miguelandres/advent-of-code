# Miguel's Advent of Code

Unified Advent of Code repository with solutions organized by year (`aoc2020`, `aoc2021`, `aoc2022`, `aoc2023`, `aoc2025`, `aoc2026`) in Kotlin, Scala (`build.gradle.kts`), and Rust (`Cargo.toml`).

## Structure

- `src/main/kotlin/aoc<year>/`: Kotlin solutions for each year (`aoc2022`, `aoc2023`, `aoc2025`, `aoc2026`)
- `src/main/scala/aoc<year>/`: Scala solutions for each year (`aoc2020`, `aoc2021`, `aoc2022`)
- `src/main/rust/aoc<year>/`: Rust solutions (`aoc2025`, `aoc2026`) and shared utilities
- `src/main/kotlin/{collections,geometry,graph,grid,utils}/`: Shared Kotlin utilities across years
- `input/aoc<year>/`: Puzzle inputs for each year (tracked in the private `aoc-inputs` Git submodule)

## Cloning & Submodules

Clone the repository along with the `input` submodule:

```sh
git clone --recurse-submodules https://github.com/miguelandres/advent-of-code.git
```

If you already cloned the repository without `--recurse-submodules`, initialize and download the submodule with:

```sh
git submodule update --init --recursive
```

## Environment Setup on macOS

```sh
brew install temurin21 kotlin pre-commit rustup
brew install --cask intellij-idea
pre-commit install
```

## Start a new day

```sh
./nextday.sh rust <dayNumber> [year]
./nextday.sh kotlin <dayNumber> [year]
```

## Run a day from console

Kotlin / Scala:

```sh
./gradlew runDay -Pyear=2025 -Pday=01
```

Rust:

```sh
cargo run --bin aoc2025_day01
```
