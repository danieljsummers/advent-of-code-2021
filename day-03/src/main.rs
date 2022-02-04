use std::env;
use std::fs;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Pass 1 or 2 to determine part");
        exit(1);
    }

    let readings = fs::read_to_string("input.txt").expect("Could not open readings");

    if args[1] == "1" {
        println!("Fuel burn rate = {}", fuel_rate(readings));
    } else {
        println!("Life support rate = {}", life_support(readings));
    }
}

fn fuel_rate(readings: String) -> usize {
    let threshold = readings.lines().count() / 2;
    let mut bit_counts = [0usize; 12];
    for line in readings.lines() {
        let mut index = 0usize;
        for digit in line.chars() {
            bit_counts[index] += digit.to_string().parse::<usize>().unwrap();
            index += 1;
        }
    }

    let mut gamma = String::new();
    let mut epsilon = String::new();
    for sum in bit_counts {
        gamma.push(if sum > threshold { '1' } else { '0' });
        epsilon.push(if sum > threshold { '0' } else { '1' });
    }
    let gamma = usize::from_str_radix(&gamma, 2).unwrap();
    let epsilon = usize::from_str_radix(&epsilon, 2).unwrap();

    println!("Gamma = {}, Epsilon = {}", gamma, epsilon);
    gamma * epsilon
}

fn life_support(readings: String) -> usize {
    let mut o2 = readings
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>().try_into().unwrap())
        .collect::<Vec<[char; 12]>>();
    let mut co2 = readings
        .clone()
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>().try_into().unwrap())
        .collect::<Vec<[char; 12]>>();
    let mut pos = 0usize;

    while o2.len() > 1 {
        let digit = max_at_position(&o2, pos);
        o2 = o2
            .iter()
            .filter(|it| it[pos] == digit)
            .map(|it| *it)
            .collect();
        pos += 1;
    }

    pos = 0;
    while co2.len() > 1 {
        let digit = min_at_position(&co2, pos);
        co2 = co2
            .iter()
            .filter(|it| it[pos] == digit)
            .map(|it| *it)
            .collect();
        pos += 1;
    }

    let o2 = usize::from_str_radix(&chars_to_str(&o2[0]), 2).unwrap();
    let co2 = usize::from_str_radix(&chars_to_str(&co2[0]), 2).unwrap();

    println!("O2 Generator = {}, CO2 Scrubber = {}", o2, co2);
    o2 * co2
}

fn chars_to_str(chars: &[char; 12]) -> String {
    let mut to_string = String::new();
    for ch in chars {
        to_string.push(*ch);
    }
    to_string
}

fn to_digit(chars: &[char; 12], position: usize) -> usize {
    chars[position].to_string().parse::<usize>().unwrap()
}

fn make_stats(readings: &Vec<[char; 12]>, position: usize) -> (usize, usize) {
    let mut zeroes = 0usize;
    let mut ones = 0usize;
    for reading in readings {
        if to_digit(reading, position) == 0 {
            zeroes += 1;
        } else {
            ones += 1;
        }
    }
    (zeroes, ones)
}
fn max_at_position(readings: &Vec<[char; 12]>, position: usize) -> char {
    let (zeroes, ones) = make_stats(readings, position);
    match () {
        _ if zeroes == 0 => '1',
        _ if ones == 0 => '0',
        _ if ones >= zeroes => '1',
        _ => '0',
    }
}

fn min_at_position(readings: &Vec<[char; 12]>, position: usize) -> char {
    let (zeroes, ones) = make_stats(readings, position);
    match () {
        _ if zeroes == 0 => '1',
        _ if ones == 0 => '0',
        _ if ones >= zeroes => '0',
        _ => '1',
    }
}
