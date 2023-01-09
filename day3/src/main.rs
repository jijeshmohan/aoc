use itertools::Itertools;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn score(value: u8) -> usize {
    match value {
        b'a'..=b'z' => 1 + (value - b'a') as usize,
        b'A'..=b'Z' => 27 + (value - b'A') as usize,
        _ => panic!("invalid value"),
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let file_path: &str = "/Users/jijeshmohan/Learn/rust/aoc/day3/input.txt";
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let result = reader
        .lines()
        .flatten()
        .map(|f| f.as_bytes().to_vec())
        .map(|x| {
            x.iter().fold(HashSet::new(), |mut set, value: &u8| {
                set.insert(*value);
                set
            })
        })
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            let priority = *chunk
                .reduce(|a, b| a.intersection(&b).cloned().collect())
                .expect("we should have 3 elements")
                .iter()
                .next()
                .expect("there should be only one item");
            score(priority)
        })
        // .batching(|line| {
        //     if let (Some(x), Some(y), Some(z)) = (line.next(), line.next(), line.next()) {
        //         Some((x, y, z))
        //     } else {
        //         None
        //     }
        // })
        // .map(|(x, y, z)| -> usize {
        //     x.iter()
        //         .copied()
        //         .find(|i| y.contains(i) && z.contains(i))
        //         .map(score)
        //         .unwrap_or_default()
        // })
        .sum::<usize>();

    println!("{}", result);
    Ok(())
}
