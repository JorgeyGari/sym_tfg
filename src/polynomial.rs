use core::panic;
use num::rational::Rational64;
use num::{FromPrimitive, ToPrimitive};
use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Eq, PartialOrd, Ord)]
pub struct Variable {
    pub name: String,
    pub degree: Rational64,
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
    pub fn max_degree(&self) -> Rational64 {
        self.variables
            .iter()
            .map(|v| v.degree)
            .max()
            .unwrap_or(0.into())
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
        new_vars.retain(|v| v.degree != 0.into());
        self.variables = new_vars;
    }

    /// Inverts the term.
    pub fn invert(&mut self) {
        self.coefficient = Rational64::new(*self.coefficient.denom(), *self.coefficient.numer());
        for var in &mut self.variables {
            var.degree *= -1;
        }
    }

    /// Returns the term to the power of q.
    pub fn pow(&self, q: Rational64) -> Term {
        let mut new_vars: Vec<Variable> = Vec::new();
        for var in &self.variables {
            new_vars.push(Variable {
                name: var.name.clone(),
                degree: var.degree * q.clone(),
            });
        }
        let ratio_coef =
            Rational64::from_f64(self.coefficient.to_f64().unwrap().powf(q.to_f64().unwrap()))
                .unwrap();
        if self.coefficient.denom() == &1 && ratio_coef.denom() != &1 {
            // Don't convert expressions like sqrt(13) to a ratio
            return self.clone();
        }
        Term {
            coefficient: ratio_coef,
            variables: new_vars,
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
        let mut product = Polynomial {
            terms: result,
            degree: 1.into(),
        };
        product.simplify();
        product.terms[0].clone()
    }
}

impl Div for Term {
    type Output = Polynomial;
    fn div(self, other: Self) -> Polynomial {
        let dividend = Polynomial {
            terms: vec![self],
            degree: 1.into(),
        };
        let divisor = Polynomial {
            terms: vec![other],
            degree: 1.into(),
        };
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
        let mut quotient = Polynomial {
            terms: result,
            degree: 1.into(),
        };
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
    pub degree: Rational64,
}

impl Polynomial {
    /// Return the degree of the polynomial.
    pub fn degree(&self) -> Rational64 {
        self.terms
            .iter()
            .map(|t| t.max_degree())
            .max()
            .unwrap_or(0.into())
    }

    /// List each term in the polynomial.
    pub fn _print(&self) {
        for term in &self.terms {
            println!("{:?}", term);
        }
    }

