// allow all warnings for development
#![allow(warnings)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;

use regex::Regex;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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

    fn eq(&self, other: &SimpleVector) -> bool {
        return self.arr.iter().zip(other.arr.iter()).all(|(&a, &b)| a == b);
    }

    fn distance_from_target(&self, target: &SimpleVector) -> f64 {
        let sum_of_squares: f64 = self.arr.iter()
            .zip(target.arr.iter())
            .map(|(&a, &b)| {
                let diff = (a as f64) - (b as f64);
                diff * diff
            })
            .sum();
        return sum_of_squares.sqrt();
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

    fn do_vector_dfs(&self) -> usize {
        // best should just return the best button from our current position unless we've already
        // explored that path
        // we should reimplement hashset to cache explored paths
        // we need a failure condition so that we know when to walk back
        // we can probably use recursion using this vector best-path algorithm
        //
        // maybe we make the best path smart enough to realize which buttons have already been
        // pushed from a given node and stop checking them so it still returns the best unchecked
        // path when asked for the best path
        //
        // if a button returns overjolt we should mark that path fully explored

        fn overjolt(target: &SimpleVector, node: &SimpleVector, button: &SimpleVector) -> bool {
            // b is the button we will push
            for ((i, j), k) in zip(zip(target.arr, node.arr), button.arr) {
                if i == 0 {
                    // target only has zero values after relevant dimensions
                    continue;
                }
                // println!("Trying {} < ({} + {})", i, j, k);
                if i < (j + k) {
                    // println!("Node {:?} + {:?} is overjolt", node, button);
                    return true;
                }
            }
            return false;
        }

        fn best(target: &SimpleVector, children: &mut Vec<SimpleVector>) {
            children.sort_by(|a, b| a.distance_from_target(&target).partial_cmp(&b.distance_from_target(&target)).unwrap());
        }

        fn descend(done: &mut bool, node: &SimpleVector, adjlist: &mut HashMap<SimpleVector, Vec<SimpleVector>>, depth: &mut usize, buttons: &Vec<SimpleVector>, target: &SimpleVector) {
            // println!("Depth is {}", depth);
            *depth += 1;

            // println!("{:?}", adjlist);
            if adjlist.get(node) == None {
                // node unprovisioned
                // println!("Unprovisioned node {:?}", node);
                let mut vnv = Vec::<SimpleVector>::with_capacity(buttons.len());
                //
                // make the child nodes
                for button in buttons {
                    // don't explore overjolt nodes
                    if overjolt(&target, &node, &button) {
                        continue;
                    }
                    vnv.push(node.add(&button));
                }
                // println!("Added nodes {:?}", vnv);

                if vnv.len() == 0 {
                    println!("Discovered leaf {:?} at depth {}", node, depth);
                    return;
                }
                // let d = vnv.clone().into_iter().map(|x: SimpleVector| x.distance_from_target(&target)).collect::<Vec<f64>>();
                // println!("VNV {:?}", d);
                best(&target, &mut vnv);
                // let d = vnv.clone().into_iter().map(|x: SimpleVector| x.distance_from_target(&target)).collect::<Vec<f64>>();
                // println!("VNV {:?}", d);
                // println!("Inserting {:?} with {:?}", node, vnv);
                let _ = adjlist.insert(node.clone(), vnv.clone());
            }

            let r = adjlist.get(node).unwrap();
            // check the adjacencies
            for vn in r.clone() {
                if *done {
                    break;
                }
                if vn.eq(target) {
                    println!("Found target!! {:?} == {:?} at depth {}", vn, target, depth);
                    *done = true;
                    return;
                }
                // let vn = &best(&target, &x);
                // println!("Recursing into {:?}", vn);
                descend(done, &vn, adjlist, depth, &buttons, &target);
            }
            *depth -= 1;
        }

        let mut depth: usize = 0;
        let mut adjlist = HashMap::<SimpleVector, Vec<SimpleVector>>::with_capacity(256);
        let mut done = false;

        descend(&mut done, &self.jolts.clone(), &mut adjlist, &mut depth, &self.buttons, &self.target);
        println!("Target {:?} reached at depth {}", &self.target, depth);
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
                let buttons: Vec::<&str> = bre.find_iter(&line).map(|m| m.as_str()).collect();
                let jolts = jre.find(&line);

                let mut l = 0_u16;
                let mut jv = [0u16; 10];

                for (i, j) in jolts.unwrap().as_str().strip_prefix('{').unwrap().strip_suffix('}').unwrap().split(',').enumerate() {
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

    machines[0].do_vector_dfs();
    // println!("Ans {}", machines.into_iter().map(|m| m.do_vector_dfs()).sum::<usize>());
}

#[cfg(test)]
mod tests {
    use super::*;
}
