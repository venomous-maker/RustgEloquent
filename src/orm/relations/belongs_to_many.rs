use async_trait::async_trait;
use std::marker::PhantomData;
use std::collections::HashMap;
use crate::orm::model::Model;
use crate::orm::query::Query;
use crate::orm::relations::{Relation, AttachableRelation};

// BelongsToMany relationship - similar to Laravel's BelongsToMany
#[derive(Debug)]
pub struct BelongsToMany<T, R> {
    parent: T,
    table: String,           // Pivot table name
    foreign_pivot_key: String,
    related_pivot_key: String,
    parent_key: String,
    related_key: String,
    pivot_columns: Vec<String>,
    _marker: PhantomData<R>,
}

impl<T, R> BelongsToMany<T, R>
where
    T: Model + Send + Sync + 'static,
    R: Model + Send + Sync + 'static,
{
    pub fn new(
        parent: T,
        table: Option<String>,
        foreign_pivot_key: Option<String>,
        related_pivot_key: Option<String>,
        parent_key: Option<String>,
        related_key: Option<String>,
    ) -> Self {
        let table = table.unwrap_or_else(|| {
            let mut tables = vec![T::table_name(), R::table_name()];
            tables.sort();
            tables.join("_")
        });
        
        let foreign_pivot_key = foreign_pivot_key.unwrap_or_else(|| {
            format!("{}_id", T::table_name().trim_end_matches('s'))
        });
        
        let related_pivot_key = related_pivot_key.unwrap_or_else(|| {
            format!("{}_id", R::table_name().trim_end_matches('s'))
        });
        
        let parent_key = parent_key.unwrap_or_else(|| T::primary_key().to_string());
        let related_key = related_key.unwrap_or_else(|| R::primary_key().to_string());
        
        Self {
            parent,
            table,
            foreign_pivot_key,
            related_pivot_key,
            parent_key,
            related_key,
            pivot_columns: Vec::new(),
            _marker: PhantomData,
        }
    }

    // Add pivot columns to be retrieved
    pub fn with_pivot(mut self, columns: Vec<&str>) -> Self {
        self.pivot_columns = columns.iter().map(|s| s.to_string()).collect();
        self
    }

    // Add timestamp columns to pivot
    pub fn with_timestamps(mut self) -> Self {
        self.pivot_columns.extend_from_slice(&[
            "created_at".to_string(),
            "updated_at".to_string(),
        ]);
        self
    }

    // Count related models
    pub async fn count(&self) -> Result<i64, sqlx::Error> {
        self.get_query().count().await
    }

    // Check if any related models exist
    pub async fn exists(&self) -> Result<bool, sqlx::Error> {
        self.get_query().exists().await
    }

    // Toggle attachment of models
    pub async fn toggle(&self, ids: Vec<i64>) -> Result<(), sqlx::Error> {
        // This would implement toggle functionality
        Ok(())
    }

    // Sync with additional pivot data
    pub async fn sync_with_pivot_data(&self, data: HashMap<i64, HashMap<String, serde_json::Value>>) -> Result<(), sqlx::Error> {
        // This would implement sync with pivot data
        Ok(())
    }

    // Update existing pivot record
    pub async fn update_existing_pivot(&self, id: i64, attributes: HashMap<String, serde_json::Value>) -> Result<(), sqlx::Error> {
        // This would update pivot table data
        Ok(())
    }
}

#[async_trait]
impl<T, R> Relation<T, R> for BelongsToMany<T, R>
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
        // Build a base query for the related model
        let mut q = Query::new();

        // Try to get the parent's primary key value
        if let Some(val) = self.parent.get_key_value() {
            // Only handle simple number/string keys for now
            if let Some(id_str) = val.as_i64().map(|n| n.to_string()).or_else(|| val.as_str().map(|s| s.to_string())) {
                // Join pivot table to related table and filter by pivot foreign key
                let pivot_foreign_col = format!("{}.{}", self.table, self.foreign_pivot_key);
                let pivot_related_col = format!("{}.{}", self.table, self.related_pivot_key);
                let related_full_key = format!("{}.{}", R::table_name(), &self.related_key);

                q = q.join(&self.table, &pivot_related_col, "=", &related_full_key)
                     .where_clause(&pivot_foreign_col, &id_str);
            }
        }

        q
    }
}

#[async_trait]
impl<T, R> AttachableRelation<T, R> for BelongsToMany<T, R>
where
    T: Model + Send + Sync + 'static,
    R: Model + Send + Sync + 'static,
{
    async fn attach(&self, ids: Vec<i64>) -> Result<(), sqlx::Error> {
        // This would insert records into the pivot table
        for _id in ids {
            // INSERT INTO pivot_table (foreign_key, related_key) VALUES (parent_id, related_id)
        }
        Ok(())
    }

    async fn detach(&self, ids: Vec<i64>) -> Result<(), sqlx::Error> {
        // This would delete records from the pivot table
        for _id in ids {
            // DELETE FROM pivot_table WHERE foreign_key = parent_id AND related_key = related_id
        }
        Ok(())
    }

    async fn sync(&self, ids: Vec<i64>) -> Result<(), sqlx::Error> {
        // This would:
        // 1. Get currently attached IDs
        // 2. Detach IDs not in the new list
        // 3. Attach IDs not currently attached
        Ok(())
    }
}
