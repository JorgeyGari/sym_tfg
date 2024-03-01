use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Eq, PartialOrd, Ord)]
pub struct Variable {
    pub name: String,
    pub degree: i32,
}

impl PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.degree == other.degree
    }
}

#[derive(Debug, Clone)]
pub struct Term {
    pub coefficient: f64,
    pub variables: Vec<Variable>,
}

impl Term {
    /// Sorts the variables in the term in ascending order based on their names.
    pub fn sort_vars(&mut self) {
        self.variables.sort_by(|a, b| a.name.cmp(&b.name));
    }

    /// Factors the term by combining like variables.
    pub fn factor(&mut self) {
        let mut new_vars: Vec<Variable> = Vec::new();
        for var1 in &self.variables {
            let mut found = false;
            for var2 in &mut new_vars {
                if var1.name == var2.name {
                    var2.degree += var1.degree;
                    found = true;
                    break;
                }
            }
            if !found {
                new_vars.push(var1.clone());
            }
        }
        self.variables = new_vars;
    }
}

#[derive(Debug, Clone)]
pub struct Polynomial {
    pub terms: Vec<Term>,
}

impl Polynomial {
    /// Sorts the terms in the polynomial in descending order based on the max degree of the variables in each term, then by alphabetical order.
    pub fn sort_terms(&mut self) {
        self.terms.sort_by(|a, b| {
            let max_degree_cmp = b
                .variables
                .iter()
                .map(|v| v.degree)
                .max()
                .unwrap_or(0)
                .cmp(&a.variables.iter().map(|v| v.degree).max().unwrap_or(0));
            if max_degree_cmp != Ordering::Equal {
                return max_degree_cmp;
            }
            a.variables.cmp(&b.variables)
        });
    }

    /// Prints the polynomial in a pretty format.
    pub fn pprint(&self) -> String {
        let mut result = String::new();
        for (i, term) in self.terms.iter().enumerate() {
            if term.coefficient == 0.0 {
                continue;
            }
            if i != 0 && term.coefficient > 0.0 {
                result.push_str("+");
            }
            if term.coefficient != 1.0 {
                result.push_str(&term.coefficient.to_string());
            }
            for variable in &term.variables {
                result.push_str(&variable.name);
                if variable.degree != 1 {
                    result.push_str(&format!("^{}", variable.degree));
                }
            }
        }
        result
    }

    /// Adds like terms in the polynomial.
    pub fn add_like_terms(&mut self) -> () {
        let mut new_terms: Vec<Term> = Vec::new();

        for term in &self.terms {
            let coeff: f64 = term.coefficient.clone();
            let mut found = false;

            for term1 in &mut new_terms {
                if term1.variables == term.variables {
                    term1.coefficient += term.coefficient;
                    found = true;
                    break;
                }
            }

            if !found {
                new_terms.push(Term {
                    coefficient: coeff,
                    variables: term.variables.clone(),
                });
            }
        }

        self.terms = new_terms;
    }

    /// Simplifies the polynomial by sorting the terms, sorting the variables in each term, factoring each term, and adding like terms.
    pub fn simplify(&mut self) {
        for term in &mut self.terms {
            term.sort_vars();
        }

        for term in &mut self.terms {
            term.factor();
        }

        self.add_like_terms();

        self.sort_terms();
    }
}

impl Add for Polynomial {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut result = self.terms.clone();
        result.extend(other.terms);
        let mut sum = Polynomial { terms: result };
        sum.simplify();
        sum
    }
}

impl Sub for Polynomial {
    type Output = Self;

    fn sub(self, mut other: Self) -> Self {
        for term in &mut other.terms {
            term.coefficient *= -1.0;
        }

        let mut result = self.terms.clone();
        result.extend(other.terms);

        let mut difference = Polynomial { terms: result };
        difference.simplify();
        difference
    }
}

impl Mul for Polynomial {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut result = Vec::new();
        for term1 in &self.terms {
            for term2 in &other.terms {
                let mut new_vars = term1.variables.clone();
                new_vars.extend(term2.variables.clone());
                let mut new_term = Term {
                    coefficient: term1.coefficient * term2.coefficient,
                    variables: new_vars,
                };
                new_term.sort_vars();
                new_term.factor();
                result.push(new_term);
            }
        }
        let mut product = Polynomial { terms: result };
        product.simplify();
        product
    }
}

impl Div for Polynomial {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        if other.terms.is_empty() {
            panic!("Division by zero");
        }

        let mut result = Polynomial { terms: Vec::new() };

        let mut dividend = Polynomial {
            terms: self.terms.clone(),
        };
        let divisor = Polynomial {
            terms: other.terms.clone(),
        };

        while !dividend.terms.is_empty()
            && dividend.terms[0].variables.len() >= divisor.terms[0].variables.len()
        {
            let mut term = Term {
                coefficient: dividend.terms[0].coefficient / divisor.terms[0].coefficient,
                variables: Vec::new(),
            };

            for (var1, var2) in dividend.terms[0]
                .variables
                .iter()
                .zip(&divisor.terms[0].variables)
            {
                if var1.name != var2.name || var1.degree < var2.degree {
                    panic!("Polynomial division results in non-integral coefficients");
                }

                let new_degree = var1.degree - var2.degree;
                term.variables.push(Variable {
                    name: var1.name.clone(),
                    degree: new_degree,
                });
            }

            result.terms.push(term.clone());

            let subtraction_term = Polynomial {
                terms: vec![term.clone()],
            } * divisor.clone();

            dividend = dividend - subtraction_term;
            dividend.simplify();
        }

        result.simplify();
        result
    }
}
