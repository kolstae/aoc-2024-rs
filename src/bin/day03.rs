#![feature(iter_map_windows)]

use regex::Regex;
use std::fs::read_to_string;

#[allow(unused_variables)]
fn main() {
    let input = &read_to_string("data/day03.txt").unwrap();
    let _input = r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let sum = sum_mul(&re, input);
    println!("Part 1: {sum}");

    let _input = r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    let re2 = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();
    let mut muls = Vec::new();
    let mut include = true;
    for s in re2.find_iter(input) {
        let s = s.as_str();
        if s.starts_with("don") {
            include = false
        } else if s.starts_with("do") {
            include = true
        } else if include {
            muls.push(s);
        }
    }
    // println!("{muls:?}");

    let sum = sum_mul(&re, &muls.into_iter().collect::<String>());
    println!("Part 2: {sum}");
}

fn sum_mul(re: &Regex, input: &str) -> u64 {
    re.captures_iter(input)
        .map(|x| {
            let (_, [a, b]) = x.extract();
            a.parse::<u64>().unwrap() * b.parse::<u64>().unwrap()
        })
        .sum()
}
