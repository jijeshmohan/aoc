use itertools::Itertools;
use std::io::BufRead;
use std::ops::RangeInclusive;
use std::{fs::File, io::BufReader};

trait RangeInclusiveExt {
    fn contains_range(&self, other: &Self) -> bool;

    fn contains_or_is_contain(&self, other: &Self) -> bool {
        self.contains_range(other) || other.contains_range(self)
    }

    fn overlaps(&self, other: &Self) -> bool;

    fn overlaps_or_is_overlaped(&self, other: &Self) -> bool {
        self.overlaps(other) || other.overlaps(self)
    }
}

impl<T: PartialOrd> RangeInclusiveExt for RangeInclusive<T> {
    fn contains_range(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.contains(other.start()) || self.contains(other.end())
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let file_path: &str = "/Users/jijeshmohan/Learn/rust/aoc/day4/input.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // let count = get_count(reader, |(a, b)| a.contains_or_is_contain(b));
    // println!("Part 1 Total: {}", count);

    println!(
        "Part 2 Total: {}",
        get_count(reader, |(a, b)| a.overlaps_or_is_overlaped(b))
    );

    Ok(())
}

fn get_count<F>(reader: BufReader<File>, f: F) -> usize
where
    F: Fn(&(RangeInclusive<i32>, RangeInclusive<i32>)) -> bool,
{
    reader
        .lines()
        .flatten()
        .map(|line| {
            line.split(',')
                .map(|range| {
                    let start_end: Vec<i32> = range
                        .split('-')
                        .map(|n| n.parse().expect("start and end should present"))
                        .collect();

                    start_end[0]..=start_end[1]
                })
                .collect_tuple::<(_, _)>()
                .expect("unable to parse tuples")
        })
        .filter(f)
        .count()
}
