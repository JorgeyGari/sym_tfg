pub struct Variable {
    pub name: String,
    pub degree: i32,
}

pub struct Term {
    pub coefficient: f64,
    pub variables: Vec<Variable>,
}

/*
pub struct Polynomial {
    pub terms: Vec<Term>,
}
*/
