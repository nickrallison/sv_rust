use std::fs;
use std::path::PathBuf;
use pest::error::Error;
use pest::iterators::Pairs;
use pest::Parser;
use rstest::rstest;
use crate::{Rule, SVParser};


/*
output = { "output" }

reg = { "reg" }
wire = { "wire" }
logic = { "logic" }
parameter = { "parameter" }
localparam = { "localparam" }
genvar = { "genvar" }
integer_kw = { "integer" }

always = { "always" }
always_comb = { "always_comb" }
always_ff = { "always_ff" }
always_latch = { "always_latch" }
assign = { "assign" }

case = { "case" }
endcase = { "endcase" }

if_kw = { "if" }
else_kw = { "else" }

begin = { "begin" }
end = { "end" }

posedge = { "posedge" }
negedge = { "negedge" }

for_kw = { "for" }
while_kw = { "while" }

generate = { "generate" }
endgenerate = { "endgenerate" }

identifier = @{ (ASCII_ALPHANUMERIC | "_")+ }

string = { "\"" ~ inner ~ "\"" }
inner = { char* }
char = {
    !("\"" | "\\") ~ ANY
    | "\\" ~ ("\"" | "\\" | "/" | "b" | "f" | "n" | "r" | "t")
    | "\\" ~ ("u" ~ ASCII_HEX_DIGIT{4})
}

net = {integer? ~ apos ~ ("b" | "B" | "o" | "O" | "d" | "D" | "h" | "H") ~ (ASCII_DIGIT | "x" | "X" | "z" | "Z")+}
number = {
             "-"?
             ~ ("0" | ASCII_NONZERO_DIGIT ~ ASCII_DIGIT*)
             ~ ("." ~ ASCII_DIGIT*)?
             ~ (^"e" ~ ("+" | "-")? ~ ASCII_DIGIT+)?
         }

// binary_operator = { "==" | "<=" | ">=" | "!=" | "+=" | "-=" | "*=" | "/=" | "&&" | "||" | "<<" | ">>" | "=" | "<" | ">" | "!" | "~" | "&" | "|" | "^" | "+" | "-" | "*" | "/" | "%" | "**" }
// unary_operator = {"&" | "|" | "^" | "~|" | "~&"}

left_paren = { "(" }
right_paren = { ")" }
left_bracket = { "[" }
right_bracket = { "]" }
left_brace = { "{" }
right_brace = { "}" }
colon = { ":" }
semicolon = { ";" }
comma = { "," }
apos = { "'" }
integer = { ASCII_DIGIT+ }


// ############################

value_p1 = { net | number }
value = { value_p1 | identifier | ("(" ~ expression ~ ")") }

lognegation_expression = {"!" ~ (value | lognegation_expression)}
bitnegation_expression = {"~" ~ (value | lognegation_expression | bitnegation_expression)}
reduceand_expression = {"&" ~ (value | lognegation_expression | bitnegation_expression)}
reduceor_expression = {"|" ~ (value | lognegation_expression | bitnegation_expression | reduceand_expression)}
reducenand_expression = {"~&" ~ (value | lognegation_expression | bitnegation_expression | reduceand_expression | reduceor_expression)}
reducenor_expression = {"~|" ~ (value | lognegation_expression | bitnegation_expression | reduceand_expression | reduceor_expression | reducenand_expression)}
reducexor_expression = {"^" ~ (value | lognegation_expression | bitnegation_expression | reduceand_expression | reduceor_expression | reducenand_expression | reducenor_expression)}
reducexnor_expression = {("~^" | "^~") ~ (value | lognegation_expression | bitnegation_expression | reduceand_expression | reduceor_expression | reducenand_expression | reducenor_expression | reducexor_expression)}
unary_plus_expression = {"+" ~ (value | lognegation_expression | bitnegation_expression | reduceand_expression | reduceor_expression | reducenand_expression | reducenor_expression | reducexor_expression | reducexnor_expression)}
unary_minus_expression = {"-" ~ (value | lognegation_expression | bitnegation_expression | reduceand_expression | reduceor_expression | reducenand_expression | reducenor_expression | reducexor_expression | reducexnor_expression | unary_plus_expression)}
unary_expression = { (value | lognegation_expression | bitnegation_expression | reduceand_expression | reduceor_expression | reducenand_expression | reducenor_expression | reducexor_expression | reducexnor_expression | unary_plus_expression | unary_minus_expression) }

mult_expression = {value ~ ("*" ~ value)+}
divide_expression = {(value | mult_expression) ~ ("/" ~ (value | mult_expression))+}
modulus_expression = {(value | mult_expression | divide_expression) ~ ("%" ~ (value | mult_expression | divide_expression))+}
plus_expression = {(value | mult_expression | divide_expression | modulus_expression) ~ ("+" ~ (value | mult_expression | divide_expression | modulus_expression))+}
minus_expression = {(value | mult_expression | divide_expression | modulus_expression | plus_expression) ~ ("-" ~ (value | mult_expression | divide_expression | modulus_expression | plus_expression))+}
shift_expression = {(value | mult_expression | divide_expression | modulus_expression | plus_expression | minus_expression) ~ (("<<" | ">>") ~ (value | mult_expression | divide_expression | modulus_expression | plus_expression | minus_expression))+}
bitand_expression = {(value | mult_expression | divide_expression | modulus_expression | plus_expression | minus_expression | shift_expression) ~ ("&" ~ (value | mult_expression | divide_expression | modulus_expression | plus_expression | minus_expression | shift_expression))+}
bitxor_expression = {(value | mult_expression | divide_expression | modulus_expression | plus_expression | minus_expression | shift_expression | bitand_expression) ~ ("^" ~ (value | mult_expression | divide_expression | modulus_expression | plus_expression | minus_expression | shift_expression | bitand_expression))+}
bitxnor_expression = {(value | mult_expression | divide_expression | modulus_expression | plus_expression | minus_expression | shift_expression | bitand_expression | bitxor_expression) ~ ("~^" ~ (value | mult_expression | divide_expression | modulus_expression | plus_expression | minus_expression | shift_expression | bitand_expression | bitxor_expression))+}
bitor_expression = {(value | mult_expression | divide_expression | modulus_expression | plus_expression | minus_expression | shift_expression | bitand_expression | bitxor_expression | bitxnor_expression) ~ ("|" ~ (value | mult_expression | divide_expression | modulus_expression | plus_expression | minus_expression | shift_expression | bitand_expression | bitxor_expression | bitxnor_expression))+}
logicand_expression = {(value | mult_expression | divide_expression | modulus_expression | plus_expression | minus_expression | shift_expression | bitand_expression | bitxor_expression | bitxnor_expression | bitor_expression) ~ ("&&" ~ (value | mult_expression | divide_expression | modulus_expression | plus_expression | minus_expression | shift_expression | bitand_expression | bitxor_expression | bitxnor_expression | bitor_expression))+}
logicor_expression = {(value | mult_expression | divide_expression | modulus_expression | plus_expression | minus_expression | shift_expression | bitand_expression | bitxor_expression | bitxnor_expression | bitor_expression | logicand_expression) ~ ("||" ~ (value | mult_expression | divide_expression | modulus_expression | plus_expression | minus_expression | shift_expression | bitand_expression | bitxor_expression | bitxnor_expression | bitor_expression | logicand_expression))+}
binary_expression = { (mult_expression | divide_expression | modulus_expression | plus_expression | minus_expression | shift_expression | bitand_expression | bitxor_expression | bitxnor_expression | bitor_expression | logicand_expression | logicor_expression) }

expression = {value | unary_expression | binary_expression}

module_io = {"." ~ identifier ~ "(" ~ identifier ~ "),?"}
module = {module_kw ~ ("#(" ~ module_io+ ~ ")")? ~ identifier ~ "(" ~ module_io* ~ ")" ~ ";"}

// value = {(unary_operator)
//  | ()}

assign_statement = {assign ~ identifier ~ "=" ~ expression ~ ";"}
test = { assign ~ identifier ~ "=" ~ expression }

 */
