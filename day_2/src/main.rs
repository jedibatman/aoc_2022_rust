use std::fs;

use itertools::Itertools;

#[derive(Clone, Copy)]
enum MatchResult {
    Unknown,
    Draw,
    Loss,
    Win,
}

#[derive(Clone, Copy)]
enum MatchMove {
    None,
    Rock,
    Paper,
    Scissors,
}

fn main() {
    let contents = fs::read_to_string("src/input.txt")
        .expect("input file to be read");

    let total_score_part_1 =
        contents.lines()
        .map(|s| s.split_whitespace())
        .fold(0, |acc, mut moves_iter| {
            let (their_move, my_move) =
                moves_iter.next_tuple()
                .map(|(a, b)| (translate_move(a), translate_move(b)))
                .unwrap();

            let result: MatchResult = get_match_result(my_move, their_move);

            let mut score = get_move_points(my_move);

            score += get_result_points(result);

            return acc + score;
        });

    let total_score_part_2 =
        contents.lines()
        .map(|s| s.split_whitespace())
        .fold(0, |acc, mut moves_iter| {
            let (their_move, desired_match_result) =
                moves_iter.next_tuple()
                .map(|(a, b)| (translate_move(a), translate_match_result(b)))
                .unwrap();

            let desired_move: MatchMove = get_desired_move(their_move, desired_match_result);

            let mut score = get_move_points(desired_move);

            score += get_result_points(desired_match_result);

            return acc + score;
        });


    println!("total score is {total_score_part_1} (part 1)");
    println!("total score is {total_score_part_2} (part 2)");
}

fn get_match_result(my_move: MatchMove, their_move: MatchMove) -> MatchResult {
    match (my_move, their_move) {
        (MatchMove::Rock, MatchMove::Rock) => MatchResult::Draw,
        (MatchMove::Rock, MatchMove::Paper) => MatchResult::Loss,
        (MatchMove::Rock, MatchMove::Scissors) => MatchResult::Win,
        (MatchMove::Paper, MatchMove::Rock) => MatchResult::Win,
        (MatchMove::Paper, MatchMove::Paper) => MatchResult::Draw,
        (MatchMove::Paper, MatchMove::Scissors) => MatchResult::Loss,
        (MatchMove::Scissors, MatchMove::Rock) => MatchResult::Loss,
        (MatchMove::Scissors, MatchMove::Paper) => MatchResult::Win,
        (MatchMove::Scissors, MatchMove::Scissors) => MatchResult::Draw,
        _ => MatchResult::Unknown,
    }
}

fn get_desired_move(their_move: MatchMove, desired_result: MatchResult) -> MatchMove {
    match (their_move, desired_result) {
        (MatchMove::Rock, MatchResult::Draw) => MatchMove::Rock,
        (MatchMove::Rock, MatchResult::Loss) => MatchMove::Scissors,
        (MatchMove::Rock, MatchResult::Win) => MatchMove::Paper,
        (MatchMove::Paper, MatchResult::Draw) => MatchMove::Paper,
        (MatchMove::Paper, MatchResult::Loss) => MatchMove::Rock,
        (MatchMove::Paper, MatchResult::Win) => MatchMove::Scissors,
        (MatchMove::Scissors, MatchResult::Draw) => MatchMove::Scissors,
        (MatchMove::Scissors, MatchResult::Loss) => MatchMove::Paper,
        (MatchMove::Scissors, MatchResult::Win) => MatchMove::Rock,
        _ => MatchMove::None,
    }
}

fn translate_move(m: &str) -> MatchMove {
    match m {
        "A" => MatchMove::Rock,
        "B" => MatchMove::Paper,
        "C" => MatchMove::Scissors,
        "X" => MatchMove::Rock,
        "Y" => MatchMove::Paper,
        "Z" => MatchMove::Scissors,
        _ => MatchMove::None,
    }
}

fn translate_match_result(m: &str) -> MatchResult {
    match m {
        "X" => MatchResult::Loss,
        "Y" => MatchResult::Draw,
        "Z" => MatchResult::Win,
        _ => MatchResult::Unknown,
    }
}

fn get_move_points(m: MatchMove) -> i32 {
    match m {
        MatchMove::Rock => 1,
        MatchMove::Paper => 2,
        MatchMove::Scissors => 3,
        _ => 0,
    }
}

fn get_result_points(result: MatchResult) -> i32 {
    match result {
        MatchResult::Win => 6,
        MatchResult::Draw => 3,
        _ => 0,
    }
}
