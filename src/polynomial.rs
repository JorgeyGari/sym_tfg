use num::rational::{Ratio, Rational64};
use num_integer::lcm;
use num_traits::cast::ToPrimitive;
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
    pub coefficient: Rational64,
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

    /// Inverts the term.
    pub fn invert(&mut self) {
        self.coefficient = Rational64::new(*self.coefficient.denom(), *self.coefficient.numer());
        for var in &mut self.variables {
            var.degree *= -1;
        }
    }
}

impl Mul for Term {
    type Output = Term;
    fn mul(self, other: Self) -> Term {
        let mut result = Vec::new();
        let mut new_vars = self.variables.clone();
        new_vars.extend(other.variables.clone());
        let mut new_term = Term {
            coefficient: self.coefficient * other.coefficient,
            variables: new_vars,
        };
        new_term.sort_vars();
        new_term.factor();
        result.push(new_term);
        let mut product = Polynomial { terms: result };
        product.simplify();
        product.terms[0].clone()
    }
}

impl Div for Term {
    type Output = Polynomial;
    fn div(self, other: Self) -> Polynomial {
        let dividend = Polynomial { terms: vec![self] };
        let divisor = Polynomial { terms: vec![other] };
        let mut result = Vec::new();
        for term1 in &dividend.terms {
            for term2 in &divisor.terms {
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

impl PartialEq for Term {
    fn eq(&self, other: &Self) -> bool {
        let mut self_copy = self.clone();
        self_copy.sort_vars();
        let mut other_copy = other.clone();
        other_copy.sort_vars();
        self_copy.coefficient == other_copy.coefficient
            && self_copy.variables == other_copy.variables
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
            coefficient: Rational64::new(0, 1),
            variables: Vec::new(),
        };
        for term in &self.terms {
            let degree = term.max_degree();
            if degree >= max_degree {
                max_degree = degree;
                leading_term = term.clone();
            }
        }
        leading_term
    }

    /// Evaluate the polynomial at a given value for the variables.
    pub fn evaluate(&mut self, values: &Vec<(String, Rational64)>) {
        let mut result = Polynomial { terms: Vec::new() };
        for term in &self.terms {
            let mut new_term = term.clone();
            for var in &mut new_term.variables {
                if let Some(val) = values.iter().find(|(name, _)| name == &var.name) {
                    for _ in 0..var.degree {
                        new_term.coefficient *= val.1;
                    }
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
            if term.coefficient == Rational64::new(0, 1) {
                continue;
            }
            if i != 0 && term.coefficient > Rational64::new(0, 1) {
                result.push_str("+");
            }
            if term.coefficient != Rational64::new(1, 1) {
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
            let coeff: Rational64 = term.coefficient.clone();
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
    pub fn simplify(&mut self) -> () {
        for term in &mut self.terms {
            term.sort_vars();
        }

        for term in &mut self.terms {
            term.factor();
        }

        self.add_like_terms();

        // Filter to remove terms with coefficient 0
        self.terms
            .retain(|term| term.coefficient != Rational64::new(0, 1));

        self.sort_terms();
    }

    /// Multiplies the polynomial by the smallest scalar such that all coefficients are integers. Returns the scalar.
    pub fn make_integer(&mut self) -> i64 {
        // Get the lcm of the denominators of the coefficients
        let mut lcm = 1;
        for term in &self.terms {
            let denom = term.coefficient.denom();
            lcm = num_integer::lcm(lcm as i64, *denom);
        }
        // Multiply each coefficient by the lcm
        for term in &mut self.terms {
            term.coefficient *= Rational64::new(lcm, 1);
        }
        lcm
    }

    /// Finds the greatest common divisor of the coefficients of the terms in a single-variable polynomial with integer coefficients. Returns the gcd and the polynomial with the gcd factored out.
    pub fn factor(&mut self) -> (Term, Polynomial) {
        let mut factored_out = Term {
            coefficient: Rational64::new(1, 1),
            variables: vec![],
        };
        let mut factored = self.clone();

        // Check the name of the variable that appears in all terms
        let mut seen_vars = vec![];
        for var in &self.terms[0].variables {
            seen_vars.push(var.name.clone());
        }
        for term in &self.terms {
            let mut curr_vars = vec![];
            for var in &term.variables {
                curr_vars.push(var.name.clone());
            }
            seen_vars = seen_vars
                .iter()
                .map(|x| x.clone())
                .filter(|x| curr_vars.contains(x))
                .collect();

            if seen_vars.len() == 0 {
                return (factored_out, factored); // No common variable, return the original polynomial
            }
        }
        let var_name = seen_vars[0].clone(); // Always the first element
                                             // TODO: Maybe add an option to this function to specify the variable to factor out

        // Make the coefficients integers
        let mut p = self.clone();
        let adjust = p.make_integer();
        p.simplify();

        // Find the gcd of the coefficients
        let mut gcd = p.terms[0].coefficient.numer().abs();
        for term in &p.terms {
            gcd = num_integer::gcd(gcd, term.coefficient.numer().abs());
        }

        // Find the smallest power of the variable that appears in all terms
        let mut min_degree = p.terms[0].variables[0].degree;
        for term in &p.terms {
            for var in &term.variables {
                if var.degree < min_degree && var.degree > 0 {
                    min_degree = var.degree;
                }
            }
        }

        // Factor out the gcd
        factored_out.coefficient = Rational64::new(gcd, 1);
        factored_out.variables.push(Variable {
            name: var_name,
            degree: min_degree,
        });

        let mut inv: Term = factored_out.clone();
        inv.invert();
        factored = factored * Polynomial { terms: vec![inv] };

        // Undo the scaling of the coefficients
        for term in &mut factored.terms {
            term.coefficient *= Rational64::new(1, adjust);
        }
        factored_out.coefficient *= Rational64::new(1, adjust);

        factored.simplify();

        return (factored_out, factored);
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
            term.coefficient *= -1;
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
    type Output = PolyRatio;
    fn div(self, other: Self) -> PolyRatio {
        let mut dividend = self.clone();
        dividend.simplify();

        if dividend.terms.len() == 0 {
            return PolyRatio::from(Polynomial {
                terms: vec![Term {
                    coefficient: Rational64::new(0, 1),
                    variables: vec![],
                }],
            });
        }

        let mut divisor = other.clone();
        divisor.simplify();

        let mut quotient = Polynomial { terms: vec![] };

        let mut remainder = dividend.clone();

        println!(
            "Remainder: {}\nDivisor: {}",
            remainder.as_string(),
            divisor.as_string()
        );

        let zero_poly = Polynomial {
            terms: vec![Term {
                coefficient: Rational64::new(0, 1),
                variables: vec![],
            }],
        };

        if remainder.degree() < divisor.degree() {
            return PolyRatio {
                numerator: remainder,
                denominator: divisor,
            };
        }

        while remainder != zero_poly
            && remainder.terms.len() != 0
            && remainder.degree() >= divisor.degree()
        // THIS LAST CONDITION IS THE PROBLEM (check what happens with 8/x)
        {
            let t = remainder.leading_term() / divisor.leading_term();
            remainder._print();
            //println!("t: {:?}", t);
            quotient = quotient + t.clone();
            //println!("Quotient: {}", quotient.as_string());
            remainder = remainder - (divisor.clone() * t.clone());
            remainder.simplify();
            //remainder._print();
        }

        quotient.simplify();
        let ratio = PolyRatio::from(quotient);
        ratio
    }
}

impl PartialEq for Polynomial {
    fn eq(&self, other: &Self) -> bool {
        let mut self_copy = self.clone();
        self_copy.simplify();
        let mut other_copy = other.clone();
        other_copy.simplify();
        self.terms == other.terms
    }
}

pub struct PolyRatio {
    pub numerator: Polynomial,
    pub denominator: Polynomial,
}

impl PolyRatio {
    pub fn simplify(&mut self) {
        // Simplify the initial numerator and denominator
        self.numerator.simplify();
        self.denominator.simplify();

        println!("Numerator: {}", self.numerator.as_string());
        println!("Denominator: {}", self.denominator.as_string());

        // Make the coefficients integers
        let mut n = self.numerator.clone();
        let mut d = self.denominator.clone();
        let adjust_n = n.make_integer();
        let adjust_d = d.make_integer();

        println!("Numerator: {}", n.as_string());
        println!("Denominator: {}", d.as_string());

        // Find the smallest negative exponent of each variable in the denominator
        let mut vars_to_move: Vec<Variable> = vec![];
        for term in &d.terms {
            for var in &term.variables {
                if var.degree < 0 {
                    vars_to_move.push(var.clone());
                }
            }
        }
        for var in &mut vars_to_move {
            var.degree *= -1;
        }

        // Multiply the numerator and denominator by the accumulated terms
        n = n * Polynomial {
            terms: vec![Term {
                coefficient: Rational64::new(1, 1),
                variables: vars_to_move.clone(),
            }],
        };
        d = d * Polynomial {
            terms: vec![Term {
                coefficient: Rational64::new(1, 1),
                variables: vars_to_move.clone(),
            }],
        };

        println!("Numer: {}", n.as_string());
        println!("Denom: {}", d.as_string());

        // Find the smallest negative exponent of each variable in the denominator
        let mut vars_to_move: Vec<Variable> = vec![];
        for term in &n.terms {
            for var in &term.variables {
                if var.degree < 0 {
                    vars_to_move.push(var.clone());
                }
            }
        }
        for var in &mut vars_to_move {
            var.degree *= -1;
        }

        // Multiply the numerator and denominator by the accumulated terms
        n = n * Polynomial {
            terms: vec![Term {
                coefficient: Rational64::new(1, 1),
                variables: vars_to_move.clone(),
            }],
        };
        d = d * Polynomial {
            terms: vec![Term {
                coefficient: Rational64::new(1, 1),
                variables: vars_to_move.clone(),
            }],
        };

        println!("Numer: {}", n.as_string());
        println!("Denom: {}", d.as_string());

        // Factor out as much as possible from the numerator and denominator
        let (t1, mut n) = n.factor();
        let (t2, mut d) = d.factor();

        println!("t1: {:?}", t1);
        println!("t2: {:?}", t2);

        // We are going to divide the numerator and denominator, these are the values by default
        let mut var_name = "".to_string();
        let mut min_degree = 0;

        // If the factored out terms share a variable
        if t1.variables.len() != 0 && t2.variables.len() != 0 {
            if t1.variables[0].name == t2.variables[0].name {
                var_name = t1.variables[0].name.clone();
                min_degree = t1.variables[0].degree.min(t2.variables[0].degree);
                println!("Var name: {}", var_name);
                println!("Min degree: {}", min_degree);
            }
        }

        let gcd_term = Term {
            // The term that will be canceled out in the numerator and denominator
            coefficient: Rational64::from_integer(num_integer::gcd(
                t1.coefficient.numer().abs(),
                t2.coefficient.numer().abs(),
            )),
            variables: if var_name != "" {
                // If the terms share a variable
                vec![Variable {
                    name: var_name,
                    degree: min_degree,
                }]
            } else {
                vec![]
            },
        };
        println!("GCD: {:?}", gcd_term);

        n = n * Polynomial { terms: vec![t1] };
        d = d * Polynomial { terms: vec![t2] };

        // Cancel out the gcd from the numerator and denominator
        let mut inv = gcd_term.clone();
        inv.invert();
        println!("Inv: {:?}", inv);
        println!("Numerator: {}", n.as_string());
        n = n * Polynomial {
            terms: vec![inv.clone()],
        };
        d = d * Polynomial { terms: vec![inv] };

        // Undo the scaling of the coefficients
        for term in &mut n.terms {
            term.coefficient *= Rational64::new(1, adjust_n);
        }
        for term in &mut d.terms {
            term.coefficient *= Rational64::new(1, adjust_d);
        }

        self.numerator = n;
        self.denominator = d;

        self.numerator.simplify();
        self.denominator.simplify();
    }

    pub fn as_string(&self) -> String {
        format!(
            "({}) / ({})",
            self.numerator.as_string(),
            self.denominator.as_string()
        )
    }

    pub fn evaluate(&mut self, values: &Vec<(String, Rational64)>) {
        self.numerator.evaluate(values);
        self.denominator.evaluate(values);
    }
}

impl Add for PolyRatio {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut result = PolyRatio {
            numerator: self.numerator.clone() * other.denominator.clone()
                + other.numerator.clone() * self.denominator.clone(),
            denominator: self.denominator.clone() * other.denominator.clone(),
        };
        result.simplify();
        result
    }
}

impl Sub for PolyRatio {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut result = PolyRatio {
            numerator: self.numerator.clone() * other.denominator.clone()
                - other.numerator.clone() * self.denominator.clone(),
            denominator: self.denominator.clone() * other.denominator.clone(),
        };
        result.simplify();
        result
    }
}

impl Mul for PolyRatio {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut result = PolyRatio {
            numerator: self.numerator.clone() * other.numerator.clone(),
            denominator: self.denominator.clone() * other.denominator.clone(),
        };
        result.simplify();
        result
    }
}

impl Div for PolyRatio {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let mut result = PolyRatio {
            numerator: self.numerator.clone() * other.denominator.clone(),
            denominator: self.denominator.clone() * other.numerator.clone(),
        };
        result.simplify();
        result
    }
}

impl From<Polynomial> for PolyRatio {
    fn from(p: Polynomial) -> Self {
        PolyRatio {
            numerator: p,
            denominator: Polynomial {
                terms: vec![Term {
                    coefficient: Rational64::new(1, 1),
                    variables: vec![],
                }],
            },
        }
    }
}

impl Add<PolyRatio> for Polynomial {
    type Output = PolyRatio;

    fn add(self, other: PolyRatio) -> PolyRatio {
        let upgraded_self = PolyRatio::from(self);
        upgraded_self + other
    }
}

impl Sub<PolyRatio> for Polynomial {
    type Output = PolyRatio;

    fn sub(self, other: PolyRatio) -> PolyRatio {
        let upgraded_self = PolyRatio::from(self);
        upgraded_self - other
    }
}

impl Mul<PolyRatio> for Polynomial {
    type Output = PolyRatio;

    fn mul(self, other: PolyRatio) -> PolyRatio {
        let upgraded_self = PolyRatio::from(self);
        upgraded_self * other
    }
}

impl Div<PolyRatio> for Polynomial {
    type Output = PolyRatio;

    fn div(self, other: PolyRatio) -> PolyRatio {
        let upgraded_self = PolyRatio::from(self);
        upgraded_self / other
    }
}

impl Add<Polynomial> for PolyRatio {
    type Output = PolyRatio;

    fn add(self, other: Polynomial) -> PolyRatio {
        let upgraded_other = PolyRatio::from(other);
        self + upgraded_other
    }
}

impl Sub<Polynomial> for PolyRatio {
    type Output = PolyRatio;

    fn sub(self, other: Polynomial) -> PolyRatio {
        let upgraded_other = PolyRatio::from(other);
        self - upgraded_other
    }
}

impl Mul<Polynomial> for PolyRatio {
    type Output = PolyRatio;

    fn mul(self, other: Polynomial) -> PolyRatio {
        let upgraded_other = PolyRatio::from(other);
        self * upgraded_other
    }
}

impl Div<Polynomial> for PolyRatio {
    type Output = PolyRatio;

    fn div(self, other: Polynomial) -> PolyRatio {
        let upgraded_other = PolyRatio::from(other);
        self / upgraded_other
    }
}
