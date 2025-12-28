use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Sub;
use std::fmt;
use std::boxed::Box;

type Point = (i64, i64);

#[derive(Debug)]
struct Polygon {
    points: Vec<Point>,
    verticals: Vec<(Point, Point)>,
    cache: HashMap<Point, bool>,
}

impl Polygon {
    fn new() -> Self {
        Polygon {
            points: Vec::<Point>::new(),
            verticals: Vec::<(Point, Point)>::new(),
            cache: HashMap::<Point, bool>::new(),
        }
    }

    fn contained(&self, p: &Point) -> bool {
        // this intersection is greatly simplified because we are only
        // checking for horizontal rays intersecting vertical segments
        let mut intersects = 0;
        let mut shouldve = 0;
        let p2 = (100_000, p.1);
        for v in &self.verticals {
            // if the line is behind our point
            // v.0.0 = a.x
            // remember v.0.x == v.1.x because vertical set of points
            if v.0.0 < p.0 {
                //println!("Skipping line {:?}", v);
                continue
            }
            // we should make sure we check both? orientations of the line?
            // or we should guarantee we always calculate it one way .. 
            let (mut b1, mut b2) = (v.0, v.1);
            if v.0.1 > v.1.1 {
                (b2, b1) = (v.0, v.1);
            }
            if p.0 == b2.0 && p.1 >= b1.1 && p.1 <= b2.1 {
                //println!("Point {:?} sits on an edge, short-circuit true", p);
                return true;
            }
            //println!("Evaluate {:?}, {:?}, {:?}, {:?}", p, p2, b1, b2);
            //println!("Evaluate {} <= {}, {} >= {}, {} < {}, {} > {}", p.0, b1.0, p2.0, b1.0, p.1, b2.1, p.1, b1.1);
            //println!("Evaluate {}, {}, {}, {}", (p.0 <= b1.0), (p2.0 >= b1.0), (p.1 < b2.1), (p.1 > b1.1));
            if p.1 == b2.1 || p.1 == b1.1 {
                shouldve += 1;
                println!("Should've {:?}, {:?}, {:?}", &p, &b1, &b2);
            }
            if (
                p.0 <= b1.0
                && p2.0 >= b1.0
                && p.1 < b2.1
                && p.1 > b1.1
            ) {
                println!("Found intersect {:?}, {:?}, {:?}", &p, &b1, &b2);
                intersects += 1;
            }
        }
        let ret = (intersects % 2 == 1 || shouldve % 2 == 1);
        println!("Returning intersection {}", ret);
        return ret;
    }

    fn contains(&mut self, p: &Point) -> bool {
        // we cache our finds to hopefully gain later
        match self.cache.get(p) {
            Some(x) => {
                // got ~250k cache-hits so was worth
                //println!("Cache-HIT {:?}", &p);
                return x.clone();
            }
            None => {
                println!("Checking point {:?}", p);
                let b = self.contained(p);
                self.cache.insert((*p).clone(), b);
                return b;
            }
        };
    }

    fn points_for(a: &Point, b: &Point) -> Box<dyn Iterator<Item=Point>> {
        if a.0 < b.0 {
            // then c is also left
            if a.1 < b.1 {
                let c = (b.0, a.1);
                let d = (a.0, b.1);
                // then a, c, b, d
                //println!("A is close to origin, Order a,c,b,d");
                //println!("Returning points {:?}, {:?}, {:?}, {:?}",
                //    (a.0+1, a.1), (c.0, c.1+1), (b.0-1, b.1), (d.0, d.1-1));
                return Box::new([(a.0+1, a.1), (c.0, c.1+1), (b.0-1, b.1), (d.0, d.1-1)].into_iter())
                
            } else {
                let c = (a.0, b.1);
                let d = (b.0, a.1);
                // then c, b, d, a
                //println!("C is close to origin, Order c,b,d,a");
                //println!("Returning points {:?}, {:?}, {:?}, {:?}",
                //    (c.0+1, c.1), (b.0, b.1+1), (d.0-1, d.1), (a.0, a.1-1));
                return Box::new([(c.0+1, c.1), (b.0, b.1+1), (d.0-1, d.1), (a.0, a.1-1)].into_iter())
            }
        } else {
            // then c is also right
            if a.1 < b.1 {
                let c = (a.0, b.1);
                let d = (b.0, a.1);
                // d, a, c, b
                //println!("D is close to origin, Order d,a,c,b");
                //println!("Returning points {:?}, {:?}, {:?}, {:?}", d, a, c, b);
                //println!("Modified points {:?}, {:?}, {:?}, {:?}",
                //    (d.0+1, d.1), (a.0, a.1+1), (c.0-1, c.1), (b.0, b.1-1));
                return Box::new([(d.0+1, d.1), (a.0, a.1+1), (c.0-1, c.1), (b.0, b.1-1)].into_iter())
            } else {
                let c = (b.0, a.1);
                let d = (a.0, b.1);
                // b, d, a, c
                //println!("B is close to origin, Order b,d,a,c");
                //println!("Returning points {:?}, {:?}, {:?}, {:?}",
                //    (b.0+1, b.1), (d.0, d.1+1), (a.0-1, a.1), (c.0, c.1-1));
                return Box::new([(b.0+1, b.1), (d.0, d.1+1), (a.0-1, a.1), (c.0, c.1-1)].into_iter())
            }
        }
    }

