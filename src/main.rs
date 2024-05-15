use core::panic;
use num::rational::Rational64;
use pest::iterators::Pairs;
use pest::Parser;
use pest_derive::Parser;
use polynomial::PolyRatio;
use std::fs;

mod polynomial;

#[derive(Parser)]
#[grammar = "poly.pest"]
pub struct PolyParser;

fn variable_from_string(var: &str) -> polynomial::Variable {
    let mut iter = var.split('^');
    let name = iter.next().unwrap().to_string();
    let degree = iter
        .next()
        .map(|d| {
            let clean = d.replace("(", "").replace(")", "");
            if d.contains('/') {
                let parts: Vec<&str> = clean.split('/').collect();
                let numerator = parts[0].trim().parse::<i64>().unwrap_or(1);
                let denominator = parts[1].trim().parse::<i64>().unwrap_or(1);
                Rational64::new(numerator, denominator)
            } else if d.contains('.') {
                let parts: Vec<&str> = clean.split('.').collect();
                let numerator = clean.replace(".", "").trim().parse::<i64>().unwrap_or(1);
                let denominator = 10_i64.pow(parts[1].len() as u32);
                Rational64::new(numerator, denominator)
            } else {
                clean.trim().parse::<i64>().unwrap_or(1).into()
            }
        })
        .unwrap_or(1.into());
    polynomial::Variable { name, degree }
}

fn parse_polynomial(expression: Pairs<Rule>) -> polynomial::Polynomial {
    let mut p = polynomial::Polynomial {
        terms: Vec::new(),
        degree: 1.into(),
    };
    for part in expression {
        match part.as_rule() {
            Rule::term => {
                let mut term = polynomial::Term {
                    coefficient: Rational64::new(1, 1),
                    variables: Vec::new(),
                };
                for factor in part.into_inner() {
                    match factor.as_rule() {
                        Rule::sign => {
                            if factor.as_str() == "-" {
                                term.coefficient *= -1;
                            }
                        }
                        Rule::number => {
                            term.coefficient *=
                                factor.as_str().trim().parse::<Rational64>().unwrap();
                        }
                        Rule::fraction => {
                            let mut iter = factor.into_inner();
                            let numerator =
                                iter.next().unwrap().as_str().trim().parse::<i64>().unwrap();
                            let denominator =
                                iter.next().unwrap().as_str().trim().parse::<i64>().unwrap();
                            term.coefficient *= Rational64::new(numerator, denominator);
                        }
                        Rule::var => {
                            let variable = variable_from_string(factor.as_str());
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

fn parse_assignment(assignment: Pairs<Rule>) -> (String, Rational64) {
    // Only assigning Rational64 values?
    let mut iter = assignment;
    let var_name = iter.next().unwrap().as_str().to_string();
    let var_value = iter
        .next()
        .unwrap()
        .as_str()
        .trim()
        .parse::<Rational64>()
        .unwrap();
    (var_name, var_value)
}

fn parse_operation(operation: Pairs<Rule>) -> polynomial::PolyRatio {
    let mut iter = operation;
    let first_poly = parse_polynomial(iter.next().unwrap().into_inner());
    let mut result = PolyRatio::from(first_poly);

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
    result
}

fn main() {
    // Test simplify fractions
    // let mut p = Polynomial {
    //     terms: vec![
    //         polynomial::Term {
    //             coefficient: Rational64::new(3, 1),
    //             variables: vec![polynomial::Variable {
    //                 name: "x".to_string(),
    //                 degree: -1,
    //             }],
    //         },
    //         polynomial::Term {
    //             coefficient: Rational64::new(4, 1),
    //             variables: vec![polynomial::Variable {
    //                 name: "x".to_string(),
    //                 degree: 1,
    //             }],
    //         },
    //     ],
    // };
    // p.simplify();
    // println!("Polynomial p: {}", p.as_string());

    // let mut q = Polynomial {
    //     terms: vec![polynomial::Term {
    //         coefficient: Rational64::new(6, 1),
    //         variables: vec![polynomial::Variable {
    //             name: "x".to_string(),
    //             degree: 1,
    //         }],
    //     }],
    // };

    // q.simplify();
    // println!("Polynomial q: {}", q.as_string());

    // let mut r = PolyRatio {
    //     numerator: p,
    //     denominator: q,
    // };

    // println!("Ratio: {}", r.as_string());

    // r.simplify();
    // println!("ratio (simplified): {}", r.as_string());

    // Test squared polynomial
    // let mut not_ab_squared = polynomial::Polynomial {
    //     terms: vec![
    //         polynomial::Term {
    //             coefficient: Rational64::new(1, 1),
    //             variables: vec![polynomial::Variable {
    //                 name: "a".to_string(),
    //                 degree: 1.into(),
    //             }],
    //         },
    //         polynomial::Term {
    //             coefficient: Rational64::new(1, 1),
    //             variables: vec![polynomial::Variable {
    //                 name: "b".to_string(),
    //                 degree: 1.into(),
    //             }],
    //         },
    //     ],
    //     degree: 2.into(),
    // };
    // not_ab_squared.simplify();
    // println!("(a + b)^2 = {}", not_ab_squared.as_string());

    let unparsed_file = fs::read_to_string("input.txt").unwrap();

    let file = PolyParser::parse(Rule::file, &unparsed_file)
        .expect("unsuccessful parse")
        .next()
        .unwrap();

    let mut var_values: Vec<(String, Rational64)> = Vec::new(); // Vector to store the values of the variables

    for line in file.into_inner() {
        if line.as_str().trim().is_empty() {
            continue; // Skip empty lines
        }

        println!("{}", line.as_str());
        match line.as_rule() {
            Rule::assign => {
                let (var_name, var_value) = parse_assignment(line.into_inner());
                var_values.push((var_name.clone(), var_value));

                println!("\t{} = {}", var_name, var_value);
            }
            Rule::polynomial => {
                let mut p = parse_polynomial(line.into_inner());
                p.evaluate(&var_values);
                println!("\t{}", p.as_string());
                println!("{:?}", p);
            }
            Rule::operation => {
                let mut result = parse_operation(line.into_inner());
                result.evaluate(&var_values);
                println!("\t{}", result.as_string());
                // println!("{:?}", result);
            }
            Rule::solve => {
                let mut iter = line.into_inner();
                println!("Solving...");
                let p = parse_polynomial(iter.next().unwrap().into_inner());
                println!("{:?}", p);
                if let Some(var) = iter.next() {
                    // Variable was specified
                    let variable = var.as_str().to_string();
                    println!("Solving for {}...", variable);
                    panic!("Not implemented");
                } else {
                    // No variable specified
                    let variable = p.first_var().unwrap_or("".to_string());
                    if variable.is_empty() {
                        panic!("No variable to solve for");
                    }
                    println!("Solving...");
                    let result = p.roots(&variable);
                    // Print all the roots as strings
                    for r in result {
                        println!("{}", r.as_string());
                    }
                    // panic!("Not implemented");
                };
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
}
