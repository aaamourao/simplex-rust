use std::collections::HashMap;
use num::Float;
use crate::var::Var;

pub enum RestrictionError {
    AlreadyCreated(String)
}

pub enum RestrictionSuccess {
    AddVarSuccess,
}

// By now, only restrictions such as "x1 + x2 = b" are supported
#[derive(Debug, Clone)]
pub struct Restriction {
    variables: HashMap<Var, f32>,
    pub expression_value: f32, // b
}

impl Restriction {
    pub fn new() -> Self {
        let variables = HashMap::new();
        let expression_value = f32::nan();
        Restriction {
            variables,
            expression_value,
        }
    }

    pub fn add_var(&mut self, var: Var, coeff: f32) -> Result<RestrictionSuccess, RestrictionError> {
        if self.variables.contains_key(&var) {
            return Err(RestrictionError::AlreadyCreated("var already exists".to_string()))
        }
        self.variables.insert(var, coeff);
        Ok(RestrictionSuccess::AddVarSuccess)
    }
}