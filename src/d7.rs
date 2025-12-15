use std::fs::File;
use std::io::{BufRead, BufReader};
use std::boxed::Box;

// beam starts from 'S'
// beams split at x to x-1 and x+1
// where x is the position of '^'
// beam passes through space '.' freely

// any node in the graph should be linked vertically to nodes below
// unless it is a splitter, then it must be linked left and right
// nodes should be informed if they are receiving a beam
// so they may emit a beam to the node below

// we actually don't care about any unreachable nodes so we should
// parse the file and add nodes as we discover reachable ones

type Link = Option<Box<Node>>;

#[derive(Debug)]
struct Manifold {
    source: Link,
}

impl Manifold {
    fn new() -> Self {
        Manifold { source: None }
    }
}

#[derive(Debug)]
struct Detector {
    count: u64,
}

impl Detector {
    fn new() -> Self {
        Detector { count: 0 }
    }

    fn inform(&mut self) {
        println!("Tachyon split detected!");
        self.count += 1;
    }
}

#[derive(Debug)]
struct Node {
    // we don't implement 'up' link because beam propagates down
    source: bool,
    split: bool,
    beam: bool,
    left: Link,
    right: Link,
    down: Link,
    detector: Option<Box<Detector>>,
}

impl Node {
    fn new() -> Self {
        Self { source: false, beam: false, split: false, down: None, left: None, right: None, detector: None }
    }

    fn send_beam(&mut self) {
        self.beam = true;
        if self.split {
            println!("{:?} splitting beam!", self);
            if let l = self.left {
                l.expect("left was none").send_beam();
            }
            if let r = self.right {
                r.expect("r was none").send_beam();
            }
            if let d = self.detector {
                d.expect("d was none").inform();
            }
        } else {
            println!("{:?} sending beam!", self);
            if let d = self.down {
                d.expect("foo").send_beam();
            }
        }
    }
}

fn build_manifold(file: &str, manifold: &mut Manifold) {
    let mut reader = BufReader::new(File::open(file).expect("reading file failed"));
    let mut l = String::new();
    loop {
        match reader.read_line(&mut l) {
            Ok(x) => {
                if x == 0 {
                    break;
                }
                println!("{}", l);
                l.clear();
            }
            Err(x) => panic!("{:?}", x),
        }
    }
    //m.source.as_mut().unwrap().down = Some(Box::new(Node::new()));
    //m.source.as_mut().unwrap().down.as_mut().unwrap().split = true;
    //m.source.as_mut().unwrap().down.as_mut().unwrap().left = Some(Box::new(Node::new()));
    //m.source.as_mut().unwrap().down.as_mut().unwrap().right = Some(Box::new(Node::new()));
}

pub fn run() {

    let mut d = Detector::new();

    let mut m = Manifold::new();
    m.source = Some(Box::new(Node::new()));

    build_manifold("beam_sample.txt", &mut m);

    m.source.as_mut().unwrap().send_beam();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_create() {
        let mut m = Manifold::new();
        m.source = Some(Box::new(Node::new()));
        assert_eq!(m.source.unwrap().split, false);
    }

}
