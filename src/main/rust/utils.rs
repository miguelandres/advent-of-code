use std::fmt::Debug;
use std::fs;
use std::ops::RangeInclusive;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Instant;

pub fn aoc_input_path(year: u16, day: u8, part: u8) -> PathBuf {
    PathBuf::from(format!("input/aoc{year}/d{day:02}p{part:02}.txt"))
}

pub fn read_aoc_input_raw(year: u16, day: u8, part: u8) -> String {
    let path = aoc_input_path(year, day, part);
    fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("Failed to read AoC input file {}: {err}", path.display()))
}

pub fn read_aoc_input(year: u16, day: u8, part: u8) -> Vec<String> {
    read_aoc_input_raw(year, day, part)
        .lines()
        .map(str::to_owned)
        .collect()
}

pub fn remove_affixes<'a>(s: &'a str, prefix: &str, suffix: &str) -> &'a str {
    let s = s.strip_prefix(prefix).unwrap_or(s);
    s.strip_suffix(suffix).unwrap_or(s)
}

pub fn split_with_affixes<'a>(
    s: &'a str,
    prefix: &str,
    delimiter: &str,
    suffix: &str,
) -> Vec<&'a str> {
    remove_affixes(s, prefix, suffix).split(delimiter).collect()
}

pub fn split_with_prefix<'a>(s: &'a str, prefix: &str, delimiter: &str) -> Vec<&'a str> {
    s.strip_prefix(prefix)
        .unwrap_or(s)
        .split(delimiter)
        .collect()
}

pub fn parse_range<T>(s: &str) -> RangeInclusive<T>
where
    T: FromStr,
    T::Err: Debug,
{
    let (start, end) = s
        .split_once('-')
        .unwrap_or_else(|| panic!("Invalid range string: {s}"));
    let start = start.parse::<T>().expect("Failed to parse range start");
    let end = end.parse::<T>().expect("Failed to parse range end");
    start..=end
}

pub fn wrap_in_time_measurement<F, T>(description: &str, block: F) -> T
where
    F: FnOnce() -> T,
{
    let start = Instant::now();
    let result = block();
    println!("{description} took {}ms.", start.elapsed().as_millis());
    result
}
