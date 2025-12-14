use async_trait::async_trait;
use std::marker::PhantomData;
use std::collections::HashMap;
use crate::orm::model::Model;
use crate::orm::query::Query;
use crate::orm::relations::{Relation, CreatableRelation};

// HasMorphOne relationship - similar to Laravel's morphOne
#[derive(Debug)]
pub struct HasMorphOne<T, R> {
    parent: T,
    morph_type: String,     // Column that stores the model type
    morph_id: String,       // Column that stores the model ID
    local_key: String,
    _marker: PhantomData<R>,
}

impl<T, R> HasMorphOne<T, R>
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

    // Additional query methods
    pub fn where_clause(self, column: &str, value: &str) -> Query<R> {
        self.get_query().where_clause(column, value)
    }

    // Check if the related model exists
    pub async fn exists(&self) -> Result<bool, sqlx::Error> {
        self.get_query().exists().await
    }

    // Delete the related model
    pub async fn delete(&self) -> Result<u64, sqlx::Error> {
        // This would implement deletion of the related model
        Ok(0)
    }

    // Update the related model
    pub async fn update(&self, attributes: HashMap<String, serde_json::Value>) -> Result<(), sqlx::Error> {
        // This would implement update of the related model
        Ok(())
    }
}

#[async_trait]
impl<T, R> Relation<T, R> for HasMorphOne<T, R>
where
    T: Model + Send + Sync + 'static,
    R: Model + Send + Sync + 'static,
{
    async fn get(&self) -> Result<Vec<R>, sqlx::Error> {
        match self.first().await? {
            Some(model) => Ok(vec![model]),
            None => Ok(vec![]),
        }
    }

    async fn first(&self) -> Result<Option<R>, sqlx::Error> {
        self.get_query().first().await
    }

    fn get_query(&self) -> Query<R> {
        let mut q = Query::new();
        // Add polymorphic constraints if parent key exists
        if let Some(val) = self.parent.get_key_value() {
            if let Some(id_str) = val.as_i64().map(|n| n.to_string()).or_else(|| val.as_str().map(|s| s.to_string())) {
                q = q.where_clause(&self.morph_type, &self.get_morph_type())
                     .where_clause(&self.morph_id, &id_str);
            }
        } else {
            // Still filter by morph_type when parent id is not available
            q = q.where_clause(&self.morph_type, &self.get_morph_type());
        }
        q
    }
}

#[async_trait]
impl<T, R> CreatableRelation<T, R> for HasMorphOne<T, R>
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
