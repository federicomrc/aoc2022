use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(PartialEq, PartialOrd, Ord, Eq, Debug)]
enum GameMove {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for GameMove {
    fn from(value: &str) -> Self {
        if value == "A" || value == "X" {
            GameMove::Rock
        } else if value == "B" || value == "Y" {
            GameMove::Paper
        } else {
            GameMove::Scissors
        }
    }
}

fn compute_score(opponent_move: GameMove, my_move: GameMove) -> u32 {
    let base_score = match my_move {
        GameMove::Rock => 1,
        GameMove::Paper => 2,
        GameMove::Scissors => 3,
    };

    let win_score = if my_move == opponent_move {
        3
    } else if (my_move == GameMove::Rock && opponent_move == GameMove::Scissors)
        || (my_move == GameMove::Paper && opponent_move == GameMove::Rock)
        || (my_move == GameMove::Scissors && opponent_move == GameMove::Paper)
    {
        6
    } else {
        0
    };

    base_score + win_score
}

fn main() -> std::io::Result<()> {
    let input_file_name = "./input.txt";

    let file = File::open(input_file_name)?;
    let buf_reader = BufReader::new(file);

    let total_score = buf_reader
        .lines()
        .filter_map(|el| el.ok())
        .filter_map(|l| {
            let mut it = l.split_ascii_whitespace();

            let opponent = it.next();
            let mine = it.next();

            if mine.is_none() && opponent.is_none() {
                return None;
            }

            let opponent = GameMove::from(opponent.unwrap());
            let mine = GameMove::from(mine.unwrap());

            let score = compute_score(opponent, mine);

            Some(score)
        })
        .sum::<u32>();

    println!("First part: {total_score}");

    let file = File::open(input_file_name)?;
    let buf_reader = BufReader::new(file);
    let total_score = buf_reader
        .lines()
        .filter_map(|el| el.ok())
        .filter_map(|l| {
            let mut it = l.split_ascii_whitespace();

            let opponent = it.next();
            let mine = it.next();

            if mine.is_none() && opponent.is_none() {
                return None;
            }

            let opponent = GameMove::from(opponent.unwrap());
            let mine = match mine.unwrap() {
                "X" => match opponent {
                    GameMove::Rock => GameMove::Scissors,
                    GameMove::Paper => GameMove::Rock,
                    GameMove::Scissors => GameMove::Paper,
                },
                "Y" => match opponent {
                    GameMove::Rock => GameMove::Rock,
                    GameMove::Paper => GameMove::Paper,
                    GameMove::Scissors => GameMove::Scissors,
                },
                _ => match opponent {
                    GameMove::Rock => GameMove::Paper,
                    GameMove::Paper => GameMove::Scissors,
                    GameMove::Scissors => GameMove::Rock,
                },
            };

            let score = compute_score(opponent, mine);

            Some(score)
        })
        .sum::<u32>();

    println!("Second part: {total_score}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{compute_score, GameMove};

    #[test]
    fn basic_test_score() {
        let mut total = 0;

        let score = compute_score(GameMove::Rock, GameMove::Paper);
        assert_eq!(score, 8);
        total += score;

        let score = compute_score(GameMove::Paper, GameMove::Rock);
        assert_eq!(score, 1);
        total += score;

        let score = compute_score(GameMove::Scissors, GameMove::Scissors);
        assert_eq!(score, 6);
        total += score;

        assert_eq!(total, 15);
    }

    #[test]
    fn basic_test_from() {
        assert_eq!(GameMove::from("A"), GameMove::Rock);
        assert_eq!(GameMove::from("X"), GameMove::Rock);
        assert_eq!(GameMove::from("Y"), GameMove::Paper);
        assert_eq!(GameMove::from("B"), GameMove::Paper);
        assert_eq!(GameMove::from("Z"), GameMove::Scissors);
        assert_eq!(GameMove::from("C"), GameMove::Scissors);
    }
}
