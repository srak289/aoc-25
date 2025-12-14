use std::fs;
use std::io::{self, BufRead};

#[derive(Debug)]
struct FreshIngredientRange {
    start: u64,
    stop: u64,
}

impl FreshIngredientRange {
    fn new(start: u64, stop: u64) -> Self {
        Self { start, stop }
    }

    fn is_fresh(&self, id: u64) -> bool {
        id >= self.start && id <= self.stop
    }

    fn num_fresh(&self) -> u64 {
        // +1 because inclusive range
        self.stop - self.start + 1
    }

    fn union(&self, rhs: &Self) -> Self {
        let mut cs: u64 = 0;
        let mut ce: u64 = 0;
        if self.start <= rhs.start {
            cs = self.start;
        } else {
            cs = rhs.start;
        }
        if self.stop >= rhs.stop {
            ce = self.stop;
        } else {
            ce = rhs.stop;
        }
        Self { start: cs, stop: ce }
    }

    fn overlaps(&self, rhs: &Self) -> bool {
        (self.start <= rhs.stop && self.start >= rhs.start) || (self.stop >= rhs.start && self.stop <= rhs.stop) || (rhs.start <= self.stop && rhs.start >= self.start) || (rhs.stop >= self.start && rhs.stop <= self.stop)
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
                    let fi = first.parse::<u64>().expect("Failed to parse u64 from line");
                    let li = last.parse::<u64>().expect("Failed to parse u64 from line");
                    //if fi > li {
                    //    // no ranges are unordered
                    //    panic!("unordered range");
                    //}
                    ranges.push(FreshIngredientRange::new(fi, li));
                } else {
                    //println!("Adding ingredient {}", line);
                    ings.push(line.parse::<u64>().expect("Failed to parse u64 from line"));
                }
                line.clear();
            }
            Err(_) => panic!("Err"),
        }
    }

    // union the ranges until there are no more overlaps
    loop {
        let mut done = true;
        for i in 0..ranges.len() {
            for j in 0..ranges.len() {
                if i == j {
                    continue;
                }
                if ranges[i].overlaps(&ranges[j]) {
                    done = false;
                    let a = ranges.remove(i);
                    // -1 because we change the length with remove
                    // we can then safely use swap_remove because we only maintain order
                    // on the first remove
                    let b = ranges.swap_remove(j-1);
                    let c = a.union(&b);
                    println!("Union {}-{} from {}-{} and {}-{}", c.start, c.stop, a.start, a.stop, b.start, b.stop);
                    ranges.push(c);
                    // break inner loop if we change the size
                    break;
                }
            }
            if !done {
                // break outer loop if we change the size
                break;
            }
        }
        if done {
            break;
        }
    }

    for i in &ranges {
        println!("{:?}", i);
    }

    let mut fresh = Vec::<u64>::new();

    for i in ings {
        for r in &ranges {
            if r.is_fresh(i) {
                //println!("{} is fresh", i);
                fresh.push(i);
                break;
            }
        }
    }

    println!("There are {} fresh ing", fresh.len());
    assert_eq!(fresh.len(), 707);
    //too high - 477207365413044
    println!("There are {} fresh ids", ranges.into_iter().map(|x| x.num_fresh()).sum::<u64>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlap() {
        let a = FreshIngredientRange::new(0, 1);
        let b = FreshIngredientRange::new(1, 2);
        assert_eq!(a.overlaps(&b), true);
    }

    #[test]
    fn test_no_overlap() {
        let a = FreshIngredientRange::new(0, 0);
        let b = FreshIngredientRange::new(1, 1);
        assert_eq!(a.overlaps(&b), false);
    }

    #[test]
    fn test_fresh_ing() {
        let a = FreshIngredientRange::new(0, 0);
        assert_eq!(a.num_fresh(), 1);
    }

    #[test]
    fn test_big_overlaps_small() {
        let a = FreshIngredientRange::new(0, 3);
        let b = FreshIngredientRange::new(1, 2);
        assert_eq!(a.overlaps(&b), true);
    }

    #[test]
    fn test_small_overlaps_big() {
        let a = FreshIngredientRange::new(0, 3);
        let b = FreshIngredientRange::new(1, 2);
        assert_eq!(b.overlaps(&a), true);
    }

    #[test]
    fn test_enclosed_union() {
        let a = FreshIngredientRange::new(0, 3);
        let b = FreshIngredientRange::new(1, 2);
        let c = a.union(&b);
        let d = b.union(&a);
        assert_eq!(c.start, 0);
        assert_eq!(c.stop, 3);
        assert_eq!(d.start, 0);
        assert_eq!(d.stop, 3);
    }

    #[test]
    fn test_union() {
        let a = FreshIngredientRange::new(0, 1);
        let b = FreshIngredientRange::new(1, 2);
        let c = a.union(&b);
        let d = b.union(&a);
        assert_eq!(c.start, 0);
        assert_eq!(c.stop, 2);
        assert_eq!(d.start, 0);
        assert_eq!(d.stop, 2);
    }
}