fn string_to_rule(rule: &str) -> Rule {
    // From Above Table
    return match rule {
        // "module_kw" => Rule::module_kw,
        // "output" => Rule::output,
        // "reg" => Rule::reg,
        // "wire" => Rule::wire,
        // "logic" => Rule::logic,
        // "parameter" => Rule::parameter,
        // "localparam" => Rule::localparam,
        // "genvar" => Rule::genvar,
        // "integer_kw" => Rule::integer_kw,
        // "always" => Rule::always,
        // "always_comb" => Rule::always_comb,
        // "always_ff" => Rule::always_ff,
        // "always_latch" => Rule::always_latch,
        // "assign" => Rule::assign,
        // "case" => Rule::case,
        // "endcase" => Rule::endcase,
        // "if_kw" => Rule::if_kw,
        // "else_kw" => Rule::else_kw,
        // "begin" => Rule::begin,
        // "end" => Rule::end,
        // "posedge" => Rule::posedge,
        // "negedge" => Rule::negedge,
        // "for_kw" => Rule::for_kw,
        // "while_kw" => Rule::while_kw,
        // "generate" => Rule::generate,
        // "endgenerate" => Rule::endgenerate,
        // "identifier" => Rule::identifier,
        // "string" => Rule::string,
        // "inner" => Rule::inner,
        // "char" => Rule::char,
        // "net" => Rule::net,
        // "number" => Rule::number,
        // "left_paren" => Rule::left_paren,
        // "right_paren" => Rule::right_paren,
        // "left_bracket" => Rule::left_bracket,
        // "right_bracket" => Rule::right_bracket,
        // "left_brace" => Rule::left_brace,
        // "right_brace" => Rule::right_brace,
        // "colon" => Rule::colon,
        // "semicolon" => Rule::semicolon,
        // "comma" => Rule::comma,
        // "apos" => Rule::apos,
        // "integer" => Rule::integer,
        // "value_p1" => Rule::value_p1,
        // "value" => Rule::value,
        // "lognegation_expression" => Rule::lognegation_expression,
        // "bitnegation_expression" => Rule::bitnegation_expression,
        // "reduceand_expression" => Rule::reduceand_expression,
        // "reduceor_expression" => Rule::reduceor_expression,
        // "reducenand_expression" => Rule::reducenand_expression,
        // "reducenor_expression" => Rule::reducenor_expression,
        // "reducexor_expression" => Rule::reducexor_expression,
        // "reducexnor_expression" => Rule::reducexnor_expression,
        // "unary_plus_expression" => Rule::unary_plus_expression,
        // "unary_minus_expression" => Rule::unary_minus_expression,
        // "unary_expression" => Rule::unary_expression,
        //
        // "mult_expression" => Rule::mult_expression,
        // "divide_expression" => Rule::divide_expression,
        // "modulus_expression" => Rule::modulus_expression,
        // "plus_expression" => Rule::plus_expression,
        // "minus_expression" => Rule::minus_expression,
        // "shift_expression" => Rule::shift_expression,
        // "bitand_expression" => Rule::bitand_expression,
        // "bitxor_expression" => Rule::bitxor_expression,
        // "bitxnor_expression" => Rule::bitxnor_expression,
        // "bitor_expression" => Rule::bitor_expression,
        // "logicand_expression" => Rule::logicand_expression,
        // "logicor_expression" => Rule::logicor_expression,
        // "binary_expression" => Rule::binary_expression,
        //
        // "expression" => Rule::expression,
        // "module_io" => Rule::module_io,
        // "module" => Rule::module,
        // "assign_statement" => Rule::assign_statement,


        _ => panic!("Rule not found: {rule}")
    }
}

fn test_file<'i>(rule: String, input: String) -> bool {
    let rule = string_to_rule(&rule);
    let input = input.as_str();
    let parse = SVParser::parse(rule, input);
    return parse.is_ok();
}

#[rstest]
fn for_each_file(#[files("examples/**/*.test")] path: PathBuf) {
    let file = fs::read_to_string(&path).expect("cannot read file");
    let re = regex::Regex::new(r"Test:\s+(\d+)\s+Rule:\s+(\w+)\s+Input:\s+([\s\S]+)$").unwrap();
    let captures = re.captures(&file).unwrap();
    let test_number = captures.get(1).unwrap().as_str().to_string();
    let rule = captures.get(2).unwrap().as_str();
    let input = captures.get(3).unwrap().as_str();
    let parse = SVParser::parse(string_to_rule(rule), input);
    assert!(parse.is_ok(), "Test: {} Rule: {} Input: {}", test_number, rule, input);
}