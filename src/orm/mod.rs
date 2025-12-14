pub mod model;
pub mod query;
pub mod relations;

use std::collections::HashMap;

// Main trait that provides Laravel-like functionality
pub trait Eloquent: model::Model + Clone + Sized + Send + Sync + 'static {
    // Query builder methods
    fn query() -> query::Query<Self> {
        query::Query::new()
    }

    fn where_(field: &str, value: &str) -> query::Query<Self> {
        query::Query::new().where_clause(field, value)
    }

    fn all() -> query::Query<Self> {
        query::Query::new()
    }

    // Relationship helper methods
    fn has_one<R>(&self, foreign_key: Option<String>, local_key: Option<String>) -> relations::HasOne<Self, R>
    where
        R: model::Model + Send + Sync + 'static,
    {
        relations::HasOne::new(self.clone(), foreign_key, local_key)
    }

    fn has_many<R>(&self, foreign_key: Option<String>, local_key: Option<String>) -> relations::HasMany<Self, R>
    where
        R: model::Model + Send + Sync + 'static,
    {
        relations::HasMany::new(self.clone(), foreign_key, local_key)
    }

    fn belongs_to<R>(&self, foreign_key: Option<String>, owner_key: Option<String>) -> relations::BelongsTo<Self, R>
    where
        R: model::Model + Send + Sync + 'static,
    {
        relations::BelongsTo::new(self.clone(), foreign_key, owner_key)
    }

    fn belongs_to_many<R>(
        &self,
        table: Option<String>,
        foreign_pivot_key: Option<String>,
        related_pivot_key: Option<String>,
        parent_key: Option<String>,
        related_key: Option<String>,
    ) -> relations::BelongsToMany<Self, R>
    where
        R: model::Model + Send + Sync + 'static,
    {
        relations::BelongsToMany::new(
            self.clone(),
            table,
            foreign_pivot_key,
            related_pivot_key,
            parent_key,
            related_key,
        )
    }

    fn morph_one<R>(
        &self,
        name: &str,
        type_column: Option<String>,
        id_column: Option<String>,
        local_key: Option<String>,
    ) -> relations::HasMorphOne<Self, R>
    where
        R: model::Model + Send + Sync + 'static,
    {
        relations::HasMorphOne::new(self.clone(), name, type_column, id_column, local_key)
    }

    fn morph_many<R>(
        &self,
        name: &str,
        type_column: Option<String>,
        id_column: Option<String>,
        local_key: Option<String>,
    ) -> relations::HasMorphMany<Self, R>
    where
        R: model::Model + Send + Sync + 'static,
    {
        relations::HasMorphMany::new(self.clone(), name, type_column, id_column, local_key)
    }

    // Static methods for creating queries - fixed the string slice issue
    fn find_by_id(id: i64) -> query::Query<Self> {
        let id_str = id.to_string();
        query::Query::new().where_clause(Self::primary_key(), &id_str)
    }

    fn find_or_fail(id: i64) -> query::Query<Self> {
        Self::find_by_id(id)
    }

    fn first_or_create(_attributes: HashMap<String, serde_json::Value>) -> query::Query<Self> {
        // This would implement first_or_create logic
        query::Query::new()
    }

    fn first_or_new(_attributes: HashMap<String, serde_json::Value>) -> query::Query<Self> {
        // This would implement first_or_new logic
        query::Query::new()
    }

    fn update_or_create(
        _attributes: HashMap<String, serde_json::Value>,
        _values: HashMap<String, serde_json::Value>,
    ) -> query::Query<Self> {
        // This would implement update_or_create logic
        query::Query::new()
    }

    // Scopes
    fn latest(column: Option<&str>) -> query::Query<Self> {
        query::Query::new().latest(column)
    }

    fn oldest(column: Option<&str>) -> query::Query<Self> {
        query::Query::new().oldest(column)
    }
}

// Re-export commonly used types
pub use model::{Model, HasTimestamps, SoftDeletes, Attributable};
pub use relations::{
    Relation, CreatableRelation, AttachableRelation,
    HasOne, HasMany, BelongsTo, BelongsToMany, HasMorphOne, HasMorphMany
};

