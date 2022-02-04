use std::env;
use std::fs;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Pass 1 or 2 to determine part");
        exit(1);
    }

    let readings = fs::read_to_string("input.txt").expect("Could not open depth readings");

    let mut increased = 0;
    let mut prior = 0;

    if args[1] == "1" {
        for line in readings.lines() {
            let current = get_reading(line);
            if prior > 0 && current > prior {
                increased += 1;
            }
            prior = current;
        }
    } else {
        let mut window = (0, 0, 0);

        for line in readings.lines() {
            let (_, snd, thd) = window;
            window = (snd, thd, get_reading(line));
            if window.0 > 0 {
                let current = snd + thd + window.2;
                if prior > 0 && current > prior {
                    increased += 1;
                }
                prior = current;
            }
        }
    }

    println!("The reading increased {} times", increased);
}

fn get_reading(line: &str) -> i32 {
    match line.trim().parse() {
        Ok(reading) => reading,
        Err(err) => {
            println!("Error with depth reading: {}", err);
            0
        }
    }
}
