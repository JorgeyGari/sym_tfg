#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub degree: i32,
}

impl PartialEq for Variable {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.degree == other.degree
    }
}

#[derive(Debug)]
pub struct Term {
    pub coefficient: f64,
    pub variables: Vec<Variable>,
}

impl Term {
    pub fn sort_vars(&mut self) {
        self.variables.sort_by(|a, b| a.name.cmp(&b.name));
    }

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

pub struct Polynomial {
    pub terms: Vec<Term>,
}

impl Polynomial {
    pub fn pprint(&self) -> String {
        let mut result = String::new();
        for term in &self.terms {
            if term.coefficient > 0.0 {
                result.push_str("+");
            }
            result.push_str(&term.coefficient.to_string());
            for variable in &term.variables {
                result.push_str(&variable.name);
                if variable.degree != 1 {
                    result.push_str(&format!("^{}", variable.degree));
                }
            }
        }
        result
    }

    pub fn simplify(&mut self) {
        for term in &mut self.terms {
            term.sort_vars();
        }

        for term in &mut self.terms {
            term.factor();
        }

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
}
