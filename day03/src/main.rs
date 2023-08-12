use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader}, error::Error,
};

use itertools::Itertools;

const A_UPPERCASE_VALUE: u32 = 'A' as u32;
const A_LOWERCASE_VALUE: u32 = 'a' as u32;

fn convert_char(c: &char) -> u32 {
    match c {
        'a'..='z' => *c as u32 - A_LOWERCASE_VALUE + 1,
        'A'..='Z' => *c as u32 - A_UPPERCASE_VALUE + 27,
        _ => 0,
    }
}

fn main() -> std::io::Result<()> {
    let input_file_name = "./input.txt";

    let buf_reader = get_file_handle(input_file_name)?;
    let total_score_pt1: u32 = buf_reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            let line_length = line.len();
            let halves = line.split_at(line_length / 2);
            let set: HashSet<char> = halves.0.chars().collect();
            halves
                .1
                .chars()
                .find(|el| set.contains(el))
                .map_or(0, |c| convert_char(&c))
        })
        .sum();

    println!("Total misplaced items value: {}", total_score_pt1);

    let buf_reader = get_file_handle(input_file_name)?;
    let total_score_pt2: u32 = buf_reader
        .lines()
        .filter_map(|line| line.ok())
        .chunks(3)
        .into_iter()
        .map(|mut chunk| {
            let set1: HashSet<char> = chunk.next().unwrap().chars().collect();
            let set2: HashSet<char> = chunk.next().unwrap().chars().collect();

            chunk
                .next()
                .unwrap()
                .chars()
                .find(|el| set1.contains(el) && set2.contains(el))
                .map_or(0, |c| convert_char(&c))
        })
        .sum();
    println!("Total priorities value: {}", total_score_pt2);
    Ok(())
}

fn get_file_handle(input_file_name:&str)->Result<BufReader<File>, std::io::Error>{
    let file = File::open(input_file_name)?;
    Ok(BufReader::new(file))
}
