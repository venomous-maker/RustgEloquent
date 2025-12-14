use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use std::collections::HashMap;
use RustEloquent::orm::{Model, Eloquent, HasTimestamps, Relation, CreatableRelation};

// User model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Option<i64>,
    pub name: String,
    pub email: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[async_trait]
impl Model for User {
    fn table_name() -> &'static str {
        "users"
    }

    fn fillable() -> Vec<&'static str> {
        vec!["name", "email"]
    }

    async fn find(id: i64) -> Result<Option<Self>, sqlx::Error> {
        // Implementation would query the database
        Ok(None)
    }

    async fn all() -> Result<Vec<Self>, sqlx::Error> {
        Ok(Vec::new())
    }

    async fn create(attributes: HashMap<String, serde_json::Value>) -> Result<Self, sqlx::Error> {
        // Implementation would insert into database and return model
        let user = User {
            id: Some(1),
            name: attributes.get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            email: attributes.get("email")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            created_at: Some(chrono::Utc::now()),
            updated_at: Some(chrono::Utc::now()),
        };
        Ok(user)
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

    // Provide instance helper to access primary key value for relations
    pub fn get_id_value(&self) -> Option<serde_json::Value> {
        self.id.map(|v| serde_json::Value::Number(serde_json::Number::from(v)))
    }

    // override get_key_value to use get_id_value
    fn get_key_value(&self) -> Option<serde_json::Value> {
        self.get_id_value()
    }
}

impl Eloquent for User {}
impl HasTimestamps for User {}

// Post model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: Option<i64>,
    pub title: String,
    pub content: String,
    pub user_id: i64,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[async_trait]
impl Model for Post {
    fn table_name() -> &'static str {
        "posts"
    }

    fn fillable() -> Vec<&'static str> {
        vec!["title", "content", "user_id"]
    }

    async fn find(id: i64) -> Result<Option<Self>, sqlx::Error> {
        Ok(None)
    }

    async fn all() -> Result<Vec<Self>, sqlx::Error> {
        Ok(Vec::new())
    }

    async fn create(attributes: HashMap<String, serde_json::Value>) -> Result<Self, sqlx::Error> {
        let post = Post {
            id: Some(1),
            title: attributes.get("title")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            content: attributes.get("content")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            user_id: attributes.get("user_id")
                .and_then(|v| v.as_i64())
                .unwrap_or(0),
            created_at: Some(chrono::Utc::now()),
            updated_at: Some(chrono::Utc::now()),
        };
        Ok(post)
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

    pub fn get_id_value(&self) -> Option<serde_json::Value> {
        self.id.map(|v| serde_json::Value::Number(serde_json::Number::from(v)))
    }

    fn get_key_value(&self) -> Option<serde_json::Value> {
        self.get_id_value()
    }
}

impl Eloquent for Post {}
impl HasTimestamps for Post {}

// Example usage
impl User {
    // Define relationship methods
    pub fn posts(&self) -> RustEloquent::orm::relations::HasMany<User, Post> {
        self.has_many(None, None)
    }
}

impl Post {
    // Define relationship methods
    pub fn user(&self) -> RustEloquent::orm::relations::BelongsTo<Post, User> {
        self.belongs_to(None, None)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Laravel-like usage examples:

    // Find a user using the Eloquent query API
    let user = <User as Eloquent>::find_by_id(1).first().await?;
    println!("Found user: {:?}", user);

    // Get all users (via query builder)
    let users = <User as Eloquent>::all().get().await?;
    println!("All users: {:?}", users);

    // Query with where clause
    let active_users = <User as Eloquent>::where_("status", "active")
        .order_by_desc("created_at")
        .limit(10)
        .get()
        .await?;
    println!("Active users: {:?}", active_users);

    // Create a new user
    let mut attributes = HashMap::new();
    attributes.insert("name".to_string(), serde_json::Value::String("John Doe".to_string()));
    attributes.insert("email".to_string(), serde_json::Value::String("john@example.com".to_string()));

    let new_user = User::create(attributes).await?;
    println!("Created user: {:?}", new_user);

    // Working with relationships (if user exists)
    if let Some(user) = user {
        // Get user's posts
        let posts = user.posts().get().await?;
        println!("User posts: {:?}", posts);

        // Create a new post for the user
        let mut post_attributes = HashMap::new();
        post_attributes.insert("title".to_string(), serde_json::Value::String("My First Post".to_string()));
        post_attributes.insert("content".to_string(), serde_json::Value::String("Hello World!".to_string()));

        let new_post = user.posts().create(post_attributes).await?;
        println!("Created post: {:?}", new_post);

        // Count user's posts
        let post_count = user.posts().count().await?;
        println!("User has {} posts", post_count);
    }

    // Pagination
    let paginated_users = <User as Eloquent>::all().paginate(1, 10).await?;
    println!("Paginated users: {:?}", paginated_users);

    // Advanced queries
    let complex_query = User::query()
        .where_clause("status", "active")
        .where_op("age", ">", serde_json::json!(18))
        .order_by_asc("name")
        .limit(20)
        .get()
        .await?;
    println!("Complex query result: {:?}", complex_query);

    Ok(())
}
