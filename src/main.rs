mod orm;
mod db;

use RustEloquent::orm::{Model, Eloquent};
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use std::collections::HashMap;

// Example User model
#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: Option<i64>,
    name: String,
    email: String,
}

#[async_trait]
impl Model for User {
    fn table_name() -> &'static str {
        "users"
    }

    fn fillable() -> Vec<&'static str> {
        vec!["name", "email"]
    }

    async fn find(_id: i64) -> Result<Option<Self>, sqlx::Error> {
        Ok(None)
    }

    async fn all() -> Result<Vec<Self>, sqlx::Error> {
        Ok(Vec::new())
    }

    async fn create(_attributes: HashMap<String, serde_json::Value>) -> Result<Self, sqlx::Error> {
        Ok(User {
            id: Some(1),
            name: "Test".to_string(),
            email: "test@example.com".to_string(),
        })
    }

    async fn save(&mut self) -> Result<(), sqlx::Error> {
        Ok(())
    }

    async fn delete(&self) -> Result<(), sqlx::Error> {
        Ok(())
    }

    async fn update(&mut self, _attributes: HashMap<String, serde_json::Value>) -> Result<(), sqlx::Error> {
        Ok(())
    }

    // Provide key value
    fn get_key_value(&self) -> Option<serde_json::Value> {
        self.id.map(|v| serde_json::Value::Number(serde_json::Number::from(v)))
    }
}

impl Eloquent for User {}

#[tokio::main]
async fn main() {
    println!("🦀 RustEloquent - Laravel-like ORM for Rust");
    println!("===========================================");

    // Demonstrate query building
    let query = User::query()
        .where_clause("status", "active")
        .order_by_desc("created_at")
        .limit(10);

    println!("Generated SQL: {}", query.to_sql());

    // Demonstrate relationships would work
    println!("\n📚 Available Relationships:");
    println!("- HasOne: user.profile()");
    println!("- HasMany: user.posts()");
    println!("- BelongsTo: post.user()");
    println!("- BelongsToMany: user.roles()");
    println!("- MorphOne: user.image()");
    println!("- MorphMany: user.comments()");

    println!("\n🔧 Available Query Methods:");
    println!("- where_clause(), where_in(), where_null()");
    println!("- join(), left_join(), right_join()");
    println!("- order_by(), latest(), oldest()");
    println!("- limit(), offset(), take(), skip()");
    println!("- group_by(), having()");
    println!("- with() for eager loading");
    println!("- paginate(), count(), exists()");
}
