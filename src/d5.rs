use std::fs;
use std::io::{self, BufRead};

// fresh ingredient id ranges inclusive
// blank line separator
// list of ids

struct FreshIngredientRange {
    start: u64,
    stop: u64,
}

impl FreshIngredientRange {
    fn new(start: u64, stop: u64) -> FreshIngredientRange {
        FreshIngredientRange { start, stop }
    }

    fn is_fresh(&self, id: u64) -> bool {
        id >= self.start && id <= self.stop
    }
}

pub fn run() {
    let mut reader = io::BufReader::new(fs::File::open("cafe.txt").expect("reading file failed"));
    let mut line = String::new();

    let mut ranges = Vec::<FreshIngredientRange>::new();
    let mut parse_ranges = true;
    let mut ings = Vec::<u64>::new();

    loop {
        match reader.read_line(&mut line) {
            Ok(x) => {
                if x == 0 {
                    break;
                }
                // remove newline
                let _ = line.pop();
                if line.len() == 0 {
                    // we are at the separator
                    //println!("Found separator");
                    parse_ranges = false;
                    continue;
                }
                if parse_ranges {
                    let idx = line.find('-').unwrap();
                    let (first, dashlast) = line.split_at(idx);
                    let last = dashlast.replace("-", "");
                    //println!("Creating ingredients from {} {}", first, last);
                    ranges.push(FreshIngredientRange::new(
                        first.parse::<u64>().expect("Failed to parse u64 from line"),
                        last.parse::<u64>().expect("Failed to parse u64 from line"),
                    ));
                } else {
                    //println!("Adding ingredient {}", line);
                    ings.push(line.parse::<u64>().expect("Failed to parse u64 from line"));
                }
                line.clear();
            }
            Err(_) => panic!("Err"),
        }
    }

    let mut fresh = Vec::<u64>::new();
    for i in ings {
        for r in &ranges {
            if r.is_fresh(i) {
                println!("{} is fresh", i);
                fresh.push(i);
                break;
            }
        }
    }
    // 293 - too low
    println!("There are {} fresh ing", fresh.len());
    println!("{:?}", fresh);
}
