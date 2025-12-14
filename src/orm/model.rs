use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Core trait for all models - similar to Laravel's Model
#[async_trait]
pub trait Model: Serialize + for<'de> Deserialize<'de> + Send + Sync + Clone + 'static {
    // Required implementations - similar to Laravel's model properties
    fn table_name() -> &'static str;
    fn primary_key() -> &'static str { "id" }
    fn fillable() -> Vec<&'static str>;

    // Optional overrides
    fn connection() -> &'static str { "default" }
    fn timestamps() -> bool { true }
    fn created_at_column() -> &'static str { "created_at" }
    fn updated_at_column() -> &'static str { "updated_at" }

    // Helper methods
    fn table() -> &'static str {
        Self::table_name()
    }
    
    fn get_key_name() -> &'static str {
        Self::primary_key()
    }

    // Database operations
    async fn find(id: i64) -> Result<Option<Self>, sqlx::Error>;
    async fn all() -> Result<Vec<Self>, sqlx::Error>;
    async fn create(attributes: HashMap<String, serde_json::Value>) -> Result<Self, sqlx::Error>;
    async fn save(&mut self) -> Result<(), sqlx::Error>;
    async fn delete(&self) -> Result<(), sqlx::Error>;
    async fn update(&mut self, attributes: HashMap<String, serde_json::Value>) -> Result<(), sqlx::Error>;
}

// Trait for models with timestamps
pub trait HasTimestamps: Model {
    fn touch(&mut self) {
        // This would update the updated_at timestamp
    }
}

// Trait for soft deletes (like Laravel's SoftDeletes)
pub trait SoftDeletes: Model {
    fn deleted_at_column() -> &'static str { "deleted_at" }
    fn trashed(&self) -> bool;
    async fn restore(&mut self) -> Result<(), sqlx::Error>;
    async fn force_delete(&self) -> Result<(), sqlx::Error>;
}

// Helper trait for attribute access
pub trait Attributable {
    fn get_attribute(&self, key: &str) -> Option<&serde_json::Value>;
    fn set_attribute(&mut self, key: &str, value: serde_json::Value);
    fn get_attributes(&self) -> &HashMap<String, serde_json::Value>;
    fn get_original(&self) -> &HashMap<String, serde_json::Value>;
    fn is_dirty(&self) -> bool;
    fn get_dirty(&self) -> HashMap<String, serde_json::Value>;
}

// Base implementation for a model instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInstance {
    pub attributes: HashMap<String, serde_json::Value>,
    pub original: HashMap<String, serde_json::Value>,
    pub exists: bool,
    pub was_recently_created: bool,
}

impl ModelInstance {
    pub fn new() -> Self {
        Self {
            attributes: HashMap::new(),
            original: HashMap::new(),
            exists: false,
            was_recently_created: false,
        }
    }

    pub fn from_attributes(attributes: HashMap<String, serde_json::Value>) -> Self {
        Self {
            original: attributes.clone(),
            attributes,
            exists: true,
            was_recently_created: false,
        }
    }
}

impl Attributable for ModelInstance {
    fn get_attribute(&self, key: &str) -> Option<&serde_json::Value> {
        self.attributes.get(key)
    }

    fn set_attribute(&mut self, key: &str, value: serde_json::Value) {
        self.attributes.insert(key.to_string(), value);
    }

    fn get_attributes(&self) -> &HashMap<String, serde_json::Value> {
        &self.attributes
    }

    fn get_original(&self) -> &HashMap<String, serde_json::Value> {
        &self.original
    }

    fn is_dirty(&self) -> bool {
        self.attributes != self.original
    }

    fn get_dirty(&self) -> HashMap<String, serde_json::Value> {
        let mut dirty = HashMap::new();
        for (key, value) in &self.attributes {
            if self.original.get(key) != Some(value) {
                dirty.insert(key.clone(), value.clone());
            }
        }
        dirty
    }
}