# Miguel's Advent of Code

Unified Advent of Code repository with solutions organized by year (`aoc2020`, `aoc2021`, `aoc2022`, `aoc2023`, `aoc2025`) in Kotlin and Scala using Gradle (`build.gradle.kts`).

## Structure

- `src/main/kotlin/aoc<year>/`: Kotlin solutions for each year (`aoc2022`, `aoc2023`, `aoc2025`)
- `src/main/scala/aoc<year>/`: Scala solutions for each year (`aoc2020`, `aoc2021`, `aoc2022`)
- `src/main/kotlin/{collections,geometry,graph,grid,utils}/`: Shared utilities across years
- `input/aoc<year>/`: Puzzle inputs for each year

## Environment Setup on macOS

```sh
brew install temurin21 kotlin pre-commit
brew install --cask intellij-idea-ce
pre-commit install
```

## Start a new day

```sh
./nextday.sh <dayNumber> [year]
```

## Run a day from console

```sh
./gradlew runDay -Pyear=2025 -Pday=01
```
