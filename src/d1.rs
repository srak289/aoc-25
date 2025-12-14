use std::fs;
use std::io::{self, BufRead};

struct ComboLock {
    position: i16,
    points: u16,
}

impl ComboLock {
    fn new(x: i16) -> ComboLock {
        return ComboLock {
            position: x,
            points: 0,
        };
    }
    fn twist(&mut self, direction: char, distance: i16) {
        match direction {
            'L' => {
                for _ in 0..distance {
                    self.position -= 1;
                    if self.position == -1 {
                        println!("ROLLOVER-LEFT");
                        self.position = 99;
                    }
                    if self.position == 0 {
                        println!("ZERO");
                        self.points += 1;
                    }
                }
            }
            'R' => {
                for _ in 0..distance {
                    self.position += 1;
                    if self.position == 100 {
                        println!("ROLLOVER-RIGHT");
                        self.position = 0;
                    }
                    if self.position == 0 {
                        println!("ZERO");
                        self.points += 1;
                    }
                }
            }
            _ => panic!("Direction must be L or R"),
        };
    }
}

fn parse_line_noalloc(line: &str) -> Option<(char, i16)> {
    let line = line.trim();
    let mut iter = line.char_indices();
    let (_, first_char) = iter.next()?;
    let (next_char_idx, _) = iter.next().unwrap_or((line.len(), '\0'));
    let rest = &line[next_char_idx..];
    Some((first_char, rest.parse::<i16>().ok()?))
}

pub fn run() {
    let mut c = ComboLock::new(50);
    let fp = fs::File::open("day1_input.txt").unwrap();
    //let fp = fs::File::open("test.txt").unwrap();
    let reader = io::BufReader::new(fp);
    for line in reader.lines() {
        let (dir, dist) = parse_line_noalloc(&line.unwrap()).unwrap();
        c.twist(dir, dist);
        println!("Password is {}", c.points);
    }
    println!("The password is {}", c.points);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position() {
        let mut c = ComboLock::new(0);
        c.twist('L', 0);
        assert_eq!(c.position, 0);
        c.twist('R', 0);
        assert_eq!(c.position, 0);
        c.twist('R', 12);
        assert_eq!(c.position, 12);
        c.twist('L', 12);
        assert_eq!(c.position, 0);
        c.twist('R', 100);
        assert_eq!(c.position, 0);
        c.twist('R', 112);
        assert_eq!(c.position, 12);
        c.twist('L', 13);
        assert_eq!(c.position, 99);
        c.twist('R', 1);
        assert_eq!(c.position, 0);
    }
}
