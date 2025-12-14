use std::fs;
use std::io::{self, BufRead};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Battery {
    cells: Vec<u8>,
    //jolts: u8,
    jolts: u64,
    tested: bool,
}

impl From<Vec<u8>> for Battery {
    fn from(item: Vec<u8>) -> Battery {
        Battery {
            cells: item,
            jolts: 0,
            tested: false,
        }
    }
}

impl Battery {
    // part one
    //fn test(&mut self) -> u8 {
    //    // set and return the jolts of this battery
    //    // the two cells with the highest concatenated value
    //    // e.g. "12345" => 2,4 => 24 jolts
    //    // but the highest joltage for the battery would be 45
    //    // find the largest number in let i = 0..len-2 (leave one)
    //    // find the largest number in i..len-1
    //    // we assume all the values in the cells are 48-57
    //    // if there are two occurrences of the same high number we'll want the first one
    //
    //    let mut tens_idx: usize = 0;
    //    let mut tens: u8 = 0;
    //    //let mut ones_idx: usize = 0;
    //    let mut ones: u8 = 0;
    //
    //    for i in 0..self.cells.len()-1 {
    //        if self.cells[i] > tens {
    //            tens_idx = i;
    //            tens = self.cells[i]
    //        }
    //    }
    //
    //    for i in tens_idx+1..self.cells.len() {
    //        if self.cells[i] > ones {
    //            //ones_idx = i;
    //            ones = self.cells[i]
    //        }
    //    }
    //
    //    let j = Vec::from([tens, ones]);
    //    let s = String::from_utf8(j).unwrap();
    //    self.jolts = s.parse::<u8>().unwrap();
    //    self.tested = true;
    //    return self.jolts;
    //}
    fn test(&mut self) -> u64 {
        //need to loop and find the largest number after the last position of the largest number of each loop
        //    so the max search space shrinks with each digit

        let mut jolts = Vec::<u8>::new();
        let mut idx: usize = 0;
        let mut ix: usize = 0;
        let mut huge: u8 = 48; // code for b'0'
        // we find the 12 largest cells
        for i in 0..12 {
            // we don't search to the end because we need room for the following cells
            for j in idx+ix..self.cells.len()-(11-i) {
                if self.cells[j] > huge {
                    huge = self.cells[j];
                    idx = j;
                }
            }
            jolts.push(huge);
            huge = 48;
            ix = 1;
        }
        println!("{:?}", jolts);
        self.jolts = String::from_utf8(jolts).unwrap().parse::<u64>().unwrap();
        self.tested = true;
        return self.jolts;
    }
}

pub fn run() {
    let mut reader = io::BufReader::new(fs::File::open("battery.txt").expect("reading file failed"));
    let mut line = String::new();

    let mut inventory = Vec::<Battery>::new();

    loop {
        match reader.read_line(&mut line) {
            Ok(x) => {
                if x == 0 {
                    break;
                }
                let _ = line.pop(); // remove newline
                inventory.push(line.as_bytes().to_vec().into());
                line.clear();
            }
            Err(_) => panic!("Err"),
        }
    }

    let mut jolts: u64 = 0;

    for mut b in inventory {
        let j = b.test();
        println!("{:?}", b);
        jolts += j;
    }
    println!("Bank capacity: {}", jolts);
    // 171419245422055

}

#[cfg(test)]
mod tests {
    use super::*;
}
