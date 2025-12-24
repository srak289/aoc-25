use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Sub;
use std::fmt;

#[derive(Eq, PartialEq, Debug, Hash)]
struct Node {
    x: i64,
    y: i64,
    z: i64,
}

impl Node {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Node { x, y, z }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

impl Sub for &Node {
    type Output = f64;
    fn sub(self, other: &Node) -> f64 {
        let dx: i64 = self.x - other.x;
        let dy: i64 = self.y - other.y;
        let dz: i64 = self.z - other.z;
        ((dx.pow(2) + dy.pow(2) + dz.pow(2)) as f64).sqrt()
    }
}

#[derive(Debug)]
struct Graph<'a> {
    circuits: Vec<HashSet<&'a Node>>,
}

impl<'a> Graph<'a> {
    fn new() -> Self {
        Graph { circuits: Vec::<HashSet<&Node>>::new() }
    }

    fn connect(&mut self, i: &'a Node, j: &'a Node) {
        let mut di = false;
        let mut ci: usize = 0;
        let mut dj = false;
        let mut cj: usize = 0;

        // we start with two nodes
        // we search all sets for each node
        // if both nodes are in separate circuits
        // we combine the circuits
        // otherwise we push the orphan node to the set of the
        // member node
        // else we create a new set and push that to our list
        for idx in 0..self.circuits.len() {
            if self.circuits[idx].contains(i) {
                println!("Found {} in circuit {}", i, idx);
                ci = idx;
                di = true;
            }
            if self.circuits[idx].contains(j) {
                println!("Found {} in circuit {}", j, idx);
                cj = idx;
                dj = true;
            }
        }

        // negate the false-positive where both nodes are seen in the
        // same set
        if di && dj {
            if ci == cj {
                dj = false;
            }
        }

        if di && dj {
            println!("Connecting circuits {}:{}", &ci, &cj);
            let hs: HashSet<&Node> = self.circuits.swap_remove(cj);
            let _ = self.circuits[ci].union(&hs);
        } else if di {
            // detected i in circuit ci, so insert j to ci
            // j is an orphan
            let _ = self.circuits[ci].insert(j);
            println!("Connecting {}:{}", i, j);
        } else if dj {
            // detected j in circuit cj, so insert i to cj
            // i is an orphan
            let _ = self.circuits[cj].insert(i);
            println!("Connecting {}:{}", i, j);
        } else {
            // if we get here we push a new circuit
            let hs = HashSet::from([i, j]);
            println!("Adding new circuit {}:{}", &i, &j);
            self.circuits.push(hs);
        }
    }
}

pub fn run() {
    let mut reader = BufReader::new(File::open("junction_sample.txt").expect("reading file failed"));
    let mut line = String::new();

    let mut nodes = Vec::<Node>::new();

    loop {
        match reader.read_line(&mut line) {
            Ok(bytes) => {
                if bytes == 0 {
                    break;
                }
                // get rid of \n
                let _ = line.pop();
                let [x, y, z] = line.split(',').map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>().try_into().unwrap();
                nodes.push(Node::new(x, y, z));
                line.clear();
            }
            Err(x) => panic!("{:?}", x),
        }
    }
    println!("Done building nodes");

    let mut distances: HashMap<(&Node, &Node), f64> = HashMap::new();

    let mut d: u64 = 0;

    for i in 0..nodes.len() {
        for j in i+1..nodes.len() {
            //println!("Distance {:?}:{:?} is {}", &a, &b, (a-b));
            d += 1;
            distances.insert((&nodes[i], &nodes[j]), &nodes[i]-&nodes[j]);
        }
    }
    println!("Done building {} distances", &d);

    let mut sorted = distances.iter().collect::<Vec<(&(&Node, &Node), &f64)>>();
    sorted.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    println!("Done sorting");
    //println!("Sorted {:#?}", &sorted);

    let mut graph = Graph::new();

    for d in 0..10 {
        let n = sorted.pop();
        graph.connect(n.unwrap().0.0, n.unwrap().0.1);
        println!("{:#?}", &graph);
    }
    println!("Done building graph");

    graph.circuits.sort_by(|b, a| a.len().partial_cmp(&b.len()).unwrap());
    //println!("{:?}", graph);
    let mut ans: usize = 1;
    for i in 0..3 {
        ans = ans * graph.circuits[i].len();
    }
    println!("{}", ans);
}