    fn area(&mut self, adx: usize, bdx: usize) -> Option<i64> {
        // this function returns area optionally,
        // if the rectangle does not match our intersection criteria
        // we discard the result

        // we first pick A as the point closest to the origin, and B as
        // the opposite corner from A
        //
        // if we calculate four points from the rectangle
        // one for each corner, but rotated one position counter-clockwise
        // we can guarantee the rectangle we drew is still in the polygon

        // first we determine which corners are which
        let mut a = self.points[adx].clone();
        let mut b = self.points[bdx].clone();
        let mut t: i64 = 0;

        // short-circuit for single width
        if a.1 == b.1 {
            return Some((a.1 - b.1).abs() + 1);
        } else if a.0 == b.0 {
            return Some((a.1 - b.1).abs() + 1);
        }

        //println!("Checking rect {:?}:{:?}", a, b);

        for p in Self::points_for(&a, &b) {
            //println!("Checking {:?}", p);
            if !self.contains(&p) {
                return None;
            }
        }
        //println!("Rect area of {:?},{:?}", &a, &b);
        //println!("Rect area {} * {}", ((a.0 - b.0).abs() + 1), ((a.1 - b.1).abs() + 1));
        return Some(((a.0 - b.0).abs() + 1) * ((a.1 - b.1).abs() + 1));
    }

    fn cache_verticals(&mut self) {
        for idx in 0..self.points.len()-1 {
            if self.points[idx].0 == self.points[idx+1].0
                && self.points[idx].1 != self.points[idx+1].1
            {
                //println!("Vertical edge! {:?}:{:?}", &self.points[idx], &self.points[idx+1]);
                if self.points[idx].1 < self.points[idx+1].1 {
                    self.verticals.push((self.points[idx], self.points[idx+1]));
                } else {
                    self.verticals.push((self.points[idx+1], self.points[idx]));
                }
            }
        }
        if self.points[0].0 == self.points[self.points.len()-1].0
            && self.points[0].1 != self.points[self.points.len()-1].1
        {
            //println!("Vertical edge! {:?}:{:?}", &self.points[0], &self.points[self.points.len()-1]);
            if self.points[0].1 < self.points[self.points.len()-1].1 {
                self.verticals.push((self.points[0], self.points[self.points.len()-1]));
            } else {
                self.verticals.push((self.points[self.points.len()-1], self.points[0]));
            }
        }
    }
}

pub fn run() {
    let mut reader = BufReader::new(File::open("movie.txt").expect("reading file failed"));
    let mut line = String::new();

    let mut poly = Polygon::new();

    loop {
        match reader.read_line(&mut line) {
            Ok(bytes) => {
                if bytes == 0 {
                    break;
                }
                // get rid of \n
                let _ = line.pop();
                let [x, y] = line.split(',').map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>().try_into().unwrap();
                let n = (x, y);
                //println!("Constructed point {:?}", n);
                poly.points.push(n);
                line.clear();
            }
            Err(x) => panic!("{:?}", x),
        }
    }

    poly.cache_verticals();

    let mut num_rect: i64 = 0;
    let mut valid_rect: i64 = 0;
    let mut big_a: i64 = 0;

    for i in 0..poly.points.len() {
        for j in i+1..poly.points.len() {
            num_rect += 1;
            match poly.area(i, j) {
                Some(x) => {
                    valid_rect += 1;
                    if x > big_a {
                        //println!("Found big a!");
                        big_a = x;
                        println!("BIG {} constructed from {:?},{:?}", big_a, &poly.points[i], &poly.points[j]);
                    }
                }
                None => (),//println!("Points {:?} and {:?} don't pass intersection", &poly.points[i], &poly.points[j]),
            }
        }
    }
    println!("Checked {} rects {} were valid", num_rect, valid_rect);

    println!("Big A: {}", big_a);
    assert_ne!(big_a, 4646235780); // too high

    //let mut big_a: i64 = 0;

    //for i in 0..points.len() {
    //    for j in i+1..points.len() {
    //        let a: i64 = points[i].rect_area(&points[j]);
    //        if a > big_a {
    //            big_a = a;
    //        }
    //        println!("Area between {:?} and {:?} is {}", &points[i], &points[j], a);
    //    }
    //}
    //println!("BIG A {}", big_a);
    //// ans: 4749838800
}

