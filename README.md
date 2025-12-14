# RustEloquent 🦀

A Laravel-inspired ORM for Rust with support for relationships, query building, and database operations.

## Features

- **Laravel-like API**: Familiar syntax for developers coming from Laravel/Eloquent
- **Relationships**: Support for all major relationship types
- **Query Builder**: Fluent, chainable query interface
- **Async/Await**: Built with modern async Rust
- **Multiple Databases**: Support for MySQL, PostgreSQL, and SQLite via SQLx
- **Type Safety**: Leverages Rust's type system for compile-time guarantees

## Supported Relationships

- **HasOne**: One-to-one relationships
- **HasMany**: One-to-many relationships  
- **BelongsTo**: Inverse of one-to-one and one-to-many
- **BelongsToMany**: Many-to-many relationships with pivot tables
- **MorphOne**: Polymorphic one-to-one relationships
- **MorphMany**: Polymorphic one-to-many relationships

## Quick Start

### 1. Add to your Cargo.toml

```toml
[dependencies]
RustEloquent = "0.1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
async-trait = "0.1"
tokio = { version = "1.0", features = ["full"] }
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "mysql", "postgres", "sqlite", "chrono", "uuid"] }
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.0", features = ["v4", "serde"] }
```

### 2. Define your models

```rust
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use std::collections::HashMap;
use RustEloquent::orm::{Model, Eloquent, HasTimestamps};

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

    // Implement required methods...
    async fn find(id: i64) -> Result<Option<Self>, sqlx::Error> {
        // Database implementation
        Ok(None)
    }

    async fn all() -> Result<Vec<Self>, sqlx::Error> {
        Ok(Vec::new())
    }

    async fn create(attributes: HashMap<String, serde_json::Value>) -> Result<Self, sqlx::Error> {
        // Create implementation
        todo!()
    }

    async fn save(&mut self) -> Result<(), sqlx::Error> {
        Ok(())
    }

    async fn delete(&self) -> Result<(), sqlx::Error> {
        Ok(())
    }

    async fn update(&mut self, attributes: HashMap<String, serde_json::Value>) -> Result<(), sqlx::Error> {
        Ok(())
    }
}

impl Eloquent for User {}
impl HasTimestamps for User {}
```

### 3. Define relationships

```rust
impl User {
    pub fn posts(&self) -> HasMany<User, Post> {
        self.has_many(None, None)
    }

    pub fn profile(&self) -> HasOne<User, UserProfile> {
        self.has_one(None, None)
    }

    pub fn roles(&self) -> BelongsToMany<User, Role> {
        self.belongs_to_many(None, None, None, None, None)
    }
}

impl Post {
    pub fn user(&self) -> BelongsTo<Post, User> {
        self.belongs_to(None, None)
    }

    pub fn comments(&self) -> HasMorphMany<Post, Comment> {
        self.morph_many("commentable", None, None, None)
    }
}
```

## Usage Examples

### Basic Queries

```rust
// Find by ID
let user = User::find(1).first().await?;

// Get all users
let users = User::all().get().await?;

// Query with conditions
let active_users = User::where_("status", "active")
    .order_by_desc("created_at")
    .limit(10)
    .get()
    .await?;

// Complex queries
let users = User::query()
    .where_clause("age", "18")
    .where_op("salary", ">", serde_json::Value::Number(serde_json::Number::from(50000)))
    .where_in("department", vec![
        serde_json::Value::String("Engineering".to_string()),
        serde_json::Value::String("Design".to_string())
    ])
    .order_by_asc("name")
    .get()
    .await?;
```

### Working with Relationships

```rust
// Load relationships
let user = User::find(1).with(vec!["posts", "profile"]).first().await?;

// Access relationships
if let Some(user) = user {
    let posts = user.posts().get().await?;
    let profile = user.profile().first().await?;
    
    // Create related models
    let mut post_data = HashMap::new();
    post_data.insert("title".to_string(), serde_json::Value::String("My Post".to_string()));
    post_data.insert("content".to_string(), serde_json::Value::String("Content here".to_string()));
    
    let new_post = user.posts().create(post_data).await?;
    
    // Count relationships
    let post_count = user.posts().count().await?;
}
```

### Many-to-Many Relationships

```rust
// Attach roles to user
user.roles().attach(vec![1, 2, 3]).await?;

// Detach roles
user.roles().detach(vec![1]).await?;

// Sync roles (detach all others, attach these)
user.roles().sync(vec![2, 3, 4]).await?;

// Query with pivot data
let roles = user.roles()
    .with_pivot(vec!["created_at", "permissions"])
    .get()
    .await?;
```

### Pagination

```rust
let paginated_users = User::query()
    .where_clause("status", "active")
    .paginate(1, 15)
    .await?;

println!("Page: {}", paginated_users.current_page);
println!("Total: {}", paginated_users.total);
println!("Users: {:?}", paginated_users.data);
```

### Polymorphic Relationships

```rust
// Polymorphic relationship - comments that can belong to posts or videos
impl Comment {
    pub fn commentable<T: Model>(&self) -> BelongsToMorph<Comment, T> {
        // Implementation for getting the related model
    }
}

impl Post {
    pub fn comments(&self) -> HasMorphMany<Post, Comment> {
        self.morph_many("commentable", None, None, None)
    }
}

// Usage
let post_comments = post.comments().get().await?;
let new_comment = post.comments().create(comment_data).await?;
```

## Query Methods

### Where Clauses
- `where_clause(column, value)` - Basic where condition
- `where_op(column, operator, value)` - Where with custom operator
- `where_in(column, values)` - Where IN condition  
- `where_not_in(column, values)` - Where NOT IN condition
- `where_null(column)` - Where column IS NULL
- `where_not_null(column)` - Where column IS NOT NULL
- `or_where(column, operator, value)` - OR where condition

### Joins
- `join(table, first, operator, second)` - Inner join
- `left_join(table, first, operator, second)` - Left join
- `right_join(table, first, operator, second)` - Right join

### Ordering
- `order_by(column, direction)` - Order by column
- `order_by_asc(column)` - Order ascending
- `order_by_desc(column)` - Order descending  
- `latest(column?)` - Order by created_at DESC (or custom column)
- `oldest(column?)` - Order by created_at ASC (or custom column)

### Limiting & Offsetting
- `limit(count)` - Limit results
- `offset(count)` - Offset results
- `take(count)` - Alias for limit
- `skip(count)` - Alias for offset

### Grouping & Aggregation
- `group_by(columns)` - Group by columns
- `having(column, operator, value)` - Having clause
- `count()` - Count results
- `exists()` - Check if any results exist

### Eager Loading
- `with(relations)` - Eager load relationships

## Database Support

RustEloquent supports multiple database backends through SQLx:

- **MySQL** - Full support with connection pooling
- **PostgreSQL** - Full support with connection pooling  
- **SQLite** - Full support with connection pooling

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License.
