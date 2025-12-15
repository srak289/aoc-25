use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Eq)]
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
    width: usize, 
}

impl Column {
    fn new(op: ColumnOp, width: usize) -> Self {
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

#[allow(unused_variables, unused_assignments)]
fn do_opline(columns: &mut Vec<Column>, line: &String) {
    let mut op = ColumnOp::NOOP;
    let mut width: usize = 0;
    let mut state = Parser::RESET;

    for c in line.chars() {
        match c {
            '*' | '+' => {
                if state == Parser::SPACE {
                    columns.push(Column::new(op, width));
                    width = 0;
                }
                op = if c == '*' { ColumnOp::MUL } else { ColumnOp::ADD };
                state = Parser::OPCODE;
                width += 1;
            }
            ' ' => {
                state = Parser::SPACE;
                width += 1;
            }
            _ => {}
        }
    }
    // append the last column
    columns.push(Column::new(op, width));
    //println!("{:?}", columns);
}

#[allow(unused_variables, unused_assignments)]
fn do_line(columns: &mut Vec<Column>, line: &String) {
    let mut idx: usize = 0;
    let mut bint = Vec::<char>::new();
    let mut state = Parser::RESET;

    for c in columns {
        // read width from line
        for i in line[idx..idx+c.width].chars() {
            match i {
                ' ' => {
                    //println!("Found space");
                    state = Parser::SPACE;
                }
                '0'..='9' => {
                    //println!("Pushing digit {}", i);
                    state = Parser::DIGIT;
                    bint.push(i)
                }
                _ => {}
            }
        }
        let s = String::from_iter(bint.clone());
        //println!("Got string {}", s);
        c.ints.push(s.parse::<u64>().unwrap());
        bint.clear();
        idx += c.width;
        //println!("New idx {}", idx);
    }
    //println!("{:?}", line);
}

pub fn run() {
    let mut reader = BufReader::new(File::open("trash.txt").expect("reading file failed"));

    // the opcode always is in line with the msb of the column
    // there is always a full vertical space
    // the colum width can vary from 1-4
    //

    let mut columns = Vec::<Column>::new();
    let mut lines = Vec::<String>::new();
    let mut line = String::new();

    loop {
        match reader.read_line(&mut line) {
            Ok(x) => {
                if x == 0 {
                    break;
                }
                // discard \n
                let _ = line.pop();
                if line.starts_with("*") || line.starts_with("+") {
                    //println!("Parsing operations line");
                    do_opline(&mut columns, &line);
                } else {
                    lines.push(line.clone());
                    line.clear();
                }
            }
            Err(x) => {
                panic!("{:?}", x);
            }
        }
    }

    for l in &lines {
        //println!("Processing line");
        do_line(&mut columns, &l);
    }

    println!("{:?}", columns);

    let mut ans: u128 = 0;
    for c in columns {
        ans += c.calculate();
    }
    println!("Ans {}", ans);

}

#[cfg(test)]
mod tests {
    use super::*;

}
