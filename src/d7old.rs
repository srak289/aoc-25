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
struct Detector {
    count: u64,
}

impl Detector {
    fn new() -> Self {
        Detector { count: 0 }
    }

    fn inform(&mut self) {
        println!("Tachyon split detected at {:?}", self);
        self.count += 1;
    }
}

#[derive(Debug)]
struct Manifold {
    source: Link,
    detector: Detector,
}

impl Manifold {
    fn new() -> Self {
        Manifold { source: None, detector: Detector::new() }
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
    detector: *mut Detector,
}

impl Node {
    fn new(detector: *mut Detector) -> Self {
        Self { source: false, beam: false, split: false, down: None, left: None, right: None, detector }
    }

    fn send_beam(&mut self) {
        self.beam = true;
        if self.split {
            println!("{:?} splitting beam!", self);
            self.left.as_mut().map(|left| {
                &mut left.send_beam();
            });
            self.right.as_mut().map(|right| {
                &mut right.send_beam();
            });
            //unsafe {
            //    (*self.detector).inform();
            //}
        } else {
            println!("{:?} sending beam!", self);
            self.down.as_mut().map(|down| {
                &mut down.send_beam();
            });
        }
    }
}

// it's possible that we could have worked out the manifold splits by
// iterating the characters but we decided to make a graph
fn build_manifold(file: &str) -> Manifold {
    let mut reader = BufReader::new(File::open(file).expect("reading file failed"));
    let mut l = String::new();

    let mut manifold = Manifold::new();

    // the head (0, 0) of the grid
    let mut head: *mut Link = &mut None;

    // a pointer to the head of the last row finished
    let mut lasthead: *mut Link = &mut None;

    // a pointer to the head of the current row
    // set lastrowhead to this value when the current row ends
    let mut curhead: *mut Link = &mut None;

    // most recently created node
    let mut last: *mut Link = &mut None;
    //
    // most recently created node
    let mut last: *mut Link = &mut None;

    // a manifold node only knows about nodes below it
    // we might say the manifold is "singly-linked" vertically
    // and "doubly-linked" horizontally as the left-to-right nodes know about
    // both of their neighbors whether or not they are a splitter

    // we set the manifold Source when we find it
    // we'll iterate the rows, assembling left and right linkages as we go
    // keep pointers to the head of the list, upper left corner in this case
    // as that's how we read the file that's how we'll build the graph
    // keep pointer to the head of the start of the previous row and follow
    // the current row to create the down linkages

    head = &mut Some(Box::new(Node::new(&mut manifold.detector)));
    curhead = head;
    lasthead = head;
    // the head is actually a node outside the graph we can rely on

    loop {
        match reader.read_line(&mut l) {
            Ok(x) => {
                if x == 0 {
                    break;
                }
                // rm newline
                let _ = l.pop();

                for c in l.chars() {
                    last = &mut Some(Box::new(Node::new(&mut manifold.detector)));
                    match c {
                        '.' => {
                            println!("empty space");
                        }
                        'S' => {
                            (*last).source = true;
                            println!("source");
                        }
                        '^' => {
                            (*last).splitter = true;
                            println!("splitter");
                        }
                        _ => {
                            panic!("Unexpected char");
                        }
                    }
                }
                l.clear();
            }
            Err(x) => panic!("{:?}", x),
        }
    }
    return manifold;
}

pub fn run() {
    let mut m = build_manifold("beam_sample.txt");
    //m.source.as_mut().unwrap().send_beam();
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
