pub mod var;
mod restriction;

#[cfg(test)]
mod tests {
    use crate::restriction::{Restriction, RestrictionError, RestrictionSuccess};
    use crate::var::Domain::Real;
    use crate::var::Var;

    #[test]
    fn it_works() {
        let var = Var::new("x1".to_string(), 0.75, Real, false);
        assert_eq!(var.get_var_name(), "x1");
        assert_eq!(var.get_cost(), 0.75);
        assert_eq!(var.get_domain(), Real);
        assert_eq!(var.is_slack_var(), false);
    }

    #[test]
    fn restrictions_works() {
        let mut restriction = Restriction::new();

        // 32x1 + 5x2 = 5
        let result_x1 = restriction.add_var(Var::new("x1".to_string(), 1.0, Real, false), 32.);
        let result_x2 = restriction.add_var(Var::new("x2".to_string(), 1.0, Real, false), 5.);
        let result_x2_again = restriction.add_var(Var::new("x2".to_string(), 1.0, Real, false), 5.);
        restriction.expression_value = 5.0;

        assert_eq!(result_x1, Ok(RestrictionSuccess::AddVarSuccess));
        assert_eq!(result_x2, Ok(RestrictionSuccess::AddVarSuccess));
        assert_eq!(result_x2_again, Err(RestrictionError::AlreadyCreated(
            "var already exists".to_string())));
    }
}
