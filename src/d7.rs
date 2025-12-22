use std::fs::File;
use std::io::{BufReader, Read};

pub fn run() {
    let mut reader = BufReader::new(File::open("beam.txt").expect("reading file failed"));
    let mut buf = Vec::<u8>::new();
    let _ = reader.read_to_end(&mut buf);

    let mut lbuf = Vec::<u64>::new();
    let mut cbuf = Vec::<u64>::new();
    let mut lchar: u8 = b'.';

    let mut reset: bool = false;
    let mut idx: usize = 0;

    for c in buf.drain(..) {
        if !(lchar == b'^') {
            cbuf.push(0);
        }
        match c {
            b'.' => {
                if lchar == b'^' {
                    cbuf.push(lbuf[idx-1]);
                    println!("idx {}", idx);
                    println!("las buf {}", lbuf[idx-1]);
                    cbuf[idx] = lbuf[idx];
                    cbuf[idx] += lbuf[idx-1];
                } else {
                    if idx < lbuf.len() {
                        cbuf[idx] = lbuf[idx];
                    } else {
                        // we grow the vec for the first row
                        cbuf[idx] = 0;
                    }
                }
            }
            b'^' => {
                println!("Handling ^");
                // check cell above and set left/right appropriately
                // handle setting cell right based on cell upper-right ?
                println!("lbuf {:?}", lbuf);
                println!("cbuf {:?}", cbuf);
                cbuf[idx-1] += lbuf[idx];
                // beam split, current cell has zero paths
                cbuf[idx] = 0;
            }
            b'\n' => {
                println!("Resetting line");
                // swap buffers
                // we don't do anything clever with pointers, we just clone
                lbuf = cbuf.clone();
                cbuf.clear();
                reset = true;
            }
            b'S' => {
                // set current index to 1
                // there will only be one source
                cbuf[idx] = 1;
            }
            _ => {
            }
        }
        lchar = c;
        idx += 1;
        if reset {
            reset = false;
            idx = 0;
        }
        //assert_eq!(idx, cbuf.len() - 1);
    }

    println!("{:?}", lbuf);
    println!("{:?}", cbuf);
    println!("There are {} timelines", lbuf.into_iter().sum::<u64>());
}
