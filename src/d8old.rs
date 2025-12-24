use std::fs::File;
use std::io::{BufRead, BufReader};

struct Node {
    x: i64,
    y: i64,
    z: i64,
}

impl Node {
    // do we care for floats or should we just do integer math?
    fn new(x: i64, y: i64, z: i64) -> Self {
        //println!("Creating node {} {} {}", x, y, z);
        // a negative circuit id indicates no circuit membership
        Node { x, y, z }
    }

    fn dist(&self, n: &Node) -> f64 {
        let dx: i64 = self.x - n.x;
        let dy: i64 = self.y - n.y;
        let dz: i64 = self.z - n.z;
        ((dx.pow(2) + dy.pow(2) + dz.pow(2)) as f64).sqrt()
    }
}

#[derive(Debug)]
struct Graph {
    circuits: Vec<Vec<usize>>,
}

impl Graph {
    fn new() -> Self {
        Graph { circuits: Vec::<Vec<usize>>::new() }
    }

    fn connect(&mut self, i: usize, j: usize) {
        let mut di = false;
        let mut ci: usize = 0;
        let mut dj = false;
        let mut cj: usize = 0;

        for idx in 0..self.circuits.len() {
            if self.circuits[idx].contains(&i) {
                ci = idx;
                di = true;
            }
            if self.circuits[idx].contains(&j) {
                cj = idx;
                dj = true;
            }
        }

        if di && dj {
            if ci == cj {
                dj = false;
            }
        }

        if di && dj {
            //println!("Found both I: {} and J: {} in circuits {:?}-{:?}", &i, &j, &self.circuits[ci], &self.circuits[cj]);
            // pop J circuit and extend I circuit with it
            println!("Connecting circuits {:?}:{:?}", &self.circuits[ci], &self.circuits[cj]);
            let diter = self.circuits.swap_remove(cj).into_iter();
            self.circuits[ci].extend(diter);

        } else if di {
            if !self.circuits[ci].contains(&j) {
                println!("Connecting {}:{} in {:?}", &i, &j, &self.circuits[ci]);
                self.circuits[ci].push(j);
            } else {
                //println!("{} already in {:?}", &j, &self.circuits[ci]);
            }
        } else if dj {
            if !self.circuits[cj].contains(&i) {
                println!("Connecting {}:{} in {:?}", &i, &j, &self.circuits[cj]);
                self.circuits[cj].push(i);
            } else {
                //println!("{} already in {:?}", &i, &self.circuits[cj]);
            }
        } else {
            // if we get here we push a new circuit
            println!("Connecting {}:{} in new circuit", &i, &j);
            self.circuits.push(Vec::<usize>::new());
            let l = self.circuits.len()-1;
            self.circuits[l].push(i);
            self.circuits[l].push(j);
        }
    }
}

#[derive(Debug)]
struct Distance {
    a: usize,
    b: usize,
    d: f64,
}

impl Distance {
    fn new(a: usize, b: usize, d: f64) -> Self {
        Self { a, b, d }
    }
}

pub fn run() {
    let mut reader = BufReader::new(File::open("junction.txt").expect("reading file failed"));
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

    let mut distance = Vec::<Distance>::new();

    let mut dup = false;
    for i in 0..nodes.len() {
        for j in 0..nodes.len() {
            if i == j {
                continue;
            }
            dup = false;
            for d in &distance {
                if d.b == i && d.a == j {
                    dup = true;
                }
            }
            if !dup {
                let dist: f64 = nodes[i].dist(&nodes[j]);
                println!("Computed {} between {}:{}", dist, i, j);
                distance.push(Distance::new(i, j, dist));
            }
        }
    }
    println!("Done building distances");

    // sort the distances
    distance.sort_by(|a, b| a.d.partial_cmp(&b.d).unwrap());
    for d in &distance {
        println!("{:?}", d);
    }
    //println!("{:?}", distance);

    let mut graph = Graph::new();

    for i in 0..10 {
        graph.connect(distance[i].a, distance[i].b);
    }

    println!("{:?}", graph);

    graph.circuits.sort_by(|b, a| a.len().partial_cmp(&b.len()).unwrap());
    println!("{:?}", graph);
    let mut ans: usize = 1;
    for i in 0..3 {
        ans = ans * graph.circuits[i].len();
    }
    println!("{}", ans);
}
