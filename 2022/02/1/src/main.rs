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

        let my_choice = match me {
            "X" => Shape::Rock,
            "Y" => Shape::Paper,
            "Z" => Shape::Scissors,
            _ => panic!("Invalid choice"),
        };

        let shape_score = match my_choice {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        };

        let outcome = match (my_choice, opponent_choice) {
            (Shape::Rock, Shape::Scissors)
            | (Shape::Paper, Shape::Rock)
            | (Shape::Scissors, Shape::Paper) => Outcome::Win,
            (m, o) if m == o => Outcome::Exact,
            _ => Outcome::Lose,
        };

        score += match outcome {
            Outcome::Win => shape_score + 6,
            Outcome::Lose => shape_score,
            Outcome::Exact => shape_score + 3,
        }
    }

    println!("Score: {score}");
}
