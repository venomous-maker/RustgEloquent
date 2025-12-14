use std::marker::PhantomData;

pub struct Query<T> {
    conditions: Vec<String>,
    _marker: PhantomData<T>,
}

impl<T> Query<T> {
    pub fn new() -> Self {
        Self {
            conditions: vec![],
            _marker: PhantomData,
        }
    }

    pub fn where_clause(mut self, field: &str, value: &str) -> Self {
        self.conditions
            .push(format!("{} = '{}'", field, value));
        self
    }

    pub fn first(self) {
        println!("SELECT * FROM {} WHERE {} LIMIT 1",
                 T::table(),
                 self.conditions.join(" AND ")
        );
    }
}
