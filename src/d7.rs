use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run() {
    let mut reader = BufReader::new(File::open("beam.txt").expect("reading file failed"));
    let mut l = String::new();
    let mut buf = vec![b'.'; 256];
    let mut splits: u64 = 0;

    loop {
        match reader.read_line(&mut l) {
            Ok(x) => {
                if x == 0 {
                    break;
                }

                let mut lbuf = l.clone().into_bytes();

                for i in 0..lbuf.len() {
                    match lbuf[i] {
                        b'.' => {
                            if buf[i] == b'|' {
                                lbuf[i] = b'|';
                                //println!("Beam travels through empty space");
                            }
                        }
                        b'S' => {
                            lbuf[i] = b'|';
                            //println!("Beam emits from the source");
                        }
                        b'^' => {
                            if buf[i] == b'|' {
                                splits += 1;
                                //println!("Splitting beam!");
                                if i-1 >= 0 {
                                    lbuf[i-1] = b'|';
                                }
                                if i+1 < lbuf.len() {
                                    lbuf[i+1] = b'|';
                                }
                            }
                        }
                        b'|' | b'\n' => (),
                        _ => {
                            panic!("Unexpected char {}", lbuf[i]);
                        }
                    }
                }
                buf = lbuf.to_vec();
                print!("{}", String::from_utf8(buf.clone()).unwrap());
                l.clear();
            }
            Err(x) => panic!("{:?}", x),
        }
    }
    println!("Beam split {} times", splits);
}
