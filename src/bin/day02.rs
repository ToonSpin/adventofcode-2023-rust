use std::cmp::max;
use std::io;
use std::io::prelude::*;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map, map_res};
use nom::multi::separated_list1;
use nom::sequence::{delimited, pair, terminated};
use nom::IResult;

type Sample = (u64, u64, u64);

#[derive(Debug)]
struct Game {
    game_id: u64,
    samples: Vec<Sample>,
}

impl Game {
    fn is_possible(&self) -> bool {
        let possible = |s: &Sample| s.0 <= 12 && s.1 <= 13 && s.2 <= 14;
        self.samples.iter().all(possible)
    }

    fn minimum_power(&self) -> u64 {
        let f = |acc: Sample, x: &Sample| (max(acc.0, x.0), max(acc.1, x.1), max(acc.2, x.2));
        let min_sample = self.samples.iter().fold((0, 0, 0), f);
        min_sample.0 * min_sample.1 * min_sample.2
    }
}

fn parse_number(input: &str) -> IResult<&str, u64> {
    map_res(digit1, |s: &str| s.parse::<u64>())(input)
}

fn parse_red(input: &str) -> IResult<&str, Sample> {
    let parser = terminated(parse_number, tag(" red"));
    map(parser, |n| (n, 0, 0))(input)
}

fn parse_green(input: &str) -> IResult<&str, Sample> {
    let parser = terminated(parse_number, tag(" green"));
    map(parser, |n| (0, n, 0))(input)
}

fn parse_blue(input: &str) -> IResult<&str, Sample> {
    let parser = terminated(parse_number, tag(" blue"));
    map(parser, |n| (0, 0, n))(input)
}

fn parse_sample(input: &str) -> IResult<&str, Sample> {
    let color_parser = alt((parse_red, parse_green, parse_blue));
    let mut parser = separated_list1(tag(", "), color_parser);
    let folder = |acc: Sample, x: &Sample| (acc.0 + x.0, acc.1 + x.1, acc.2 + x.2);
    let (rest, v) = parser(input)?;
    Ok((rest, v.iter().fold((0, 0, 0), folder)))
}

fn parse_game_header(input: &str) -> IResult<&str, u64> {
    delimited(tag("Game "), parse_number, tag(": "))(input)
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let parse_samples = separated_list1(tag("; "), parse_sample);
    let mut parser = pair(parse_game_header, parse_samples);
    let (rest, (game_id, samples)) = parser(input)?;
    Ok((rest, Game { game_id, samples }))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Game>> {
    separated_list1(tag("\n"), parse_game)(input)
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let (_, input) = parse_input(&input[..]).unwrap();

    let mut sum_part_one = 0;
    let mut sum_part_two = 0;
    for game in input.iter() {
        if game.is_possible() {
            sum_part_one += game.game_id;
        }
        sum_part_two += game.minimum_power();
    }

    println!("The product of IDs of possible games: {}", sum_part_one);
    println!("The product of minimum powers: {}", sum_part_two);

    Ok(())
}
