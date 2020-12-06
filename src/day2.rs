use aoc_runner_derive::*;
use scan_fmt::*;

#[derive(Debug)]
struct Entry {
    min: usize,
    max: usize,
    c: char,
    password: String,
}

#[aoc_generator(day2)]
fn day2_gen(input: &str) -> Vec<Entry> {
    input
        .split("\n")
        .map(|s| scan_fmt!(s, "{d}-{d} {/(.):/}{}", usize, usize, char, String).unwrap())
        .map(|(min, max, c, password)| Entry {
            min,
            max,
            c,
            password,
        })
        .collect()
}

#[aoc(day2, part1)]
fn day2p1(input: &Vec<Entry>) -> usize {
    input
        .iter()
        .filter(|entry| {
            let num_matching = entry.password.chars().filter(|c| *c == entry.c).count();
            entry.min <= num_matching && entry.max >= num_matching
        })
        .count()
}

#[aoc(day2, part2)]
fn day2p2(input: &Vec<Entry>) -> usize {
    input
        .iter()
        .filter(|entry| {
            entry
                .password
                .chars()
                .nth(entry.min - 1)
                .map(|c| c == entry.c)
                .unwrap_or(false)
                ^ entry
                    .password
                    .chars()
                    .nth(entry.max - 1)
                    .map(|c| c == entry.c)
                    .unwrap_or(false)
        })
        .count()
}
