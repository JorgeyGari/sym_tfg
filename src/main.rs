use lazy_static::lazy_static;
use pest::iterators::Pairs;
use pest::pratt_parser::{Assoc, Op, PrattParser};
use pest::Parser;
use pest_derive::Parser;
use std::fs;
use std::ops::Add;

mod polynomial;

#[derive(Parser)]
#[grammar = "poly.pest"]
pub struct PolyParser;

lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use Assoc::*;
        use Rule::*;

        PrattParser::new()
            .op(Op::infix(add, Left)) // | Op::infix(subtract, Left))
            //.op(Op::infix(multiply, Left) | Op::infix(divide, Left))
    };
}

fn parse_polynomial(expression: Pairs<Rule>) -> polynomial::Polynomial {
    let mut p = polynomial::Polynomial { terms: Vec::new() };
    for part in expression {
        match part.as_rule() {
            Rule::term => {
                let mut term = polynomial::Term {
                    coefficient: 1.0,
                    variables: Vec::new(),
                };
                for factor in part.into_inner() {
                    match factor.as_rule() {
                        Rule::sign => {
                            if factor.as_str() == "-" {
                                term.coefficient *= -1.0;
                            }
                        }
                        Rule::number => {
                            term.coefficient *= factor.as_str().trim().parse::<f64>().unwrap();
                        }
                        Rule::var => {
                            let variable = polynomial::Variable {
                                name: factor.as_str().to_string(),
                                degree: 1,
                            };
                            term.variables.push(variable);
                        }
                        Rule::EOI => (),
                        _ => unreachable!(),
                    }
                }
                p.terms.push(term);
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
    p
}

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
                let p = parse_polynomial(line.into_inner());
                p.pprint();
            }
            Rule::expr => {
                let mut iter = line.into_inner();
                let left_poly = parse_polynomial(iter.next().unwrap().into_inner());
                let _op = iter.next().unwrap();
                let right_poly = parse_polynomial(iter.next().unwrap().into_inner());

                // FIXME: Performs the operation only on two polynomials
                let result = left_poly.add(right_poly);
                println!("Result: {:?}", result.pprint());
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
}
