use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::fs;

struct Move {
    count: usize,
    from: usize,
    to: usize,
}

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();

    let mut rows: Vec<Vec<String>> = Vec::new();

    let mut lines = data.lines();

    for line in &mut lines {
        if line.contains('[') {
            update_rows(&mut rows, line);
        } else {
            break;
        }
    }

    for row in &mut rows {
        row.reverse();
    }

    for line in &mut lines {
        if line.starts_with("move") {
            let info = parse_move_instruction(line);

            for _ in 0..info.count {
                let boxx = rows[info.from].pop().unwrap();
                rows[info.to].push(boxx);
            }
            // println!("{rows:?}");
        }
    }

    let mut answer: Vec<&str> = Vec::new();
    for row in &rows {
        answer.push(row.last().unwrap());
    }

    println!("{}", answer.join(""));
}

fn update_rows(rows: &mut Vec<Vec<String>>, line: &str) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([A-Z]|    )").unwrap();
    }

    for (row, letter) in RE.find_iter(line).map(|x| x.as_str().trim()).enumerate() {
        if rows.get(row).is_none() {
            rows.push(Vec::new());
        }

        if !letter.is_empty() {
            rows[row].push(letter.to_owned());
        }
    }
}

fn parse_move_instruction(line: &str) -> Move {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(?:move) (?P<count>\d+) (?:from) (?P<from>\d+) (?:to) (?P<to>\d+)")
                .unwrap();
    };

    let groups = RE.captures(line).unwrap();

    let get_group = |groups: &Captures, name: &str| groups.name(name).unwrap().as_str().to_owned();
    let parse_into_usize = |x: String| x.parse::<usize>().unwrap();

    let count = parse_into_usize(get_group(&groups, "count"));
    let from = parse_into_usize(get_group(&groups, "from")) - 1;
    let to = parse_into_usize(get_group(&groups, "to")) - 1;

    Move { count, from, to }
}
