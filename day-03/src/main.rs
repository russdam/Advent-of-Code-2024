#![allow(unused_variables)]

use nom::{branch::alt, bytes::complete::tag, character::complete::u32, combinator::map};

fn main() {
    let exampledata_1 = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    let data_1 = include_str!("../data/data-1.in");
    let exampledata_2 = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    let data_2 = include_str!("../data/data-2.in");
    part_one(data_1);
    part_two(data_1);
}
#[derive(Debug)]
struct Mul {
    a: u32,
    b: u32,
}
use nom::*;
fn mul(input: &str) -> IResult<&str, Mul> {
    let (input, _) = tag("mul(")(input)?;
    let (input, a) = u32(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, b) = u32(input)?;
    let (input, _) = tag(")")(input)?;
    Ok((input, Mul { a, b }))
}

enum Keywords {
    Do(()),
    Dont(()),
    Mul(Mul),
}

fn dont(input: &str) -> IResult<&str, ()> {
    let (input, _) = tag("don't()")(input)?;
    Ok((input, ()))
}

fn r#do(input: &str) -> IResult<&str, ()> {
    let (input, _) = tag("do()")(input)?;
    Ok((input, ()))
}

fn read_next(input: &str) -> IResult<&str, Keywords> {
    alt((
        map(r#do, Keywords::Do),
        map(dont, Keywords::Dont),
        map(mul, Keywords::Mul),
    ))(input)
}

fn part_one(data: &str) {
    let mut input = data;
    let mut sum = 0u32;
    while !input.is_empty() {
        input = match mul(input) {
            Ok(r) => {
                sum += r.1.a * r.1.b;
                &input[1..]
            }
            Result::Err(_) => &input[1..],
        }
    }
    println!("Sum total is {sum}");
}
fn part_two(data: &str) {
    let mut enabled = true;
    let mut input = data;
    let mut sum = 0u32;

    while !input.is_empty() {
        input = match read_next(input) {
            Ok(kw) => {
                match kw.1 {
                    Keywords::Do(_) => enabled = true,
                    Keywords::Dont(_) => enabled = false,
                    Keywords::Mul(mul) => {
                        if enabled {
                            sum += mul.a * mul.b
                        };
                    }
                }
                &input[1..]
            }
            Result::Err(_) => &input[1..],
        }
    }
    println!("Sum total is {sum}");
}
