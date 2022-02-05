use std::env;
use std::fs;
use std::process::exit;

#[derive(Copy, Clone)]
struct BingoCard {
    rows: [[(i32, bool); 5]; 5],
    has_bingo: bool,
}
impl BingoCard {
    /// Create a new bingo card
    fn new(grid: &str) -> BingoCard {
        if grid.lines().count() != 5 {
            panic!("A bingo card is a 5x5 grid")
        }
        let mut rows = [[(0, false); 5]; 5];
        let mut row = 0usize;
        for line in grid.lines() {
            rows[row] = line
                .split_whitespace()
                .map(|it| (it.parse::<i32>().unwrap(), false))
                .collect::<Vec<(i32, bool)>>()
                .try_into()
                .unwrap();
            row += 1;
        }
        BingoCard {
            rows,
            has_bingo: false,
        }
    }
    /// Mark a call on the card; if the call results in bingo, update the property
    fn mark(&mut self, call: i32) {
        let mut marked = false;
        'marking: for row in &mut self.rows {
            for col in row {
                if col.0 == call {
                    *col = (call, true);
                    marked = true;
                    break 'marking;
                }
            }
        }
        if marked {
            // Look for horizontal bingo
            for row in self.rows {
                if row.iter().filter(|it| !it.1).count() == 0 {
                    self.has_bingo = true;
                    break;
                }
            }
            if !self.has_bingo {
                // Look for vertical bingo
                for col in 0..4 {
                    if self.rows[0][col].1
                        && self.rows[1][col].1
                        && self.rows[2][col].1
                        && self.rows[3][col].1
                        && self.rows[4][col].1
                    {
                        self.has_bingo = true;
                        break;
                    }
                }
            }
        }
    }
    /// Get the total value of the unmarked spots
    fn unmarked_total(&self) -> i32 {
        let mut total = 0;
        for row in self.rows {
            for col in row {
                if !col.1 {
                    total += col.0
                }
            }
        }
        total
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Pass 1 or 2 to determine part");
        exit(1);
    }

    let input = fs::read_to_string("input.txt").expect("Could not load input");

    let calls = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|it| it.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut boards = input
        .split("\n\n")
        .skip(1)
        .map(|it| BingoCard::new(it))
        .collect::<Vec<BingoCard>>();

    if args[1] == "1" {
        // First winning card
        let mut found_bingo = false;

        for call in calls {
            for board in &mut boards {
                board.mark(call);
                if board.has_bingo {
                    found_bingo = true;
                }
            }
            if found_bingo {
                let board_score = boards
                    .iter()
                    .filter(|board| board.has_bingo)
                    .map(|board| board.unmarked_total())
                    .max()
                    .unwrap();
                println!("The largest score is {}", board_score * call);
                break;
            }
        }
    } else {
        // Last winning card
        for call in calls {
            for board in &mut boards {
                board.mark(call);
            }
            let new_boards = boards
                .iter()
                .filter(|it| !it.has_bingo)
                .map(|it| *it)
                .collect::<Vec<BingoCard>>();
            if new_boards.len() == 0 {
                let board_score = boards.iter().map(|it| it.unmarked_total()).min().unwrap();
                println!("The minimum score is {}", board_score * call);
                break;
            }
            boards = new_boards;
        }
    }
}
