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
    /// Find max degree of the variables in the term.
    pub fn max_degree(&self) -> i32 {
        self.variables.iter().map(|v| v.degree).max().unwrap_or(0)
    }

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
        new_vars.retain(|v| v.degree != 0);
        self.variables = new_vars;
    }
}

impl Div for Term {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        let mut result = Term {
            coefficient: self.coefficient / other.coefficient,
            variables: Vec::new(),
        };
        for var1 in &self.variables {
            let mut found = false;
            for var2 in &other.variables {
                if var1.name == var2.name {
                    result.variables.push(Variable {
                        name: var1.name.clone(),
                        degree: var1.degree - var2.degree,
                    });
                    found = true;
                    break;
                }
            }
            if !found {
                let inv_var = Variable {
                    name: var1.name.clone(),
                    degree: -var1.degree,
                };
                result.variables.push(inv_var);
            }
        }
        result
    }
}

#[derive(Debug, Clone)]
pub struct Polynomial {
    pub terms: Vec<Term>,
}

impl Polynomial {
    /// Return the degree of the polynomial.
    pub fn degree(&self) -> i32 {
        self.terms.iter().map(|t| t.max_degree()).max().unwrap_or(0)
    }

    /// List each term in the polynomial.
    pub fn _print(&self) {
        for term in &self.terms {
            println!("{:?}", term);
        }
    }

    /// Return the leading term of the polynomial.
    pub fn leading_term(&self) -> Term {
        let mut max_degree = 0;
        let mut leading_term = Term {
            coefficient: 0.0,
            variables: Vec::new(),
        };
        for term in &self.terms {
            let degree = term.max_degree();
            if degree > max_degree {
                max_degree = degree;
                leading_term = term.clone();
            }
        }
        leading_term
    }

    /// Evaluate the polynomial at a given value for the variables.
    pub fn evaluate(&mut self, values: &Vec<(String, f64)>) -> () {
        let mut result = Polynomial { terms: Vec::new() };
        for term in &self.terms {
            let mut new_term = term.clone();
            for var in &mut new_term.variables {
                if let Some(val) = values.iter().find(|(name, _)| name == &var.name) {
                    new_term.coefficient *= val.1.powi(var.degree as i32);
                    var.degree = 0; // Set the degree of the variable to 0, essentially removing it from the term
                }
            }
            result.terms.push(new_term);
        }
        *self = result;
        self.simplify();
    }

    /// Sorts the terms in the polynomial in descending order based on the max degree of the variables in each term, then by alphabetical order.
    pub fn sort_terms(&mut self) -> () {
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

    /// Converts the polynomial to a string in a pretty format.
    pub fn as_string(&self) -> String {
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
                    result.push_str(&format!("^({})", variable.degree));
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

        self.add(other)
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
        let mut result = Vec::new();
        for term1 in &self.terms {
            for term2 in &other.terms {
                let mut new_vars = term2.variables.clone();
                for var in &mut new_vars {
                    var.degree *= -1;
                }
                new_vars.extend(term1.variables.clone());
                let mut new_term = Term {
                    coefficient: term1.coefficient / term2.coefficient,
                    variables: new_vars,
                };
                new_term.sort_vars();
                new_term.factor();
                result.push(new_term);
            }
        }
        let mut quotient = Polynomial { terms: result };
        quotient.simplify();
        quotient
    }
}
