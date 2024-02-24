#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub degree: i32,
}

#[derive(Debug)]
pub struct Term {
    pub coefficient: f64,
    pub variables: Vec<Variable>,
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
                if variable.degree > 1 {
                    result.push_str(&format!("^{}", variable.degree));
                }
            }
        }
        result
    }
}
