use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

// light has max 10 position
// we map the positions into bit space as (2**n)
#[derive(Debug)]
struct Machine {
    lights: u16,
    buttons: Vec<u16>,
    start_cond: u16,
    running: bool,
}

impl Machine {
    fn new() -> Self {
        Machine {
            lights: 0,
            buttons: Vec::<u16>::new(),
            start_cond: 0,
            running: false,
        }
    }

    fn add_button(&mut self, b: u16) {
        self.buttons.push(b);
    }

    fn set_start_condition(&mut self, s: u16) {
        self.start_cond = s;
    }

    fn push_button(&mut self, b: usize) {
        self.lights ^= self.buttons[b];
    }

    fn do_bfs(&self) -> usize {
        let mut map: HashMap<u16, HashSet<u16>> = HashMap::new();
        // map.insert(0, HashSet::<u16>::new());
        // map.insert(self.start_cond, HashSet::<u16>::new());
        let mut frontier: HashSet<u16> = HashSet::from([0]);

        let mut done = false;
        let mut depth = 1;

        loop {
            let mut newfront: HashSet<u16> = HashSet::new();

            for f in &frontier {
                println!("Checking key {}", f);
                if !map.contains_key(f) {
                    // new node
                    let mut edges = HashSet::new();
                    for bdx in 0..self.buttons.len() {
                        let e = self.buttons[bdx] ^ f;
                        println!("Building edge {}", e);
                        edges.insert(e.clone());
                        newfront.insert(e.clone());
                    }
                    // edges.clone().into_iter().map(|x| newfront.insert(x));
                    if edges.clone().into_iter().any(|x| x == self.start_cond) {
                        println!("Found start! Depth: {}", depth);
                        done = true;
                        break;
                    }
                    map.insert(*f, edges);
                } else {
                    println!("{} already visited", f);
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

//impl fmt::Display for Machine {
//    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//        write!(f, "Machine([{}],[])", self.x, self.y, self.z)
//    }
//}

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
                for (i, c) in lights.unwrap().as_str().chars().enumerate() {
                    if c == '#' {
                        l |= 2_u16.pow((i-1).try_into().unwrap());
                        println!("Found # l is {}", l);
                    }
                }
                println!("Constructed start {} from {}", l, lights.unwrap().as_str());
                m.start_cond = l;
                for bt in buttons {
                    let mut btn: u16 = 0;
                    for c in bt.chars() {
                        // error somewhere here how did 2,3 => 0d12 => 0b1100
                        // we have to make sure our lights and buttons map to the same space
                        match c {
                            '0'..='9' => btn |= 2_u16.pow(c.to_digit(10).unwrap()),
                            _ => (),
                        }
                    }
                    println!("Constructed button {} from {}", btn, bt);
                    m.buttons.push(btn);
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
        for ibx in 0..m.buttons.len() {
            m.push_button(ibx);
        }
        let d = m.do_bfs();
        depths.push(d);
        println!("Machine depth {}", d);
    }
    println!("{}", depths.into_iter().sum::<usize>());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_machine() -> Machine {
        let mut m = Machine::new();
        m.start_cond = 1;
        m.buttons.push(1);
        return m;
    }

    #[test]
    fn test_machine_lights() {
        let m = setup_machine();
        assert_eq!(m.lights, 0);
    }

    #[test]
    fn test_machine_start_cond() {
        let m = setup_machine();
        assert_eq!(m.start_cond, 1);
    }

    #[test]
    fn test_machine_button() {
        let m = setup_machine();
        assert_eq!(m.buttons, vec![1]);
    }

    #[test]
    fn test_machine_button_toggle() {
        let mut m = setup_machine();
        m.push_button(0);
        assert_eq!(m.lights, m.start_cond);
    }

    #[test]
    fn test_machine_multi_buttons() {
        let mut m = Machine::new();
        m.start_cond = 12;
        m.buttons = vec![0b100, 0b1, 0b1001];
        m.push_button(0);
        assert_eq!(m.lights, 0b100);
        m.push_button(1);
        assert_eq!(m.lights, 0b101);
        m.push_button(2);
        assert_eq!(m.lights, 0b1100);
        assert_eq!(m.lights, m.start_cond);

    }
}
