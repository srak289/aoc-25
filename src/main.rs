use clap::{Command, arg, value_parser};

mod d1;
mod d2;
mod d3;
mod d4;

fn main() {
    let ap = Command::new("aoc")
        .version("0.1.0")
        .about("Runs aoc answers")
        .arg_required_else_help(true)
        .arg(arg!(<module> "The module to run").value_parser(value_parser!(String)))
        .get_matches();
    match ap
        .get_one::<String>("module")
        .expect("Module is required")
        .as_str()
    {
        "d1" => {
            d1::run();
        }
        "d2" => {
            d2::run();
        }
        "d3" => {
            d3::run();
        }
        "d4" => {
            d4::run();
        }
        x => {
            panic!("No module {}", x);
        }
    }
}
