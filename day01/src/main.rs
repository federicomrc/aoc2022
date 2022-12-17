use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> std::io::Result<()> {
    let input_file_name = "./input.txt";

    let file = File::open(input_file_name)?;
    let buf_reader = BufReader::new(file);

    let top_three: Vec<u64> = buf_reader
        .lines()
        .filter_map(|el| el.ok())
        .into_iter()
        .group_by(|elt| !(*elt).is_empty())
        .into_iter()
        .filter(|(key, _)| *key)
        .map(|(_, group)| group.filter_map(|el| el.parse::<u64>().ok()).sum::<u64>())
        .sorted_by(|a, b| {
            if a > b {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        })
        .take(3)
        .collect();

    println!("Riposta prima parte: {}", top_three[0]);
    println!("Riposta seconda parte: {}", top_three.into_iter().sum::<u64>());

    Ok(())
}
