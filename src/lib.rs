pub mod var;

#[cfg(test)]
mod tests {
    use crate::var::Domain::Real;
    use crate::var::Var;

    #[test]
    fn it_works() {
        let var = Var::new("x1".to_string(), 0.75, Real);
        assert_eq!(var.get_var_name(), "x1");
        assert_eq!(var.get_cost(), 0.75);
        assert_eq!(var.get_domain(), Real);
    }
}