    /// Return the leading term of the polynomial.
    pub fn leading_term(&self) -> Term {
        let mut max_degree = Rational64::new(0, 1);
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
        let mut result = Polynomial {
            terms: Vec::new(),
            degree: 1.into(),
        };
        for term in &self.terms {
            let mut new_term = term.clone();
            for var in &mut new_term.variables {
                if let Some(val) = values.iter().find(|(name, _)| name == &var.name) {
                    let value = *val.1.clone().numer() as f64 / *val.1.denom() as f64;
                    let expon = *var.degree.numer() as f64 / *var.degree.denom() as f64;
                    new_term.coefficient =
                        new_term.coefficient * Rational64::from_f64(value.powf(expon)).unwrap();
                    var.degree = 0.into(); // Set the degree of the variable to 0, essentially removing it from the term
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
                .unwrap_or(0.into())
                .cmp(
                    &a.variables
                        .iter()
                        .map(|v| v.degree)
                        .max()
                        .unwrap_or(0.into()),
                );
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
            if term.coefficient == Rational64::new(0, 1) && self.terms.len() > 1 {
                continue;
            }
            if i != 0 && term.coefficient > Rational64::new(0, 1) {
                result.push_str("+");
            }
            if term.variables.is_empty() || term.coefficient != Rational64::new(1, 1) {
                result.push_str(&term.coefficient.to_string());
            }
            for variable in &term.variables {
                result.push_str(&variable.name);
                if variable.degree != 1.into() {
                    result.push_str(&format!("^({})", variable.degree));
                }
            }
        }
        if self.degree != 1.into() {
            result = format!("({})^({})", result, self.degree);
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
        let d: Option<f64> = self.degree.to_f64();
        // println!("Degree: {:?}", d);
        if d.is_some() {
            if self.terms.len() == 1 {
                let exp = Rational64::from_f64(d.unwrap());
                let powered = self.terms[0].pow(exp.unwrap()); // TODO: Here, sqrt(13) becomes a ratio
                self.terms = vec![powered];
            } else if d.unwrap().fract() == 0.0 && d.unwrap() >= 2.0 {
                for _i in 1..d.unwrap() as i64 {
                    *self = self.clone() * self.clone();
                }
            }
        }
        // println!("Simplifying 1: {}", self.as_string());

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

        // Add a term with coefficient 0 if all terms were removed
        if self.terms.is_empty() {
            self.terms.push(Term {
                coefficient: Rational64::new(0, 1),
                variables: vec![],
            });
        }

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

        // Find the gcd of the coefficients
        let mut gcd = self.terms[0].coefficient.numer().abs();
        for term in &self.terms {
            gcd = num_integer::gcd(gcd, term.coefficient.numer().abs());
        }
        factored_out.coefficient = Rational64::new(gcd, 1);

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
                if gcd != 0 {
                    for term in &mut factored.terms {
                        // println!("{:?}", term);
                        // println!("{:?}", gcd);
                        term.coefficient *= Rational64::new(1, gcd);
                    }
                }
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
                if var.degree < min_degree && var.degree > 0.into() {
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
        factored = factored
            * Polynomial {
                terms: vec![inv],
                degree: 1.into(),
            };

        // Undo the scaling of the coefficients
        for term in &mut factored.terms {
            term.coefficient *= Rational64::new(1, adjust);
        }
        factored_out.coefficient *= Rational64::new(1, adjust);

        factored.simplify();

        return (factored_out, factored);
    }

    /// Returns the name of the first variable in the polynomial.
    pub fn first_var(&self) -> Option<String> {
        if self.terms.len() == 0 {
            panic!("Polynomial has no terms!");
        } else if self.terms[0].variables.len() == 0 {
            return None;
        } else {
            return Some(self.terms[0].variables[0].name.clone());
        }
    }

    /// Finds the roots (numerical or symbolic) of the polynomial.
    pub fn roots(&self, var: &str) -> Vec<PolyRatio> {
        let mut result = Vec::new();
        let mut self_copy = self.clone();

        // Find out the degree of the polynomial
        self_copy.simplify();
        let degree = self.degree();

        match degree {
            d if d == 1.into() => {
                // If the degree is 1, the polynomial is linear: ax + b = 0
                // That means x = -b/a
                let a = self.terms[0].coefficient.clone();
                let b = Polynomial {
                    terms: self.terms[1..].to_vec(),
                    degree: 1.into(),
                };
                let minus_b = PolyRatio::from(b)
                    * PolyRatio::from(Polynomial {
                        terms: vec![Term {
                            coefficient: Rational64::new(-1, 1),
                            variables: vec![],
                        }],
                        degree: 1.into(),
                    });
                let root = minus_b
                    / PolyRatio::from(Polynomial {
                        terms: vec![Term {
                            coefficient: a,
                            variables: vec![],
                        }],
                        degree: 1.into(),
                    });
                result.push(root);
            }
            d if d == 2.into() => {
                // If the degree is 2, the polynomial is quadratic: ax^2 + bx + c = 0
                // That means x = (-b ± sqrt(b^2 - 4ac)) / 2a
                let a = self.terms[0].coefficient.clone();
                let b = self.terms[1].coefficient.clone();
                let c = self.terms[2].coefficient.clone();
                let minus_b = PolyRatio::from(Polynomial {
                    terms: vec![Term {
                        coefficient: Rational64::new(-1, 1),
                        variables: vec![],
                    }],
                    degree: 1.into(),
                }) * PolyRatio::from(Polynomial {
                    terms: vec![Term {
                        coefficient: b.clone(),
                        variables: vec![],
                    }],
                    degree: 1.into(),
                });
                let b_squared = PolyRatio::from(Polynomial {
                    terms: vec![Term {
                        coefficient: b.clone(),
                        variables: vec![],
                    }],
                    degree: 1.into(),
                }) * PolyRatio::from(Polynomial {
                    terms: vec![Term {
                        coefficient: b.clone(),
                        variables: vec![],
                    }],
                    degree: 1.into(),
                });
                let four_ac = PolyRatio::from(Polynomial {
                    terms: vec![Term {
                        coefficient: Rational64::new(4, 1),
                        variables: vec![],
                    }],
                    degree: 1.into(),
                }) * PolyRatio::from(Polynomial {
                    terms: vec![Term {
                        coefficient: a.clone(),
                        variables: vec![],
                    }],
                    degree: 1.into(),
                }) * PolyRatio::from(Polynomial {
                    terms: vec![Term {
                        coefficient: c.clone(),
                        variables: vec![],
                    }],
                    degree: 1.into(),
                });
                let mut discriminant = b_squared.clone() - four_ac.clone();
                discriminant.numerator.degree = Rational64::new(1, 2);
                discriminant.denominator.degree = Rational64::new(1, 2);
                // println!("Discriminant: {}", discriminant.as_string());
                discriminant.simplify();
                // println!("Discriminant: {}", discriminant.as_string());
                let two_a = PolyRatio::from(Polynomial {
                    terms: vec![Term {
                        coefficient: Rational64::new(2, 1),
                        variables: vec![],
                    }],
                    degree: 1.into(),
                }) * PolyRatio::from(Polynomial {
                    terms: vec![Term {
                        coefficient: a.clone(),
                        variables: vec![],
                    }],
                    degree: 1.into(),
                });
                // println!("Two a: {}", two_a.as_string());
                let root1 = (minus_b.clone() + discriminant.clone()) / two_a.clone();
                let root2 = (minus_b - discriminant) / two_a;
                result.push(root1);
                result.push(root2);
            }
            _ => {
                panic!("Higher degree polynomials not supported yet!");
            }
        }
        return result;
    }
}

impl Add for Polynomial {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut result = self.terms.clone();
        result.extend(other.terms);
        let mut sum = Polynomial {
            terms: result,
            degree: 1.into(),
        };
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
        let mut product = Polynomial {
            terms: result,
            degree: 1.into(),
        };
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
                degree: 1.into(),
            });
        }

        let mut divisor = other.clone();
        divisor.simplify();

        let mut quotient = Polynomial {
            terms: vec![],
            degree: 1.into(),
        };

        let mut remainder = dividend.clone();

        // println!(
        //     "Remainder: {}\nDivisor: {}",
        //     remainder.as_string(),
        //     divisor.as_string()
        // );

        let zero_poly = Polynomial {
            terms: vec![Term {
                coefficient: Rational64::new(0, 1),
                variables: vec![],
            }],
            degree: 1.into(),
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
        // THIS LAST CONDITION was THE PROBLEM (check what happens with 8/x)
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

#[derive(Debug, Clone)]
pub struct PolyRatio {
    pub numerator: Polynomial,
    pub denominator: Polynomial,
}

impl PolyRatio {
    pub fn simplify(&mut self) {
        // Simplify the initial numerator and denominator
        // println!("Numerator!: {}", self.numerator.as_string());
        // println!("Denominator!: {}", self.denominator.as_string());

        self.numerator.simplify();
        self.denominator.simplify();

        // println!("Numerator: {}", self.numerator.as_string());
        // println!("Denominator: {}", self.denominator.as_string());

        // Make the coefficients integers
        let mut n = self.numerator.clone();
        let mut d = self.denominator.clone();
        let adjust_n = n.make_integer();
        let adjust_d = d.make_integer();

        // println!("Numerator: {}", n.as_string());
        // println!("Denominator: {}", d.as_string());

        // Find the smallest negative exponent of each variable in the denominator
        let mut vars_to_move: Vec<Variable> = vec![];
        for term in &d.terms {
            for var in &term.variables {
                if var.degree < 0.into() {
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
            degree: 1.into(),
        };
        d = d * Polynomial {
            terms: vec![Term {
                coefficient: Rational64::new(1, 1),
                variables: vars_to_move.clone(),
            }],
            degree: 1.into(),
        };

        // println!("Numer: {}", n.as_string());
        // println!("Denom: {}", d.as_string());

        // Find the smallest negative exponent of each variable in the denominator
        let mut vars_to_move: Vec<Variable> = vec![];
        for term in &n.terms {
            for var in &term.variables {
                if var.degree < 0.into() {
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
            degree: 1.into(),
        };
        d = d * Polynomial {
            terms: vec![Term {
                coefficient: Rational64::new(1, 1),
                variables: vars_to_move.clone(),
            }],
            degree: 1.into(),
        };

        // println!("Numer: {}", n.as_string());
        // println!("Denom: {}", d.as_string());

        // Factor out as much as possible from the numerator and denominator
        let (t1, mut n) = n.factor();
        let (t2, mut d) = d.factor();

        // println!("t1: {:?}", t1);
        // println!("t2: {:?}", t2);

        // We are going to divide the numerator and denominator, these are the values by default
        let mut var_name = "".to_string();
        let mut min_degree = 0.into();

        // If the factored out terms share a variable
        if t1.variables.len() != 0 && t2.variables.len() != 0 {
            if t1.variables[0].name == t2.variables[0].name {
                var_name = t1.variables[0].name.clone();
                min_degree = t1.variables[0].degree.min(t2.variables[0].degree);
                // println!("Var name: {}", var_name);
                // println!("Min degree: {}", min_degree);
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
        // println!("GCD: {:?}", gcd_term);

        n = n * Polynomial {
            terms: vec![t1],
            degree: 1.into(),
        };
        d = d * Polynomial {
            terms: vec![t2],
            degree: 1.into(),
        };

        // Cancel out the gcd from the numerator and denominator
        let mut inv = gcd_term.clone();
        inv.invert();
        // println!("Inv: {:?}", inv);
        // println!("Numerator: {}", n.as_string());
        n = n * Polynomial {
            terms: vec![inv.clone()],
            degree: 1.into(),
        };
        d = d * Polynomial {
            terms: vec![inv],
            degree: 1.into(),
        };

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

        if self.denominator == self.numerator {
            self.numerator = Polynomial {
                terms: vec![Term {
                    coefficient: Rational64::new(1, 1),
                    variables: vec![],
                }],
                degree: 1.into(),
            };
            self.denominator = Polynomial {
                terms: vec![Term {
                    coefficient: Rational64::new(1, 1),
                    variables: vec![],
                }],
                degree: 1.into(),
            };
        }
    }

    pub fn as_string(&self) -> String {
        if self.denominator.as_string() == "1".to_string() {
            self.numerator.as_string()
        } else if self.denominator.as_string() == "0".to_string() {
            "ERROR: Division by zero!".to_string()
        } else {
            format!(
                "({}) / ({})",
                self.numerator.as_string(),
                self.denominator.as_string()
            )
        }
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
                degree: 1.into(),
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
