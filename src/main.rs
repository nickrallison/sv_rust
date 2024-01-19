mod test;
mod module_serde;

use std::fs;
use pest::consumes_to;
use pest::{Parser, parses_to};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "sv.pest"]
pub struct SVParser;



fn main() {

    let test_str = "assign y = ~a & ~b & ~c | a & ~b & ~c | a & ~b & c;";
    let test_parse = SVParser::parse(Rule::assignment , test_str);
    println!("{:?}", test_parse);

    let files = fs::read_dir("examples").unwrap();

    for file in files {
        let file = file.unwrap();
        let path = file.path();
        let file_name = path.to_str().unwrap();
        let file: &str = &fs::read_to_string(file_name).expect("Unable to read file");
        let file_parse = SVParser::parse(Rule::file , file);

        if !file_parse.is_ok() {
            println!("Parse failed for file: {}", file_name);
            println!("{:?}", file_parse);
            panic!();
        }
    }
}
