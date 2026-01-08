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

        let mut map: HashSet<Vec<u16>> = HashSet::with_capacity(1024);
        // we only need to check whether the node is already in the set
        // and we carry the rest in the frontier
        let mut frontier: HashSet<Vec<u16>> = HashSet::with_capacity(256);
        let mut v = Vec::with_capacity(self.jolts.len());
        for _ in 0..self.jolts.len() {
            v.push(0);
        }
        frontier.insert(v);

        let mut newfront: HashSet<Vec<u16>> = HashSet::with_capacity(256);
        let mut edges: HashSet<Vec<u16>> = HashSet::with_capacity(256);

        let mut done = false;
        let mut depth = 1;

        let mut expb: Vec<Vec<u16>> = Vec::with_capacity(self.buttons.len());
        let mut btn: Vec<u16> = Vec::with_capacity(self.jolts.len());

        for bdx in 0..self.buttons.len() {
            for i in 0..self.jolts.len() {
                let j = (self.buttons[bdx] & 2_u16.pow(i.try_into().unwrap())) >> i;
                // println!("Adding {} at pos {} to {:?}", j, i, ff);
                btn.push(j);
            }
            expb.push(btn.clone());
            btn.clear();
        }
        println!("Constructed buttons {:?}", expb);

        loop {

            for f in &frontier {
                // println!("Checking key {:?}", f);
                if !map.contains(f) {
                    // new node
                    if overjolt(f, &self.jolts) {
                        // if any jolt is too high we stop exploring that path
                        println!("{:?} is overjolt", f);
                        map.insert(f.clone());
                        continue;
                    }
                    for b in 0..expb.len() {
                        let mut ff = f.clone();
                        for i in 0..self.jolts.len() {
                            ff[i] += expb[b][i];
                        }
                        // println!("Building edge {:?}", ff);
                        edges.insert(ff.clone());
                        newfront.insert(ff.clone());
                    }
                    if edges.clone().into_iter().any(|x| x == self.jolts) {
                        println!("Found start! Depth: {}", depth);
                        done = true;
                        break;
                    }
                    edges.clear();
                    map.insert(f.clone());
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
    let mut reader = BufReader::new(File::open("factory.txt").expect("reading file failed"));
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
