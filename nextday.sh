#!/bin/zsh

set -e

if [[ "$1" =~ ^[0-9]+$ ]]; then
  lang="rust"
  day="$1"
  year="${2:-$(date +%Y)}"
else
  lang="${1:l}"
  day="$2"
  year="${3:-$(date +%Y)}"
fi

if [[ -z "$day" ]]; then
  echo "Usage: ./nextday.sh [kotlin|rust|both] <dayNumber> [year]"
  exit 1
fi

padded=$(printf "%02d" "$day")

if [[ "$lang" == "kotlin" || "$lang" == "kt" || "$lang" == "both" ]]; then
  mkdir -p "src/main/kotlin/aoc${year}"
  target_kt="src/main/kotlin/aoc${year}/Day${padded}.kt"
  if [[ ! -a "$target_kt" ]]; then
    {
      echo "package aoc${year}"
      echo ""
      sed -e "s/1/$day/g" src/main/kotlin/Template.kt
    } > "$target_kt"
    echo "Created $target_kt"
  fi
fi

if [[ "$lang" == "rust" || "$lang" == "rs" || "$lang" == "both" ]]; then
  mkdir -p "src/main/rust/aoc${year}"
  target_rs="src/main/rust/aoc${year}/day${padded}.rs"
  bin_name="aoc${year}_day${padded}"
  if [[ ! -a "$target_rs" ]]; then
    sed -e "s/(2026, 1, 0)/($year, $day, 0)/g" src/main/rust/template.rs > "$target_rs"
    echo "Created $target_rs"
  fi
  if ! grep -q "name = \"${bin_name}\"" Cargo.toml; then
    cat <<EOF >> Cargo.toml

[[bin]]
name = "${bin_name}"
path = "${target_rs}"
EOF
    echo "Registered [[bin]] ${bin_name} in Cargo.toml"
  fi
fi

mkdir -p "input/aoc${year}"
touch "input/aoc${year}/d${padded}p00.txt"
touch "input/aoc${year}/d${padded}p01.txt"
