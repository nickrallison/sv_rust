mod test;

use pest::consumes_to;
use pest::{Parser, parses_to};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "sv.pest"]
pub struct SVParser;

fn main() {

    let input: &str = "module parity_using_bitwise (
    input   wire [7:0] data_in    , //  8 bit data in
    output  wire       parity_out   //  1 bit parity out
    );
    //--------------Code Starts Here-----------------------
    assign parity_out = ^data_in;

endmodule";
    let test_parse = SVParser::parse(Rule::module_def , input);

    println!("{:?}", test_parse);


    // match rule {
    //     Some(r) => println!("{:?}", r),
    //     None => panic!("Rule not found"),
    // }

}
