//#[allow(unused)]
//#[allow(unused_assignments)]
//#[allow(unused_variables)]
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Sub;
use std::fmt;
use regex::Regex;

// light has max 10 position
// we map the positions into bit space as (2**n)
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
                    if c == '.' {
                        l <<= 1;
                    } else if c == '#' {
                        l |= 1;
                        l <<= 1;
                    }
                }
                println!("Lights is {:b}", l);
                m.start_cond = l;
                for bt in buttons {
                    let mut btn: u16 = 0;
                    for c in bt.chars() {
                        match c {
                            '0'..='9' => {
                                let bv = 2_u16.pow(c.to_digit(10).unwrap());
                                btn |= bv;
                            }
                            _ => (),
                        }
                    }
                    m.buttons.push(btn);
                }
                machines.push(m);
                line.clear();
            }
            Err(x) => panic!("{:?}", x),
        }
    }
}
