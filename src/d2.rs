use std::fs;
use std::io::{self, BufRead};

// we know clone is not fast
#[derive(Debug, Clone)]
struct VecInt {
    // A structure that represents an integer string
    // as a vector of characters
    // In order to store a u64 as a decimal set of characters we need
    // 20 positions + 1 for string terminator read from buf unless we decide
    // to omit the terminator
    buf: Vec<u8>,
}

impl VecInt {
    fn _increment(&mut self, i: usize) {
        //println!("In self_increment");
        match self.buf[i] {
            // 48 == b'0'
            // 57 == b'9'
            48 ..= 56 => {
                self.buf[i] += 1;
                //println!("Adding 1");
            }
            57 => {
                self.buf[i] = 48;
                if i == 0 {
                    self.buf.insert(0, b'0');
                    self._increment(i);
                } else {
                    self._increment(i - 1);
                }
            }
            _ => {
                panic!("Unexpected");
            }
        }
    }

    fn increment(&mut self) {
        self._increment(self.buf.len() - 1);
    }

    #[allow(dead_code)]
    fn invalid_one(&self) -> bool {
        // part one
        match self.buf.len() % 2 {
            0 => {
                let (f, l) = self.buf.split_at(self.buf.len() / 2);
                f == l
            }
            1 => false,
            _ => panic!("That was unexpected"),
        }
    }


    fn invalid_two(&self) -> bool {
        // invalid if any sequence of digits repeats at least twice
        // e.g. 111 => '1' x3
        // 1212 => '12' x2
        // 12341234 => '1234' x2

        // iterate whole buf to check pat match
        // pat length must be multiple of string len so we can 
        // short-circuit if that check does not pass
        // e.g. a string of length 6 could have a pattern of 1, 2, or 3
        // a string of length 7 can only have a pattern of one

        if self.buf.len() == 1 {
            return false;
        }

        fn factor(n: usize) -> Vec<usize> {
            // we don't do any fancy factoring because
            // we're only dealing with numbers up to 20
            let mut f = Vec::<usize>::new();
            f.push(1);
            for i in 2..n {
                if n % i == 0 {
                    f.push(i);
                }
            }
            return f;
        }

        let check = |n: usize| -> bool {
            //println!("Checking {}", n);
            let pat = &self.buf[0..n];
            let mut i = n;
            //println!("Starting at {}", n);
            while i <= self.buf.len() - n {
                if self.buf[i..i+n] != *pat {
                    //println!("FAILED PATTERN MATCH");
                    return false;
                }
                i += n;
            }
            //println!("SUCCESSFUL PATTERN MATCH");
            return true;
        };

        let nf = factor(self.buf.len());
        //println!("Factors of {}: {:?}", self.buf.len(), nf);
        nf.into_iter().map(|x| check(x)).any(|x| x == true)
    }
}

impl From<&[u8]> for VecInt {
    fn from(item: &[u8]) -> VecInt {
        let mut buf = Vec::<u8>::with_capacity(20);
        let mut iter = item.into_iter().enumerate();
        while let Some((i, c)) = iter.next() {
            buf[i] = *c;
        }
        VecInt { buf: buf }
    }
}

impl From<Vec<u8>> for VecInt {
    fn from(item: Vec<u8>) -> VecInt {
        let mut buf = item.clone();
        if buf[buf.len() - 1] == b'\0' {
            let _ = buf.pop();
        }
        while buf.len() < 20 {
            buf.insert(0, 0);
        }
        VecInt { buf: item }
    }
}

impl Into<u64> for VecInt {
    fn into(self) -> u64 {
        // a u64 may have up to 20 decimal characters
        // plus one for string terminator
        //println!("Try to parse {:?}", self.buf);
        str::from_utf8(&self.buf).unwrap().parse::<u64>().unwrap()
    }
}

impl PartialEq for VecInt {
    fn eq(&self, other: &Self) -> bool {
        self.buf == other.buf
    }
}

impl Eq for VecInt {}

#[derive(Debug, Clone)]
struct VecIntRange {
    // A range of struct::VecInt that implements iterator
    start: VecInt,
    stop: VecInt,
}

impl VecIntRange {
    fn new(start: Vec<u8>, stop: Vec<u8>) -> VecIntRange {
        VecIntRange { start: start.into(), stop: stop.into() }
    }
}

// invalid
// any double-sequence is invalid
// e.g. 55 or 6464
// anywhere within the range separated by -
// sum the invalid ids to get password
//
// 1) read file into ranges based on - and ,
// 2) generate strings within ranges
// 3) check strings for dupes
// for strings n % 2 == 0
// the pattern is the 0..n.len()/2
//
// we need u64 to hold large integers, our file exceeds u32 integer space
//
// strings n % 2 == 1 are always valid because you cannot repeat a pattern TWO TIMES
// in an odd-length string

pub fn run() {
    let mut reader = io::BufReader::new(fs::File::open("gift_shop.txt").expect("reading file failed"));
    let mut buf = Vec::<u8>::new();
    let _ = reader.read_until(b'\n', &mut buf);

    let mut capfirst = true;
    let mut start = Vec::<u8>::new();
    let mut stop = Vec::<u8>::new();
    let mut inventory = Vec::<VecIntRange>::new();

    for c in buf.drain(..) {
        //println!("Read char {}", c);
        match c {
            b'-' => {
                capfirst = false;
                //println!("Read record separator");
            }
            b',' | b'\0' | b'\n' => {
                capfirst = true;
                //println!("Read range separator or EOF");
                let vir = VecIntRange::new(start.clone(), stop.clone());
                inventory.push(vir.clone());
                //println!("Constructed new range {}-{}",
                //    Into::<u64>::into(vir.start),
                //    Into::<u64>::into(vir.stop)
                //);
                start.clear();
                stop.clear();
            }
            _ => {
                if capfirst {
                    start.push(c);
                } else {
                    stop.push(c);
                }
            }
        };
    }

    let mut ans: u64 = 0;

    for mut ir in inventory {
        //println!("Working on {:?}", ir.start);
        let mut start = <VecInt as Into<u64>>::into(ir.start.clone()); 
        let stop = <VecInt as Into<u64>>::into(ir.stop.clone()); 
        //println!("Start: {} Stop: {}", start, stop);
        while start <= stop {
            match ir.start.invalid_two() {
                true => {
                    println!("{} is invalid", start);
                    ans += start;
                }
                false => (),
            }
            ir.start.increment();
            start = <VecInt as Into<u64>>::into(ir.start.clone()); 
        }
    }

    println!("The password is {}", ans);
    //assert_eq!(ans, 4174379265);
    // 73694270733 -- too high
    // 4174379265 -- sample
    // 73694270688

}

#[cfg(test)]
mod tests {
    use super::*;
}
