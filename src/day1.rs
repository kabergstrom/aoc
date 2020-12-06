use std::cmp::{max, min, Ordering};

use aoc_runner::*;
use aoc_runner_derive::*;

#[aoc_generator(day1)]
fn day1_gen(input: &str) -> Vec<u32> {
    let mut values: Vec<u32> = input
        .split("\n")
        .map(|s| s.trim().parse().unwrap())
        .collect();
    values.sort_unstable();
    values
}

#[aoc(day1, part1)]
fn day1p1(input: &Vec<u32>) -> u32 {
    let mut top = input.iter();
    let mut bottom = input.iter().rev();
    let mut a = *top.next().unwrap();
    let mut b = *bottom.next().unwrap();
    loop {
        match (a + b).cmp(&2020) {
            Ordering::Greater => b = *bottom.next().unwrap(),
            Ordering::Less => a = *top.next().unwrap(),
            Ordering::Equal => return a * b,
        }
    }
}

#[aoc(day1, part2)]
fn day1p2(input: &Vec<u32>) -> u32 {
    let mut top = (0..input.len()).into_iter().map(|i| (i, input[i]));
    let mut bottom = (0..input.len()).into_iter().map(|i| (i, input[i]));
    let mut a = top.next().unwrap();
    let mut b = bottom.next().unwrap();
    loop {
        match (a.1 + b.1 + max(a.1, b.1)).cmp(&2020) {
            Ordering::Less => b = bottom.next().unwrap(),
            Ordering::Greater => {
                for i in &input[(a.0 + 1)..b.0] {
                    if a.1 + b.1 + i == 2020 {
                        return a.1 * b.1 * *i;
                    }
                }
                a = top.next().unwrap()
            }
            Ordering::Equal => return a.1 + b.1 + max(a.1, b.1),
        }
    }
}
