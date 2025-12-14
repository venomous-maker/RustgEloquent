use async_trait::async_trait;
use std::marker::PhantomData;
use std::collections::HashMap;
use crate::orm::model::Model;
use crate::orm::query::Query;
use crate::orm::relations::{Relation, CreatableRelation};

// HasMany relationship - similar to Laravel's HasMany
#[derive(Debug)]
pub struct HasMany<T, R> {
    parent: T,
    foreign_key: String,
    local_key: String,
    _marker: PhantomData<R>,
}

impl<T, R> HasMany<T, R>
where
    T: Model + Send + Sync + 'static,
    R: Model + Send + Sync + 'static,
{
    pub fn new(parent: T, foreign_key: Option<String>, local_key: Option<String>) -> Self {
        let foreign_key = foreign_key.unwrap_or_else(|| {
            format!("{}_id", T::table_name().trim_end_matches('s'))
        });
        let local_key = local_key.unwrap_or_else(|| T::primary_key().to_string());
        
        Self {
            parent,
            foreign_key,
            local_key,
            _marker: PhantomData,
        }
    }

    // Additional query methods specific to HasMany
    pub fn where_clause(self, column: &str, value: &str) -> Query<R> {
        self.get_query().where_clause(column, value)
    }

    pub fn order_by(self, column: &str, direction: &str) -> Query<R> {
        self.get_query().order_by(column, direction)
    }

    pub fn limit(self, limit: i64) -> Query<R> {
        self.get_query().limit(limit)
    }

    // Count related models
    pub async fn count(&self) -> Result<i64, sqlx::Error> {
        self.get_query().count().await
    }

    // Check if any related models exist
    pub async fn exists(&self) -> Result<bool, sqlx::Error> {
        self.get_query().exists().await
    }

    // Delete all related models
    pub async fn delete(&self) -> Result<u64, sqlx::Error> {
        // This would implement deletion of related models
        Ok(0)
    }

    // Update all related models
    pub async fn update(&self, attributes: HashMap<String, serde_json::Value>) -> Result<u64, sqlx::Error> {
        // This would implement bulk update of related models
        Ok(0)
    }
}

#[async_trait]
impl<T, R> Relation<T, R> for HasMany<T, R>
where
    T: Model + Send + Sync + 'static,
    R: Model + Send + Sync + 'static,
{
    async fn get(&self) -> Result<Vec<R>, sqlx::Error> {
        self.get_query().get().await
    }

    async fn first(&self) -> Result<Option<R>, sqlx::Error> {
        self.get_query().first().await
    }

    fn get_query(&self) -> Query<R> {
        Query::new()
            // This would add the foreign key constraint
            // .where_clause(&self.foreign_key, &parent_key_value)
    }
}

#[async_trait]
impl<T, R> CreatableRelation<T, R> for HasMany<T, R>
where
    T: Model + Send + Sync + 'static,
    R: Model + Send + Sync + 'static,
{
    async fn create(&self, mut attributes: HashMap<String, serde_json::Value>) -> Result<R, sqlx::Error> {
        // Set the foreign key to the parent's primary key value
        // attributes.insert(self.foreign_key.clone(), parent_key_value);
        R::create(attributes).await
    }

    async fn save(&self, model: &R) -> Result<(), sqlx::Error> {
        // This would save the model with the correct foreign key
        Ok(())
    }
}