#[cfg(test)]
mod test {
    use super::*;

    fn setup_polygon() -> Polygon {
        let mut p = Polygon::new();
        p.points.push((7, 1)); //0
        p.points.push((11, 1)); //1
        p.points.push((11, 7)); //2
        p.points.push((9, 7)); //3
        p.points.push((9, 5)); //4
        p.points.push((2, 5)); //5
        p.points.push((2, 3)); //6
        p.points.push((7, 3)); //7
        p.cache_verticals();
        return p;
    }

    #[test]
    fn test_not_contains() {
        let mut p = setup_polygon();
        assert_eq!(p.contains(&(12,6)), false);
        assert_eq!(p.contains(&(1,6)), false);
    }

    #[test]
    fn test_contains() {
        let mut p = setup_polygon();
        assert_eq!(p.contains(&(11,2)), true);
        assert_eq!(p.contains(&(9,6)), true);
        assert_eq!(p.contains(&(3,3)), true);
    }

    #[test]
    fn test_origin_contains() {
        let mut p = setup_polygon();
        assert_eq!(p.contains(&(2,3)), true);
    }

    #[test]
    fn test_maximum_contains() {
        let mut p = setup_polygon();
        assert_eq!(p.contains(&(11,7)), true);
    }

    #[test]
    fn test_rect_area_is_none() {
        let mut p = setup_polygon();
        assert_eq!(p.area(0, 5), None);
        assert_eq!(p.area(1, 5), None);
        assert_eq!(p.area(0, 6), None);
        assert_eq!(p.area(1, 6), None);
        assert_eq!(p.area(2, 5), None);
        assert_eq!(p.area(5, 2), None);
    }

    #[test]
    fn test_rect_area_is_some() {
        let mut p = setup_polygon();
        assert_eq!(p.area(4, 3), Some(3));
        assert_eq!(p.area(3, 4), Some(3));
        assert_eq!(p.area(1, 7), Some(15));
        assert_eq!(p.area(4, 6), Some(24));
        assert_eq!(p.area(1, 2), Some(7));
        assert_eq!(p.area(1, 4), Some(15));
    }

    #[test]
    fn test_points_for_2_x_2() {
        let mut iter = Polygon::points_for(&(1,1), &(2,2));
        assert_eq!(iter.next(), Some((2,1)));
        assert_eq!(iter.next(), Some((2,2)));
        assert_eq!(iter.next(), Some((1,2)));
        assert_eq!(iter.next(), Some((1,1)));
    }

    #[test]
    fn test_points_acbd() {
        let mut iter = Polygon::points_for(&(1,1), &(3,3));
        assert_eq!(iter.next(), Some((2,1)));
        assert_eq!(iter.next(), Some((3,2)));
        assert_eq!(iter.next(), Some((2,3)));
        assert_eq!(iter.next(), Some((1,2)));
    }

    #[test]
    fn test_points_cbda() {
        let mut iter = Polygon::points_for(&(1,3), &(3,1));
        assert_eq!(iter.next(), Some((2,1)));
        assert_eq!(iter.next(), Some((3,2)));
        assert_eq!(iter.next(), Some((2,3)));
        assert_eq!(iter.next(), Some((1,2)));
    }
    #[test]
    fn test_points_bdac() {
        let mut iter = Polygon::points_for(&(3,3), &(1,1));
        assert_eq!(iter.next(), Some((2,1)));
        assert_eq!(iter.next(), Some((3,2)));
        assert_eq!(iter.next(), Some((2,3)));
        assert_eq!(iter.next(), Some((1,2)));
    }
    #[test]
    fn test_points_dacb() {
        let mut iter = Polygon::points_for(&(3,1), &(1,3));
        assert_eq!(iter.next(), Some((2,1)));
        assert_eq!(iter.next(), Some((3,2)));
        assert_eq!(iter.next(), Some((2,3)));
        assert_eq!(iter.next(), Some((1,2)));
    }
}
