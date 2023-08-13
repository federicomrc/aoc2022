use once_cell::sync::Lazy;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

static REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?P<first_from>\d+)-(?P<first_to>\d+),(?P<second_from>\d+)-(?P<second_to>\d+)")
        .unwrap()
});

#[derive(Debug, Eq, PartialEq)]
struct Assignment {
    from: u32,
    to: u32,
}

impl Assignment {
    fn contains(&self, other: &Self) -> bool {
        self.from <= other.from && self.to >= other.to
    }

    fn overlap(&self, other: &Self) -> bool {
        (self.from >= other.from && self.from <= other.to)
            || (self.to >= other.from && self.to <= other.to)
            || (other.from >= self.from && other.from <= self.to)
            || (other.to >= self.from && other.to <= self.to)
    }
}

struct AssignmentRow {
    first_elf_assignment: Assignment,
    second_elf_assignment: Assignment,
}

impl AssignmentRow {
    fn complete_overlap(&self) -> bool {
        self.first_elf_assignment
            .contains(&self.second_elf_assignment)
            || self
                .second_elf_assignment
                .contains(&self.first_elf_assignment)
    }

    fn overlap(&self) -> bool {
        self.first_elf_assignment
            .overlap(&self.second_elf_assignment)
    }
}

impl TryFrom<String> for AssignmentRow {
    fn try_from(value: String) -> Result<Self, String> {
        let Some(caps) = REGEX.captures(&value) else{
            return Err("Error while parsing".into());
        };

        Ok(AssignmentRow {
            first_elf_assignment: Assignment {
                from: caps["first_from"].parse().unwrap(),
                to: caps["first_to"].parse().unwrap(),
            },
            second_elf_assignment: Assignment {
                from: caps["second_from"].parse().unwrap(),
                to: caps["second_to"].parse().unwrap(),
            },
        })
    }

    type Error = String;
}

fn main() -> Result<(), std::io::Error> {
    let num_complete_overlap = get_input_file_handler()?
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| AssignmentRow::try_from(line).ok())
        .filter(|ass| ass.complete_overlap())
        .count();

    println!("Complete overlap: {}", num_complete_overlap);

    let num_overlap = get_input_file_handler()?
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| AssignmentRow::try_from(line).ok())
        .filter(|ass| ass.overlap())
        .count();

    println!("Overlaps: {}", num_overlap);

    Ok(())
}

fn get_input_file_handler() -> Result<BufReader<File>, std::io::Error> {
    let file_name = "./input.txt";
    let input_file = File::open(file_name)?;
    Ok(BufReader::new(input_file))
}
