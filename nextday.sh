#!/bin/zsh

padded=`echo ${(l:2::0:)1}`
year=${2:-$(date +%Y)}

mkdir -p src/main/kotlin/aoc${year}

if [[ ! -a src/main/kotlin/aoc${year}/Day${padded}.kt ]]; then
  {
    echo "package aoc${year}"
    echo ""
    sed -e "s/1/$1/g" src/main/kotlin/Template.kt
  } > src/main/kotlin/aoc${year}/Day${padded}.kt
fi

mkdir -p input/aoc${year}

touch input/aoc${year}/d${padded}p00.txt
touch input/aoc${year}/d${padded}p01.txt
