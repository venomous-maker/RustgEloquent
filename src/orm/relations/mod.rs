pub mod belongs_to;
pub mod has_many;
pub mod has_one;
pub mod belongs_to_many;
pub mod has_morph_one;
pub mod has_morph_many;

use async_trait::async_trait;
use crate::orm::model::Model;
use crate::orm::query::Query;

// Base relationship trait
#[async_trait]
pub trait Relation<T, R> 
where 
    T: Model + Send + Sync + 'static,
    R: Model + Send + Sync + 'static,
{
    async fn get(&self) -> Result<Vec<R>, sqlx::Error>;
    async fn first(&self) -> Result<Option<R>, sqlx::Error>;
    fn get_query(&self) -> Query<R>;
}

// Trait for relationships that can be created/updated
#[async_trait]
pub trait CreatableRelation<T, R>: Relation<T, R>
where 
    T: Model + Send + Sync + 'static,
    R: Model + Send + Sync + 'static,
{
    async fn create(&self, attributes: std::collections::HashMap<String, serde_json::Value>) -> Result<R, sqlx::Error>;
    async fn save(&self, model: &R) -> Result<(), sqlx::Error>;
}

// Trait for relationships that support attachment/detachment (many-to-many)
#[async_trait]
pub trait AttachableRelation<T, R>: Relation<T, R>
where 
    T: Model + Send + Sync + 'static,
    R: Model + Send + Sync + 'static,
{
    async fn attach(&self, ids: Vec<i64>) -> Result<(), sqlx::Error>;
    async fn detach(&self, ids: Vec<i64>) -> Result<(), sqlx::Error>;
    async fn sync(&self, ids: Vec<i64>) -> Result<(), sqlx::Error>;
}

pub use belongs_to::BelongsTo;
pub use has_many::HasMany;
pub use has_one::HasOne;
pub use belongs_to_many::BelongsToMany;
pub use has_morph_one::HasMorphOne;
pub use has_morph_many::HasMorphMany;

