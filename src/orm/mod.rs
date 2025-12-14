pub mod model;
pub mod query;
mod relations;

use query::Query;
use model::Model;

pub trait Eloquent: Model + Sized {
    fn query() -> Query<Self> {
        Query::new()
    }

    fn where_(field: &str, value: &str) -> Query<Self> {
        Query::new().where_clause(field, value)
    }
}
