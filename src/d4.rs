use std::fs;
use std::io::{self, BufRead};

pub fn run() {
    let mut reader = io::BufReader::new(fs::File::open("forklift_sample.txt").expect("reading file failed"));
    let mut line = String::new();

    let mut floor = Vec::<Vec<u8>>::new();

    loop {
        match reader.read_line(&mut line) {
            Ok(x) => {
                if x == 0 {
                    break;
                }
                let _ = line.pop(); // remove newline
                floor.push(line.as_bytes().to_vec());
                line.clear();
            }
            Err(_) => panic!("Err"),
        }
    }

    //println!("{:?}", floor);
    //for cell in floor, calculate the 8 adjacent positions (may be fewer than 8 if on an edge
    //check the valid cells for paper
    //count the number of paper
    //return whether that cell can be accessed
    //
    // 46 == b'.' empty
    // 64 == b'@' paper

    let mut total_paper = 0;

    let check = |x: usize, y: usize| -> bool {
        let mut paper = 0;

        let ys = if y == 0 { y } else { y - 1 };
        let ye = if y == floor.len()-1 { y } else { y + 1 };
        let xs = if x == 0 { x } else { x - 1 };
        let xe = if x == floor[y].len()-1 { x } else { x + 1 };

        //println!("Searching the area {xs}, {ys} - {xe}, {ye} for location {x}, {y}");

        for j in ys..=ye {
            for i in xs..=xe {
                if i == x && j == y {
                    //println!("Skipping {}", floor[j][i]);
                    continue;
                }
                //println!("Checking {}", floor[j][i]);
                if floor[j][i] == 64 {
                    paper += 1;
                }
            }
        }
        return paper < 4;
    };

    loop {
        let mut paper = 0;

        for y in 0..floor.len() {
            for x in 0..floor[y].len() {
                // don't check if it's not paper
                if floor[y][x] == 64 {
                    if check(x, y) {
                        println!("Removing paper at {}, {}", x, y);
                        floor[y][x] = 46;
                        paper += 1;
                    }
                }
            }
        }

        println!("Removed {} rolls", paper);
        if paper == 0 {
            println!("Cannot remove more rolls");
            break;
        }
        total_paper += paper;
        paper = 0;
    }
    println!("Finished: Removed {} rolls", total_paper);
}
