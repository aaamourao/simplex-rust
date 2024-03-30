use std::hash::{Hash, Hasher};
use num::Float;

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
pub enum Domain {
    Real,
    Integer,
    Binary,
}

#[derive(Debug, Clone)]
pub struct Var {
    var_name: String,
    cost: f32,
    domain: Domain,
    is_slack_var: bool,
    pub value: f32, // all values are represented as f64, despite they might be Z or B
}

impl Var {
    pub fn new(var_name: String, cost: f32, domain: Domain, is_slack_var: bool) -> Self{
        Var {
            var_name,
            cost,
            domain,
            is_slack_var,
            value: f32::nan(),
        }
    }

    /*
     * These variables can't change, so there is a getter to get their value
     */
    pub fn get_var_name(&self) -> String {
        self.var_name.clone()
    }

    pub fn get_cost(&self) -> f32 {
        self.cost.clone()
    }

    pub fn get_domain(&self) -> Domain {
        self.domain.clone()
    }

    pub fn is_slack_var(&self) -> bool {
        self.is_slack_var.clone()
    }
}

impl Hash for Var {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.var_name.hash(state);
    }
}

impl PartialEq for Var {
    fn eq(&self, other: &Self) -> bool {
        self.var_name.eq(&other.get_var_name())
    }
}

impl Eq for Var {

}