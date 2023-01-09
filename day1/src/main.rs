use itertools::Itertools;
use std::{
    cmp::Reverse,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file_path: &str = "/Users/jijeshmohan/Learn/rust/aoc/day1/input.txt";
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let max = reader
        .lines()
        .filter_map(|f| f.ok())
        .map(|f| f.parse::<u64>().ok())
        .batching(|it| it.map_while(|l| l).sum1::<u64>())
        .map(Reverse)
        .k_smallest(3)
        .map(|x| x.0)
        .sum::<u64>();

    // .collect::<Vec<_>>();

    println!("{:?}", max)
}
