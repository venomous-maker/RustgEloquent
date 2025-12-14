use async_trait::async_trait;
use std::marker::PhantomData;
use std::collections::HashMap;
use crate::orm::model::Model;
use crate::orm::query::Query;
use crate::orm::relations::{Relation, CreatableRelation};

// HasMorphMany relationship - similar to Laravel's morphMany
#[derive(Debug)]
pub struct HasMorphMany<T, R> {
    parent: T,
    morph_type: String,     // Column that stores the model type
    morph_id: String,       // Column that stores the model ID
    local_key: String,
    _marker: PhantomData<R>,
}

impl<T, R> HasMorphMany<T, R>
where
    T: Model + Send + Sync + 'static,
    R: Model + Send + Sync + 'static,
{
    pub fn new(
        parent: T,
        name: &str,
        type_column: Option<String>,
        id_column: Option<String>,
        local_key: Option<String>,
    ) -> Self {
        let morph_type = type_column.unwrap_or_else(|| format!("{}_type", name));
        let morph_id = id_column.unwrap_or_else(|| format!("{}_id", name));
        let local_key = local_key.unwrap_or_else(|| T::primary_key().to_string());
        
        Self {
            parent,
            morph_type,
            morph_id,
            local_key,
            _marker: PhantomData,
        }
    }

    // Get the morph type value for the parent model
    fn get_morph_type(&self) -> String {
        // This would return the class name or a configured morph map value
        T::table_name().to_string()
    }

    // Additional query methods specific to HasMorphMany
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

    // Get results with pagination
    pub async fn paginate(&self, page: i64, per_page: i64) -> Result<crate::orm::query::Pagination<R>, sqlx::Error> {
        self.get_query().paginate(page, per_page).await
    }
}

#[async_trait]
impl<T, R> Relation<T, R> for HasMorphMany<T, R>
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
            // This would add the polymorphic constraints
            // .where_clause(&self.morph_type, &self.get_morph_type())
            // .where_clause(&self.morph_id, &parent_key_value)
    }
}

#[async_trait]
impl<T, R> CreatableRelation<T, R> for HasMorphMany<T, R>
where
    T: Model + Send + Sync + 'static,
    R: Model + Send + Sync + 'static,
{
    async fn create(&self, mut attributes: HashMap<String, serde_json::Value>) -> Result<R, sqlx::Error> {
        // Set the morph type and ID
        attributes.insert(self.morph_type.clone(), serde_json::Value::String(self.get_morph_type()));
        // attributes.insert(self.morph_id.clone(), parent_key_value);
        R::create(attributes).await
    }

    async fn save(&self, model: &R) -> Result<(), sqlx::Error> {
        // This would save the model with the correct morph fields
        Ok(())
    }
}
