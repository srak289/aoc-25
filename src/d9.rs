use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Sub;
use std::fmt;

#[derive(Debug, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }

    fn rect_area(&self, o: &Point) -> i64 {
        // given an opposing corner compute the area
        let dx: i64 = (self.x - o.x).abs() + 1;
        println!("abs({} - {}) = {}", self.x, o.x, dx);
        let dy: i64 = (self.y - o.y).abs() + 1;
        println!("abs({} - {}) = {}", self.y, o.y, dy);
        return dx * dy;
    }
}

pub fn run() {
    let mut reader = BufReader::new(File::open("movie.txt").expect("reading file failed"));
    let mut line = String::new();

    let mut points = Vec::<Point>::new();

    loop {
        match reader.read_line(&mut line) {
            Ok(bytes) => {
                if bytes == 0 {
                    break;
                }
                // get rid of \n
                let _ = line.pop();
                let [x, y] = line.split(',').map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>().try_into().unwrap();
                let n = Point::new(x, y);
                println!("Constructed point {:?}", n);
                points.push(n);
                line.clear();
            }
            Err(x) => panic!("{:?}", x),
        }
    }

    let mut big_a: i64 = 0;

    for i in 0..points.len() {
        for j in i+1..points.len() {
            let a: i64 = points[i].rect_area(&points[j]);
            if a > big_a {
                big_a = a;
            }
            println!("Area between {:?} and {:?} is {}", &points[i], &points[j], a);
        }
    }
    println!("BIG A {}", big_a);
    // ans: 4749838800
}
