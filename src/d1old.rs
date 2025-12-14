use std::fs;
use std::io::{self, BufRead};

struct ComboLock {
    position: i16,
    rollover: i16,
}

impl ComboLock {
    fn new(x: i16) -> ComboLock {
        return ComboLock {
            position: x,
            rollover: 0,
        };
    }
    fn twist(&mut self, direction: char, distance: i16) {
        match direction {
            'L' => {
                self.rollover = if self.position - distance >= 0 {
                    0
                } else {
                    (self.position - distance).div_euclid(100).abs()
                };
                self.position = (self.position - distance).rem_euclid(100);
            }
            'R' => {
                self.rollover = (self.position + distance) / 100;
                self.position = (self.position + distance).rem_euclid(100);
            }
            _ => panic!("Unknown direction"),
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
    let mut ans = 0;
    for line in reader.lines() {
        let (dir, dist) = parse_line_noalloc(&line.unwrap()).unwrap();
        let start = c.position;
        println!("S: {}", c.position);
        println!("M: {}:{}", dir, dist);
        c.twist(dir, dist);
        println!("E: {}", c.position);
        if c.position == 0 {
            ans += 1;
            println!("Zero Stop: Adding 1 to password");
        } else if c.rollover > 0 {
            if start != 0 {
                ans += c.rollover;
                println!("Rollover: Adding {} to password", c.rollover);
            } else {
                ans += c.rollover - 1;
                println!("Rollover-1: Adding {} to password", c.rollover - 1);
            }
        }
        println!("Password is {}", ans);
    }
    println!("The password is {}", ans);
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

    #[test]
    fn test_no_rollover_left() {
        let mut c = ComboLock::new(99);
        c.twist('L', 1);
        assert_eq!(c.rollover, 0);
    }

    #[test]
    fn test_rollover_left() {
        let mut c = ComboLock::new(0);
        c.twist('L', 1);
        assert_eq!(c.rollover, 1);
    }

    #[test]
    fn test_no_rollover_right() {
        let mut c = ComboLock::new(0);
        c.twist('R', 1);
        assert_eq!(c.rollover, 0);
    }

    #[test]
    fn test_rollover_right() {
        let mut c = ComboLock::new(99);
        c.twist('R', 1);
        assert_eq!(c.rollover, 1);
    }

    #[test]
    fn test_rollover_1000() {
        let mut c = ComboLock::new(50);
        c.twist('R', 1000);
        assert_eq!(c.rollover, 10);
    }
}
