use std::io;
use std::io::prelude::*;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::one_of;
use nom::combinator::{map, value};
use nom::IResult;

fn parse_zero(input: &str) -> IResult<&str, u64> {
    value(0, tag("zero"))(input)
}

fn parse_one(input: &str) -> IResult<&str, u64> {
    value(1, tag("one"))(input)
}

fn parse_two(input: &str) -> IResult<&str, u64> {
    value(2, tag("two"))(input)
}

fn parse_three(input: &str) -> IResult<&str, u64> {
    value(3, tag("three"))(input)
}

fn parse_four(input: &str) -> IResult<&str, u64> {
    value(4, tag("four"))(input)
}

fn parse_five(input: &str) -> IResult<&str, u64> {
    value(5, tag("five"))(input)
}

fn parse_six(input: &str) -> IResult<&str, u64> {
    value(6, tag("six"))(input)
}

fn parse_seven(input: &str) -> IResult<&str, u64> {
    value(7, tag("seven"))(input)
}

fn parse_eight(input: &str) -> IResult<&str, u64> {
    value(8, tag("eight"))(input)
}

fn parse_nine(input: &str) -> IResult<&str, u64> {
    value(9, tag("nine"))(input)
}

fn parse_digit_english(input: &str) -> IResult<&str, u64> {
    alt((
        parse_zero,
        parse_one,
        parse_two,
        parse_three,
        parse_four,
        parse_five,
        parse_six,
        parse_seven,
        parse_eight,
        parse_nine,
    ))(input)
}

fn parse_digit_part_one(input: &str) -> IResult<&str, u64> {
    map(one_of("0123456789"), |c| ((c as u8) - b'0') as u64)(input)
}

fn parse_digit_part_two(input: &str) -> IResult<&str, u64> {
    alt((parse_digit_part_one, parse_digit_english))(input)
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut total_part_one = 0;
    let mut total_part_two = 0;

    for line in input.lines() {
        let mut first_part_one = None;
        let mut first_part_two = None;
        let mut last_part_one = 0;
        let mut last_part_two = 0;

        for i in 0..line.len() {
            if let Ok((_, d)) = parse_digit_part_one(&line[i..]) {
                if let None = first_part_one {
                    first_part_one = Some(d);
                }
                last_part_one = d;
            }
            if let Ok((_, d)) = parse_digit_part_two(&line[i..]) {
                if let None = first_part_two {
                    first_part_two = Some(d);
                }
                last_part_two = d;
            }
        }
        total_part_one += (10 * first_part_one.unwrap() + last_part_one) as u64;
        total_part_two += (10 * first_part_two.unwrap() + last_part_two) as u64;
    }

    println!("The sum of the calibration values: {}", total_part_one);
    println!("The sum including spelled out digits: {}", total_part_two);

    Ok(())
}
