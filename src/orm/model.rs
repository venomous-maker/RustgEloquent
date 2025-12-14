pub trait Model {
    fn table_name() -> &'static str;
    fn primary_key() -> &'static str;
    fn table() -> &'static str {
        Self::table_name()
    }
    
    fn fields() -> Vec<&'static str>;
    fn columns() -> Vec<&'static str>;
    fn get_filterable_fields() -> Vec<&'static str> {
        Self::fields()
            .into_iter()
            .filter(|&field| field != Self::primary_key())
            .collect()
    }
}