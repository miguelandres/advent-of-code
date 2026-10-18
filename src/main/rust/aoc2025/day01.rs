use advent_of_code::utils::read_aoc_input;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LockCounter {
    lock_pointer: i32,
    counter: i32,
}

fn main() {
    let lines = read_aoc_input(2025, 1, 1);
    let input: Vec<i32> = lines
        .iter()
        .map(|s| {
            let sign = if s.starts_with('L') { -1 } else { 1 };
            let distance: i32 = s[1..].parse().expect("Invalid distance");
            sign * distance
        })
        .collect();

    let part1_result = input.iter().fold(
        LockCounter {
            lock_pointer: 50,
            counter: 0,
        },
        |acc, &mv| {
            let next_pointer = (acc.lock_pointer + mv).rem_euclid(100);
            LockCounter {
                lock_pointer: next_pointer,
                counter: if next_pointer == 0 {
                    acc.counter + 1
                } else {
                    acc.counter
                },
            }
        },
    );
    println!("{part1_result:?}");

    let part2_result = input.iter().fold(
        LockCounter {
            lock_pointer: 50,
            counter: 0,
        },
        |acc, &mv| {
            let sum = acc.lock_pointer + mv;
            let count = if sum <= 0 {
                if acc.lock_pointer == 0 {
                    sum.abs() / 100
                } else {
                    1 + sum.abs() / 100
                }
            } else {
                sum / 100
            };
            let next_pointer = sum.rem_euclid(100);
            LockCounter {
                lock_pointer: next_pointer,
                counter: count + acc.counter,
            }
        },
    );
    println!("{part2_result:?}");
}
