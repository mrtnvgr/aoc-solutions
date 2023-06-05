use std::fs;

#[derive(PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Lose,
    Exact,
}

fn main() {
    let list = fs::read_to_string("input.txt").unwrap();
    let mut score = 0;

    for game in list.lines().map(str::trim) {
        let (opponent, me) = game.split_once(' ').unwrap();

        let opponent_choice = match opponent {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => panic!("Invalid opponent choice"),
        };

        let needed_outcome = match me {
            "X" => Outcome::Lose,
            "Y" => Outcome::Exact,
            "Z" => Outcome::Win,
            _ => panic!("Invalid outcome"),
        };

        let my_choice = match (&needed_outcome, opponent_choice) {
            (Outcome::Exact, o) => o,
            (Outcome::Lose, o) => match o {
                Shape::Rock => Shape::Scissors,
                Shape::Paper => Shape::Rock,
                Shape::Scissors => Shape::Paper,
            },
            (Outcome::Win, o) => match o {
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissors,
                Shape::Scissors => Shape::Rock,
            },
        };

        let shape_score = match my_choice {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        };

        score += match needed_outcome {
            Outcome::Win => shape_score + 6,
            Outcome::Lose => shape_score,
            Outcome::Exact => shape_score + 3,
        }
    }

    println!("Score: {score}");
}
