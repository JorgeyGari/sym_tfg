use pest::Parser;
use pest_derive::Parser;
use std::fs;
mod polynomial;

#[derive(Parser)]
#[grammar = "poly copy.pest"]
pub struct PolyParser;

fn main() {
    let unparsed_file = fs::read_to_string("input.txt").unwrap();

    let file = PolyParser::parse(Rule::file, &unparsed_file)
        .expect("unsuccessful parse")
        .next()
        .unwrap();

    for line in file.into_inner() {
        println!("LINE: {}", line.as_str());
        let mut p = polynomial::Polynomial { terms: Vec::new() };
        match line.as_rule() {
            Rule::polynomial => {
                for part in line.into_inner() {
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
                                        term.coefficient *=
                                            factor.as_str().trim().parse::<f64>().unwrap();
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
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
        for element in &p.terms {
            println!("{:?}", element);
        }

        println!("{}", p.pprint());
        p.simplify();
        println!("{}", p.pprint());
    }
}
