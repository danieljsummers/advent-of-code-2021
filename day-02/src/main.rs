use std::env;
use std::fs;
use std::process::exit;
use crate::Movement::*;

enum Movement {
    Forward(i32),
    Down(i32),
    Up(i32),
    Invalid,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Pass 1 or 2 to determine part");
        exit(1);
    }

    let movements = fs::read_to_string("input.txt").expect("Could not open movements");

    let mut pos = 0;
    let mut depth = 0;
    
    if args[1] == "1" {
        
        for line in movements.lines() {
            match parse_movement(line) {
                Forward(dist) => pos += dist,
                Down(dist) => depth += dist,
                Up(dist) => depth -= dist,
                Invalid => (),
            }
        }
    } else {
        let mut aim = 0;
        for line in movements.lines() {
            match parse_movement(line) {
                Forward(dist) => {
                    pos += dist;
                    depth += dist * aim;
                },
                Down(dist) => aim += dist,
                Up(dist) => aim -= dist,
                Invalid => (),
            }
        }
    }
    println!("Position x Depth = {}", pos  * depth);
}

fn parse_movement(line: &str) -> Movement {
    let mut parts = line.trim().split_whitespace();
    if let Some(direction) = parts.next() {
        if let Some(distance) = parts.next() {
            match distance.parse() {
                Ok(dist) => {
                    match direction {
                        "forward" => Forward(dist),
                        "down" => Down(dist),
                        "up" => Up(dist),
                        _ => Invalid
                    }
                },
                Err(_) => Invalid
            }
        } else {
            Invalid
        }
    } else {
        Invalid
    }
}
