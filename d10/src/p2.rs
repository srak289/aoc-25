use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;
use regex::Regex;

#[derive(Debug)]
struct Machine {
    jolts: Vec<u16>,
    buttons: Vec<u16>,
}

impl Machine {
    fn new() -> Self {
        Machine {
            jolts: Vec::<u16>::new(),
            buttons: Vec::<u16>::new(),
        }
    }

    fn do_bfs(&self) -> usize {
        fn overjolt(current: &Vec<u16>, target: &Vec<u16>) -> bool {
            for (i, j) in zip(current, target) {
                if i > j {
                    return true;
                }
            }
            return false;
        }

        let mut map: HashSet<Vec<u16>> = HashSet::new();
        // we only need to check whether the node is already in the set
        // and we carry the rest in the frontier
        let mut frontier: HashSet<Vec<u16>> = HashSet::new();
        let mut v = Vec::new();
        for _ in 0..self.jolts.len() {
            v.push(0);
        }
        frontier.insert(v);

        let mut done = false;
        let mut depth = 1;

        // if any jolt is too high we stop exploring that path
        loop {
            let mut newfront: HashSet<Vec<u16>> = HashSet::new();

            for f in &frontier {
                println!("Checking key {:?}", f);
                if !map.contains(f) {
                    // new node
                    let mut edges: HashSet<Vec<u16>> = HashSet::new();
                    if overjolt(f, &self.jolts) {
                        println!("{:?} is overjolt", f);
                        map.insert(f.clone());
                        continue;
                    }
                    for bdx in 0..self.buttons.len() {
                        let mut ff = f.clone();
                        for i in 0..ff.len() {
                            println!("Adding {} at pos {} to {:?}", (self.buttons[bdx] & 2_u16.pow(i.try_into().unwrap())), bdx, ff);
                            ff[i] += self.buttons[bdx] & 2_u16.pow(i.try_into().unwrap());
                        }
                        println!("Building edge {:?}", ff);
                        edges.insert(ff.clone());
                        newfront.insert(ff.clone());
                    }
                    if edges.clone().into_iter().any(|x| x == self.jolts) {
                        println!("Found start! Depth: {}", depth);
                        done = true;
                        break;
                    }
                    map.insert(f.clone());
                } else {
                    println!("{:?} already visited", f);
                }
            }
            if done {
                println!("Done!");
                break;
            }
            if newfront.len() == 0 {
                panic!("No frontier!");
            }
            depth += 1;
            frontier = newfront.clone();
            newfront.clear();
        }
        return depth;
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
                let buttons: Vec::<_> = bre.find_iter(&line).map(|m| m.as_str()).collect();
                let jolts = jre.find(&line);
                let mut m = Machine::new();
                let mut l: u16 = 0;
                for bt in buttons {
                    let mut btn: u16 = 0;
                    for c in bt.chars() {
                        match c {
                            '0'..='9' => btn |= 2_u16.pow(c.to_digit(10).unwrap()),
                            _ => (),
                        }
                    }
                    println!("Constructed button {} from {}", btn, bt);
                    m.buttons.push(btn);
                }
                for j in jolts.unwrap().as_str().strip_prefix('{').unwrap().strip_suffix('}').unwrap().split(',') {
                    println!("{}", j);
                    m.jolts.push(j.parse::<u16>().unwrap());
                }
                machines.push(m);
                line.clear();
            }
            Err(x) => panic!("{:?}", x),
        }
    }

    let mut depths: Vec<usize> = Vec::new();

    for mut m in machines {
        println!("{:?}", m);
        let d = m.do_bfs();
        depths.push(d);
        println!("Machine depth {}", d);
    }
    println!("{}", depths.into_iter().sum::<usize>());
}

#[cfg(test)]
mod tests {
    use super::*;
}
