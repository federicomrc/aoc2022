use color_eyre::Report;
use tracing::info;
use tracing_subscriber::EnvFilter;

use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader, Read},
};

use once_cell::sync::Lazy;
use regex::Regex;

static REGEX_MOVES: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"move (?P<quantity>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap());

static REGEX_STACK: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?P<token>\[(?P<name>\S)\]|\s{3}) ?").unwrap());

#[derive(Debug)]
struct Move {
    quantity: u8,
    from: usize,
    to: usize,
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} | {} -> {}", self.quantity, self.from, self.to)
    }
}

impl TryFrom<String> for Move {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let Some(caps) = REGEX_MOVES.captures(&value) else {
            return Err("Connot find anything".into());
        };

        Ok(Move {
            quantity: caps["quantity"].parse().unwrap(),
            from: caps["from"].parse().unwrap(),
            to: caps["to"].parse().unwrap(),
        })
    }
}
#[derive(Debug)]
struct CargoStack {
    stacks: Vec<VecDeque<char>>,
}
// ┌────────────────────────────────────────────────────────────────────────┐
// │                                                                        │
// │                                                                        │
// │        [      ┌────────┐\w  ┌────────┐ ]  ┌────────┐                   │
// │      ┌───────►│ OPEN   ├───►│ NAME   ├───►│ CLOSE  ├──────────┐        │
// │      │        └────────┘    └────────┘    └────┬───┘          │        │
// │      │                                         │              │        │
// │      │                                         ▼              ▼        │
// │  ┌───┴────┐                               ┌────────┐     ┌────────┐    │
// └─►│ INIT   │                               │ END LN │     │ SPACE  ├────┘
//    └───┬────┘                               └────────┘     └────────┘
//        │                                         ▲              ▲
//        │                                         │              │
//        │        ┌────────┐    ┌────────┐    ┌────┴───┐          │
//        └───────►│ 1 SPAC ├───►│ 2 SPAC ├───►│ 3 SPAC ├──────────┘
//          ' '    └───┬────┘' ' └────────┘' ' └────────┘
//                     │                            ▲
//                     │         ┌────────┐         │
//                     └────────►│ NUMBER ├─────────┘
//                               └────────┘
#[derive(Debug, Copy, Clone)]
enum ParsingStates {
    Init,
    Open,
    ObjectName,
    Close,
    FSpace,
    SSpace,
    Number,
    TSpace,
}

impl TryFrom<String> for CargoStack {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut res_stack: Vec<VecDeque<char>> = Vec::new();
        let mut couting_first_line = true;

        for line in value.lines() {
            let mut index = 0;
            let mut current_state = ParsingStates::Init;

            for next_char in line.chars() {
                match current_state {
                    ParsingStates::Init => {
                        current_state = if next_char == '[' {
                            ParsingStates::Open
                        } else {
                            ParsingStates::FSpace
                        };
                    }
                    ParsingStates::Open => {
                        if couting_first_line {
                            res_stack.insert(index, VecDeque::new());
                        }

                        res_stack[index].push_front(next_char);
                        index += 1;
                        current_state = ParsingStates::ObjectName;
                    }
                    ParsingStates::ObjectName => current_state = ParsingStates::Close,
                    ParsingStates::Close | ParsingStates::TSpace => {
                        current_state = ParsingStates::Init
                    }
                    ParsingStates::FSpace => {
                        current_state = if next_char == ' ' {
                            ParsingStates::SSpace
                        } else {
                            ParsingStates::Number
                        }
                    }
                    ParsingStates::SSpace => {
                        if couting_first_line {
                            res_stack.insert(index, VecDeque::new());
                        }
                        index += 1;
                        current_state = ParsingStates::TSpace;
                    }
                    ParsingStates::Number => current_state = ParsingStates::TSpace,
                }
            }

            couting_first_line = false;
        }

        Ok(CargoStack { stacks: res_stack })
    }
}

impl std::fmt::Display for CargoStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, el) in self.stacks.iter().enumerate() {
            let mut comma_separated = String::new();

            for vector_element in el {
                comma_separated.push_str(&vector_element.to_string());
                comma_separated.push_str(", ");
            }

            writeln!(f, "{} | [{}]", idx, comma_separated)?;
        }

        Ok(())
    }
}

fn main() -> Result<(), Report> {
    setup()?;

    part_one();
    part_two();
    Ok(())
}

fn part_one() -> Result<(), Report> {
    let all_moves = get_moves_file_hanle()?
        .lines()
        .filter_map(|ln| ln.ok())
        .filter_map(|ln| Move::try_from(ln).ok());

    let mut buf = String::new();
    let _ = get_stack_file_handle()?.read_to_string(&mut buf)?;

    if let Ok(mut stack) = CargoStack::try_from(buf) {
        for mv in all_moves {
            //info!("{}", &mv);
            //info!("{}", stack);
            for cnt in 0..mv.quantity {
                if let Some(extracted) = stack.stacks[mv.from - 1].pop_back() {
                    stack.stacks[mv.to - 1].push_back(extracted);
                }
            }
        }
        println!("Part one");

        for idx in 0..stack.stacks.len() {
            println!("{:?}", stack.stacks[idx]);
        }
    }
    Ok(())
}

fn part_two() -> Result<(), Report> {
    let all_moves = get_moves_file_hanle()?
        .lines()
        .filter_map(|ln| ln.ok())
        .filter_map(|ln| Move::try_from(ln).ok());

    let mut buf = String::new();
    let _ = get_stack_file_handle()?.read_to_string(&mut buf)?;

    if let Ok(mut stack) = CargoStack::try_from(buf) {
        for mv in all_moves {
            //info!("{}", &mv);
            //info!("{}", stack);
            let mut tmp: VecDeque<char> = VecDeque::new();
            for cnt in 0..mv.quantity {
                if let Some(extracted) = stack.stacks[mv.from - 1].pop_back() {
                    tmp.push_back(extracted);
                }
            }

            for cnt in 0..mv.quantity {
                if let Some(extracted) = tmp.pop_back() {
                    stack.stacks[mv.to - 1].push_back(extracted);
                }
            }
        }

        println!("Part two");
        for idx in 0..stack.stacks.len() {
            println!("{:?}", stack.stacks[idx]);
        }
    }
    Ok(())
}

fn get_stack_file_handle() -> Result<BufReader<File>, std::io::Error> {
    let file_name = "./stack.txt";
    let input_file = File::open(file_name)?;
    Ok(BufReader::new(input_file))
}

fn get_moves_file_hanle() -> Result<BufReader<File>, std::io::Error> {
    let file_name = "./moves.txt";
    let input_file = File::open(file_name)?;
    Ok(BufReader::new(input_file))
}

fn setup() -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}
