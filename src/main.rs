use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::Parser;
use std::fs;

mod polynomial;

#[derive(Parser)]
#[grammar = "poly.pest"]
pub struct PolyParser;

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
                p.as_string();
            }
            Rule::expr => {
                let mut iter = line.into_inner();
                let first_poly = parse_polynomial(iter.next().unwrap().into_inner());
                let mut result = first_poly;

                while let Some(op) = iter.next() {
                    let next_poly = parse_polynomial(iter.next().unwrap().into_inner());
                    match op.as_rule() {
                        Rule::add => result = result + next_poly,
                        Rule::sub => result = result - next_poly,
                        Rule::mul => result = result * next_poly,
                        Rule::div => result = result / next_poly,
                        _ => unreachable!(),
                    }
                }

                println!("Result: {:?}", result.as_string());
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
}
