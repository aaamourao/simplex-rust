use num::Float;

#[derive(Debug, Clone, Copy)]
pub enum Domain {
    Real,
    Integer,
    Binary,
}

#[derive(Debug, Clone)]
pub struct Var<T: Float> {
    var_name: String,
    cost: T,
    domain: Domain,
    pub value: f64, // all values are represented as f64, despite they might be Z or B
}

impl<T: Float> Var<T> {
    pub fn new(var_name: String, cost: T, domain: Domain) -> Self{
        Var {
            var_name,
            cost,
            domain,
            value: -1.0,
        }
    }

    /*
     * These variables can't change, so there is a getter to get their value
     */
    pub fn get_var_name(&self) -> String {
        self.var_name.clone()
    }

    pub fn get_cost(&self) -> T {
        self.cost.clone()
    }

    pub fn get_domain(&self) -> Domain {
        self.domain.clone()
    }
}