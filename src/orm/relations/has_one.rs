use async_trait::async_trait;
use std::marker::PhantomData;
use std::collections::HashMap;
use crate::orm::model::Model;
use crate::orm::query::Query;
use crate::orm::relations::{Relation, CreatableRelation};

// HasOne relationship - similar to Laravel's HasOne
#[derive(Debug)]
pub struct HasOne<T, R> {
    parent: T,
    foreign_key: String,
    local_key: String,
    _marker: PhantomData<R>,
}

impl<T, R> HasOne<T, R>
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

    // Additional query methods specific to HasOne
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
impl<T, R> Relation<T, R> for HasOne<T, R>
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
        if let Some(val) = self.parent.get_key_value() {
            if let Some(id_str) = val.as_i64().map(|n| n.to_string()).or_else(|| val.as_str().map(|s| s.to_string())) {
                q = q.where_clause(&self.foreign_key, &id_str);
            }
        }
        q
    }
}

#[async_trait]
impl<T, R> CreatableRelation<T, R> for HasOne<T, R>
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
