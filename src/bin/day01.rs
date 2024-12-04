use std::collections::HashMap;
use std::fs::read_to_string;

#[allow(unused_variables)]
fn main() {
    let input = read_to_string("data/day01.txt").unwrap();
    let _input = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";

    let (mut xs, mut ys): (Vec<_>, Vec<_>) = input.lines().map(split_ns).unzip();
    xs.sort();
    ys.sort();
    // println!("{xs:?} {ys:?}");
    let sum: u32 = xs.iter().zip(ys.iter()).map(|(x, &y)| x.abs_diff(y)).sum();
    println!("Part 1: {sum}");

    let mut freqs = HashMap::new();
    ys.iter().for_each(|&y| {
        freqs.entry(y).and_modify(|x| *x += 1).or_insert(1);
    });
    let sum: u32 = xs.iter().map(|&x| x * freqs.get(&x).copied().unwrap_or(0)).sum();
    println!("Part 2: {sum}");
}

fn split_ns(s: &str) -> (u32, u32) {
    let mut ws = s.split_whitespace();
    (ws.next().unwrap().parse().unwrap(), ws.next().unwrap().parse().unwrap())
}
