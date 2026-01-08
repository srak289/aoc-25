// allow all warnings for development
#![allow(warnings)]

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;

use regex::Regex;

#[derive(Debug, Clone)]
struct SimpleVector {
    arr: [u16; 10],
}

impl SimpleVector {
    fn new(arr: &[u16; 10]) -> Self {
        return Self { arr: arr.clone() };
    }

    fn add(&self, other: &SimpleVector) -> SimpleVector {
        let mut a = [0u16; 10];
        for i in 0..10 {
            a[i] = self.arr[i] + other.arr[i];
        }
        SimpleVector { arr: a }
    }

    fn sub(&self, other: &SimpleVector) -> SimpleVector {
        let mut a = [0u16; 10];
        for i in 0..10 {
            a[i] = self.arr[i] - other.arr[i];
        }
        SimpleVector { arr: a }
    }

    fn euclidean_dist(&self, other: &SimpleVector) -> f64 {
        let sum_of_squares: f64 = self.arr.iter()
            .zip(other.arr.iter())
            .map(|(&a, &b)| {
                let diff = (a as f64) - (b as f64);
                diff * diff
            })
            .sum();
        sum_of_squares.sqrt()
    }

    fn distance_from_target(&self, origin: &SimpleVector, target: &SimpleVector) -> f64 {
        return (origin.add(self)).euclidean_dist(target);
    }
}

impl From<[u16; 10]> for SimpleVector {
    fn from(item: [u16; 10]) -> SimpleVector {
        return SimpleVector { arr: item.clone() };
    }
}

impl From<&[u16; 10]> for SimpleVector {
    fn from(item: &[u16; 10]) -> SimpleVector {
        return SimpleVector { arr: item.clone() };
    }
}

#[derive(Debug)]
struct Machine {
    target: SimpleVector,
    jolts: SimpleVector,
    buttons: Vec<SimpleVector>,
}

impl Machine {
    fn new(jolts: &[u16; 10]) -> Self {
        Machine {
            jolts: SimpleVector::from([0u16; 10]),
            target: SimpleVector::from(jolts),
            buttons: Vec::<SimpleVector>::new(),
        }
    }

    fn overjolt(&self, button: usize) -> bool {
        for ((i, j), k) in zip(zip(self.target.arr, self.jolts.arr), self.buttons[button].arr) {
            if i == 0 {
                continue;
            }
            // println!("Trying {} < ({} + {})", i, j, k);
            if i < (j + k) {
                println!("Button {} is overjolt", button);
                return true;
            }
        }
        return false;
    }

    fn best(&mut self) {
        let mut bestidx: Option<usize> = None;
        let mut curval: Option<f64> = None;
        let mut bestval: Option<f64> = None;

        // println!("Starting with button {:?} {:?}", &self.buttons[0], bestval);
        for i in 0..self.buttons.len() {
            if self.overjolt(i) {
                // println!("{:?} is overjolt", self.buttons[i]);
                continue;
            }
            // println!("Trying button {:?}", &self.buttons[i]);
            curval = Some(self.buttons[i].distance_from_target(&self.jolts, &self.target));
            // println!("Found curval {}", curval.unwrap());
            match bestval {
                Some(x) => {
                    // println!("Try {} < {}", curval.unwrap(), x);
                    if curval.unwrap() < x {
                        bestidx = Some(i);
                        bestval = curval;
                        // println!("Found better button {} < {}", curval.unwrap(), x);
                    }
                }
                None => {
                    bestval = curval;
                    bestidx = Some(i);
                }
            }
        }

        match bestval {
            Some(x) => {
                // println!("Jolts is {:?}", &self.jolts);
                // println!("Adding button {:?} to jolts", &self.buttons[bestidx.unwrap()]);
                self.jolts = self.jolts.add(&self.buttons[bestidx.unwrap()]);
                // println!("Target is {:?}", &self.target);
                println!("Pushed button {}", bestidx.unwrap());
                println!("Jolts is now {:?}", &self.jolts);
            }
            None => {
                println!("Did not find a better button");
            }
        }
    }
}

fn main() {
    let mut reader = BufReader::new(File::open("factory_sample.txt").expect("reading file failed"));
    let mut line = String::new();

    let mut machines = Vec::<Machine>::new();

    let lre = Regex::new(r"\[[.#]+\]").expect("invalid regex");
    let bre = Regex::new(r"\(\d(,\d)*\)").expect("invalid regex");
    let jre = Regex::new(r"\{\d+(,\d+)*\}").expect("invalid regex");

    loop {
        match reader.read_line(&mut line) {
            Ok(bytes) => {
                if bytes == 0 {
                    break;
                }

                let lights = lre.find(&line);
                let buttons: Vec::<&str> = bre.find_iter(&line).map(|m| m.as_str()).collect();
                let jolts = jre.find(&line);

                let mut l: u16 = 0;
                let mut jv = [0u16; 10];

                for (i, j) in jolts.unwrap().as_str().strip_prefix('{').unwrap().strip_suffix('}').unwrap().split(',').enumerate() {
                    println!("{}", j);
                    jv[i] = j.parse::<u16>().unwrap();
                }
                // println!("Constructed jolt target {:?}", &jv);

                let mut m = Machine::new(&jv);

                for (i, bt) in buttons.into_iter().enumerate() {
                    let mut btn = [0u16; 10];

                    for c in bt.chars() {
                        match c {
                            '0'..='9' => btn[c.to_digit(10).unwrap() as usize] = 1,
                            _ => (),
                        }
                    }
                    // println!("Constructed button {:?} from {:?}", btn, bt);
                    m.buttons.push(SimpleVector::from(btn));
                }
                machines.push(m);
                line.clear();
            }
            Err(x) => panic!("{:?}", x),
        }
    }

    for _ in 0..10 {
        machines[0].best();
    }
    // for mut m in machines {
    //     // println!("{:?}", m);
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
}
