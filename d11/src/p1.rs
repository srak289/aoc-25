#![allow(warnings)]
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

use regex::{Match, Regex};

struct DG {
    nodes: HashMap<String, Vec<String>>,
}

impl DG {
    fn new() -> Self {
        Self { nodes: HashMap::new() }
    }

    fn insert(&mut self, s: String, n: Vec<String>) {
        self.nodes.insert(s, n);
    }

    fn solve(&self) -> usize {
        // remember cycle detection
        // find 'backedges'
        //
        // start point is 'you'
        // find all paths to 'out'

        let mut alist: HashSet<String> = HashSet::with_capacity(1024);

        let mut paths: usize = 0;

        fn descend(node: &String, nodes: &HashMap<String, Vec<String>>, alist: &mut HashSet<String>, paths: &mut usize) {
            match nodes.get(node) {
                Some(edges) => {
                    println!("Found edges {:?}", edges);
                    for n in edges {
                        descend(n, nodes, alist, paths);
                    }
                }
                None => {
                    *paths += 1;
                    println!("Found leaf {}", node);
                }
            }
        }

        descend(&String::from("you"), &self.nodes, &mut alist, &mut paths);
        return paths;
    }
}

fn build_dg() -> DG {
    let mut reader = BufReader::new(File::open("input.txt").expect("reading file failed"));
    let mut line = String::new();

    let label_re = Regex::new(r"^([a-z]+):").expect("invalid regex");
    let output_re = Regex::new(r" ([a-z]+)").expect("invalid regex");

    let mut dg = DG::new();

    loop {
        match reader.read_line(&mut line) {
            Ok(bytes) => {
                if bytes == 0 {
                    break;
                }
                let label = label_re.captures(&line).unwrap().get(1).unwrap().as_str();
                // println!("Label: {}", label);
                let outputs: Vec::<_> = output_re.find_iter(&line).map(|m: Match| m.as_str().trim().to_string()).collect();
                // println!("Outputs {:?}", outputs);
                dg.insert(label.to_string().clone(), outputs.clone());
                line.clear();
            }
            Err(x) => panic!("{:?}", x),
        }
    }

    return dg;
}

fn main() {
    let dg = build_dg();
    let paths = dg.solve();
    println!("There are {}", paths);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_you_out() {
        let dg = build_dg();
        let paths = dg.solve();
        assert_eq!(paths, 5);
    }
}
