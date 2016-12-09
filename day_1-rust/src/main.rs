use std::str::FromStr;
use std::string::ParseError;
extern crate regex;
use regex::Regex;

#[derive(Debug)]
pub enum Direction {
    North,
    East,
    South,
    West
}
#[derive(Debug)]
pub struct Point {
    x: i32,
    y: i32
}

#[derive(Debug)]
pub enum Turn {
    L,
    R
}

#[derive(Debug)]
pub struct Instruction {
    turn: Turn,
    steps: i32
}

/// An error returned when parsing a `bool` from a string fails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseInstructionError { _priv: () }

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Instruction, ParseInstructionError> {
        let re = Regex::new(r"^([L|R])(\d+)$").unwrap();
        let cap = re.captures(s).unwrap();
        let turn = match cap.at(1) {
            Some("L") => Turn::L,
            Some("R") => Turn::R,
            _ => return Err(ParseInstructionError{ _priv: ()})
        };
        let steps = i32::from_str(cap.at(2).unwrap()).unwrap();
        Ok(Instruction{turn: turn, steps: steps})
    }
}

pub fn turn(input: &Instruction, face: &Direction) -> Direction {
    match input.turn {
        Turn::L =>
            match face {
                &Direction::North => Direction::West,
                &Direction::West => Direction::South,
                &Direction::South => Direction::East,
                &Direction::East => Direction::North
            },
        Turn::R =>
            match face {
                &Direction::North => Direction::East,
                &Direction::East => Direction::South,
                &Direction::South => Direction::West,
                &Direction::West => Direction::North,
            }
    }
}

fn main() {
    let mut face = Direction::North;
    let mut point = Point{x: 0, y: 0};

    let mut s = String::from("R3 L2 L2 R4 L1 R2 R3 R4 L2 R4 L2 L5 L1 R5 R2 R2 L1 R4 R1 L5 L3 R4 R3 R1 L1 \
       L5 L4 L2 R5 L3 L4 R3 R1 L3 R1 L3 R3 L4 R2 R5 L190 R2 L3 R47 R4 L3 R78 L1 R3 \
       R190 R4 L3 R4 R2 R5 R3 R4 R3 L1 L4 R3 L4 R1 L4 L5 R3 L3 L4 R1 R2 L4 L3 R3 R3 \
       L2 L5 R1 L4 L1 R5 L5 R1 R5 L4 R2 L2 R1 L5 L4 R4 R4 R3 R2 R3 L1 R4 R5 L2 L5 \
       L4 L1 R4 L4 R4 L4 R1 R5 L1 R1 L5 R5 R1 R1 L3 L1 R4 L1 L4 L4 L3 R1 R4 R1 R1 \
       R2 L5 L2 R4 L1 R3 L5 L2 R5 L4 R5 L5 R3 R4 L3 L3 L2 R2 L5 L5 R3 R4 R3 R4 R3 \
       R1");
    let mut lst = s.split_whitespace();
    let list_of_instructions = lst.map(Instruction::from_str);

    for instruction in list_of_instructions {
        let instruct = instruction.unwrap();
        face = turn(&instruct, &face);
        match face {
            Direction::North => point.y += instruct.steps,
            Direction::South => point.y -= instruct.steps,
            Direction::East => point.x += instruct.steps,
            Direction::West => point.x -= instruct.steps
        }
    }

    println!("{:?}", point.x.abs() + point.y.abs());
}
