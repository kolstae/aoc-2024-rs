#![feature(iter_map_windows)]

use std::fs::read_to_string;

#[allow(unused_variables)]
fn main() {
    let input = read_to_string("data/day02.txt").unwrap();
    let _input = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";

    let ns: Vec<_> = input.lines().map(split_ns).collect();
    // println!("{ns:?}");

    let count = ns.iter().filter(|is| is_safe(is)).count();
    println!("Part 1: {count}");

    let count = ns
        .iter()
        .filter(|is| is_safe(is) || is_safe_lax(is))
        .count();
    println!("Part 2: {count}");
}

fn is_safe_lax(is: &[u32]) -> bool {
    (0..is.len()).any(|i| is_safe(&[&is[..i], &is[(i + 1)..]].concat()))
}

fn is_safe(is: &[u32]) -> bool {
    (is.is_sorted_by(|a, b| a < b)
        && is
            .iter()
            .map_windows(|&[a, b]| b - a)
            .all(|d| (1..=3).contains(&d)))
        || (is.is_sorted_by(|a, b| a > b)
            && is
                .iter()
                .map_windows(|&[a, b]| a - b)
                .all(|d| (1..=3).contains(&d)))
}

fn split_ns(s: &str) -> Vec<u32> {
    s.split_whitespace().map(|s| s.parse().unwrap()).collect()
}
