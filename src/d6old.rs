use std::fs::File;
use std::io::{SeekFrom, BufReader};

#[derive(Debug)]
enum Parser {
    DIGIT,
    OPCODE,
    SPACE,
    RESET,
}

#[derive(Debug)]
enum ColumnOp {
    ADD,
    MUL,
    NOOP,
}

#[derive(Debug)]
struct Column {
    ints: Vec<u64>,
    op: ColumnOp,
    width: u8, 
}

impl Column {
    fn new(op: ColumnOp, width: u8) -> Self {
        Self { ints: Vec::<u64>::new(), op, width }
    }

    fn calculate(&self) -> u128 {
        match self.op {
            ColumnOp::ADD => {
                return self.ints.clone().into_iter().sum::<u64>().into();
            }
            ColumnOp::MUL => {
                let mut ans: u128 = self.ints[0].into();
                for i in 1..self.ints.len() {
                    ans = ans * Into::<u128>::into(self.ints[i]);
                }
                return ans;
            }
            ColumnOp::NOOP => {
                panic!("{:?} specified NOOP", self);
            }
        }
    }
}

pub fn run() {
    let mut reader = BufReader::new(File::open("trash_sample.txt").expect("reading file failed"));
    //let mut fp = File::open("trash_sample.txt").expect("reading file failed");
    //let mut buf = [0; 1024];

    let mut columns = Vec::<Column>::new();
    let mut bint = Vec::<u8>::with_capacity(8);

    // the opcode always is in line with the msb of the column
    // there is always a full vertical space
    // the colum width can vary from 1-4
    //

    // seek to EOF
    // seek to beginning of line
    // define all columns
    // use column width to parse the file

    fp.seek(SeekFrom::End(0)).unwrap();

    loop {
        let mut state = Parser::RESET;
        let mut cidx: usize = 0;

        match fp.read(&mut buf[..]) {
            Ok(x) => {
                match x {
                    0 => {
                        println!("Reached EOF");
                        break;
                    }
                    n => {
                        for i in 0..n {
                            match buf[i] {
                                b' ' => {
                                    match state {
                                        Parser::DIGIT => {
                                            //println!("Push bint to column {}", cidx);
                                            columns[cidx].ints.push(String::from_utf8(bint.clone()).unwrap().parse::<u64>().unwrap());
                                            bint.clear();
                                            cidx += 1;
                                        }
                                        Parser::OPCODE => {
                                            cidx += 1;
                                        }
                                        _ => {
                                            //println!("Noop");
                                        }
                                    }
                                    //println!("Increment cidx {}", cidx);
                                    state = Parser::SPACE;
                                }
                                b'\n' => {
                                    match state {
                                        Parser::DIGIT => {
                                            //println!("Push bint to column {}", cidx);
                                            columns[cidx].ints.push(String::from_utf8(bint.clone()).unwrap().parse::<u64>().unwrap());
                                            println!("{:?}", columns[cidx].ints);
                                            bint.clear();
                                            cidx = 0;
                                        }
                                        _ => {
                                            cidx = 0;
                                        }
                                    }
                                    state = Parser::RESET;
                                }
                                b'+' => {
                                    state = Parser::OPCODE;
                                    columns[cidx].op = ColumnOp::ADD;
                                    //println!("Set opcode ADD for column {:?}", columns[cidx]);
                                }
                                b'*' => {
                                    state = Parser::OPCODE;
                                    columns[cidx].op = ColumnOp::MUL;
                                    //println!("Set opcode MUL for column {:?}", columns[cidx]);
                                }
                                48..58 => {
                                    //println!("Found digit {}", buf[i]);
                                    match state {
                                        Parser::RESET | Parser::SPACE => {
                                            if columns.len() < cidx+1 {
                                                //println!("Pushing new column!!");
                                                columns.push(Column::new());
                                            }
                                        }
                                        _ => {
                                            //println!("Noop");
                                        }
                                    }
                                    state = Parser::DIGIT;
                                    //println!("Pushing {} to bint", buf[i]);
                                    bint.push(buf[i]);
                                }
                                _ => {
                                    println!("Ignoring char {}", buf[i]);
                                }
                            }
                        }
                    }
                }
            }
            Err(x) => panic!("Err {:?}", x),
        }
    }
    for c in columns {
        println!("{:?}", c);
    }
    //let mut ans: u128 = 0;
    //for c in columns {
    //    ans += c.calculate();
    //}
    //println!("Ans {}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

}
