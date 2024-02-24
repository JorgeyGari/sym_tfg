use pest::Parser;
use pest_derive::Parser;
use std::fs;
mod polynomial;

#[derive(Parser)]
#[grammar = "poly.pest"]
pub struct PolyParser;

fn main() {
    let unparsed_file = fs::read_to_string("input.txt").unwrap();

    let file = PolyParser::parse(Rule::file, &unparsed_file)
        .expect("unsuccessful parse")
        .next()
        .unwrap();

    for line in file.into_inner() {
        println!("LINE: {}", line.as_str());
        match line.as_rule() {
            Rule::polynomial => {
                for part in line.into_inner() {
                    match part.as_rule() {
                        Rule::op => {
                            println!("op: {}", part.as_str());
                        }
                        Rule::term => {
                            let mut term = polynomial::Term {
                                coefficient: 0.0,
                                variables: Vec::new(),
                            };
                            for factor in part.into_inner() {
                                match factor.as_rule() {
                                    Rule::number => {
                                        term.coefficient = match factor
                                            .as_str()
                                            .trim()
                                            .parse::<f64>()
                                        {
                                            Ok(value) => value,
                                            Err(_) => {
                                                eprintln!("Could not parse \"{}\" as a floating point number.", factor.as_str());
                                                return;
                                            }
                                        };
                                    }
                                    Rule::var => {
                                        let variable = polynomial::Variable {
                                            name: String::new(),
                                            degree: 1,
                                        };
                                        term.variables.push(variable);
                                    }
                                    Rule::EOI => (),
                                    _ => unreachable!(),
                                }
                            }
                        }
                        Rule::EOI => (),
                        _ => unreachable!(),
                    }
                }
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
}
