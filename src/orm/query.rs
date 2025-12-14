use std::marker::PhantomData;
use serde_json::Value;
use crate::orm::model::Model;

#[derive(Debug, Clone)]
pub struct Query<T> {
    table: Option<String>,
    select_columns: Vec<String>,
    where_conditions: Vec<WhereCondition>,
    joins: Vec<Join>,
    order_by: Vec<OrderBy>,
    limit_value: Option<i64>,
    offset_value: Option<i64>,
    group_by: Vec<String>,
    having_conditions: Vec<WhereCondition>,
    with_relations: Vec<String>,
    _marker: PhantomData<T>,
}

#[derive(Debug, Clone)]
pub struct WhereCondition {
    pub column: String,
    pub operator: String,
    pub value: Value,
    pub boolean: String, // AND, OR
}

#[derive(Debug, Clone)]
pub struct Join {
    pub table: String,
    pub first: String,
    pub operator: String,
    pub second: String,
    pub join_type: String, // INNER, LEFT, RIGHT, etc.
}

#[derive(Debug, Clone)]
pub struct OrderBy {
    pub column: String,
    pub direction: String, // ASC, DESC
}

impl<T> Query<T>
where
    T: Model + Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self {
            table: None,
            select_columns: vec!["*".to_string()],
            where_conditions: Vec::new(),
            joins: Vec::new(),
            order_by: Vec::new(),
            limit_value: None,
            offset_value: None,
            group_by: Vec::new(),
            having_conditions: Vec::new(),
            with_relations: Vec::new(),
            _marker: PhantomData,
        }
    }

    // Select methods
    pub fn select(mut self, columns: Vec<&str>) -> Self {
        self.select_columns = columns.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn select_raw(mut self, sql: &str) -> Self {
        self.select_columns = vec![sql.to_string()];
        self
    }

    // Where methods
    pub fn where_clause(mut self, column: &str, value: &str) -> Self {
        self.where_conditions.push(WhereCondition {
            column: column.to_string(),
            operator: "=".to_string(),
            value: Value::String(value.to_string()),
            boolean: "AND".to_string(),
        });
        self
    }

    pub fn where_op(mut self, column: &str, operator: &str, value: Value) -> Self {
        self.where_conditions.push(WhereCondition {
            column: column.to_string(),
            operator: operator.to_string(),
            value,
            boolean: "AND".to_string(),
        });
        self
    }

    pub fn where_in(mut self, column: &str, values: Vec<Value>) -> Self {
        self.where_conditions.push(WhereCondition {
            column: column.to_string(),
            operator: "IN".to_string(),
            value: Value::Array(values),
            boolean: "AND".to_string(),
        });
        self
    }

    pub fn where_not_in(mut self, column: &str, values: Vec<Value>) -> Self {
        self.where_conditions.push(WhereCondition {
            column: column.to_string(),
            operator: "NOT IN".to_string(),
            value: Value::Array(values),
            boolean: "AND".to_string(),
        });
        self
    }

    pub fn where_null(mut self, column: &str) -> Self {
        self.where_conditions.push(WhereCondition {
            column: column.to_string(),
            operator: "IS NULL".to_string(),
            value: Value::Null,
            boolean: "AND".to_string(),
        });
        self
    }

    pub fn where_not_null(mut self, column: &str) -> Self {
        self.where_conditions.push(WhereCondition {
            column: column.to_string(),
            operator: "IS NOT NULL".to_string(),
            value: Value::Null,
            boolean: "AND".to_string(),
        });
        self
    }

    pub fn or_where(mut self, column: &str, operator: &str, value: Value) -> Self {
        self.where_conditions.push(WhereCondition {
            column: column.to_string(),
            operator: operator.to_string(),
            value,
            boolean: "OR".to_string(),
        });
        self
    }

    // Join methods
    pub fn join(mut self, table: &str, first: &str, operator: &str, second: &str) -> Self {
        self.joins.push(Join {
            table: table.to_string(),
            first: first.to_string(),
            operator: operator.to_string(),
            second: second.to_string(),
            join_type: "INNER".to_string(),
        });
        self
    }

    pub fn left_join(mut self, table: &str, first: &str, operator: &str, second: &str) -> Self {
        self.joins.push(Join {
            table: table.to_string(),
            first: first.to_string(),
            operator: operator.to_string(),
            second: second.to_string(),
            join_type: "LEFT".to_string(),
        });
        self
    }

    pub fn right_join(mut self, table: &str, first: &str, operator: &str, second: &str) -> Self {
        self.joins.push(Join {
            table: table.to_string(),
            first: first.to_string(),
            operator: operator.to_string(),
            second: second.to_string(),
            join_type: "RIGHT".to_string(),
        });
        self
    }

    // Order methods
    pub fn order_by(mut self, column: &str, direction: &str) -> Self {
        self.order_by.push(OrderBy {
            column: column.to_string(),
            direction: direction.to_uppercase(),
        });
        self
    }

    pub fn order_by_asc(self, column: &str) -> Self {
        self.order_by(column, "ASC")
    }

    pub fn order_by_desc(self, column: &str) -> Self {
        self.order_by(column, "DESC")
    }

    pub fn latest(self, column: Option<&str>) -> Self {
        let col = column.unwrap_or("created_at");
        self.order_by_desc(col)
    }

    pub fn oldest(self, column: Option<&str>) -> Self {
        let col = column.unwrap_or("created_at");
        self.order_by_asc(col)
    }

    // Limit and offset
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit_value = Some(limit);
        self
    }

    pub fn offset(mut self, offset: i64) -> Self {
        self.offset_value = Some(offset);
        self
    }

    pub fn skip(self, skip: i64) -> Self {
        self.offset(skip)
    }

    pub fn take(self, take: i64) -> Self {
        self.limit(take)
    }

    // Group by and having
    pub fn group_by(mut self, columns: Vec<&str>) -> Self {
        self.group_by = columns.iter().map(|s| s.to_string()).collect();
        self
    }

    pub fn having(mut self, column: &str, operator: &str, value: Value) -> Self {
        self.having_conditions.push(WhereCondition {
            column: column.to_string(),
            operator: operator.to_string(),
            value,
            boolean: "AND".to_string(),
        });
        self
    }

    // Eager loading
    pub fn with(mut self, relations: Vec<&str>) -> Self {
        self.with_relations = relations.iter().map(|s| s.to_string()).collect();
        self
    }

    // Execution methods
    pub async fn get(self) -> Result<Vec<T>, sqlx::Error> {
        // This would execute the query and return results
        // For now, we'll return an empty vector
        Ok(Vec::new())
    }

    pub async fn first(self) -> Result<Option<T>, sqlx::Error> {
        let query = self.limit(1);
        let results = query.get().await?;
        Ok(results.into_iter().next())
    }

    pub async fn find_by_id(self, id: i64) -> Result<Option<T>, sqlx::Error> {
        let id_str = id.to_string();
        let query = self.where_clause(T::primary_key(), &id_str);
        query.first().await
    }

    pub async fn count(self) -> Result<i64, sqlx::Error> {
        // This would execute a COUNT query
        Ok(0)
    }

    pub async fn exists(self) -> Result<bool, sqlx::Error> {
        let count = self.count().await?;
        Ok(count > 0)
    }

    pub async fn paginate(self, page: i64, per_page: i64) -> Result<Pagination<T>, sqlx::Error> {
        let offset = (page - 1) * per_page;
        let results = self.clone().skip(offset).take(per_page).get().await?;
        let total = self.count().await?;
        
        let remaining = if total > offset { total - offset } else { 0 };
        let to_value = offset + std::cmp::min(per_page, remaining);
        
        Ok(Pagination {
            data: results,
            current_page: page,
            per_page,
            total,
            last_page: (total as f64 / per_page as f64).ceil() as i64,
            from: offset + 1,
            to: to_value,
        })
    }

    // SQL generation (for debugging)
    pub fn to_sql(&self) -> String {
        let table_name = T::table_name();
        let select = self.select_columns.join(", ");
        
        let mut sql = format!("SELECT {} FROM {}", select, table_name);
        
        // Add joins
        for join in &self.joins {
            sql.push_str(&format!(" {} JOIN {} ON {} {} {}", 
                join.join_type, join.table, join.first, join.operator, join.second));
        }
        
        // Add where conditions
        if !self.where_conditions.is_empty() {
            sql.push_str(" WHERE ");
            for (i, condition) in self.where_conditions.iter().enumerate() {
                if i > 0 {
                    sql.push_str(&format!(" {} ", condition.boolean));
                }
                sql.push_str(&format!("{} {} {}", condition.column, condition.operator, 
                    match &condition.value {
                        Value::String(s) => format!("'{}'", s),
                        Value::Number(n) => n.to_string(),
                        Value::Bool(b) => b.to_string(),
                        Value::Null => "NULL".to_string(),
                        Value::Array(arr) => format!("({})", 
                            arr.iter()
                               .map(|v| match v {
                                   Value::String(s) => format!("'{}'", s),
                                   Value::Number(n) => n.to_string(),
                                   _ => "NULL".to_string(),
                               })
                               .collect::<Vec<_>>()
                               .join(", ")),
                        _ => "NULL".to_string(),
                    }));
            }
        }
        
        // Add group by
        if !self.group_by.is_empty() {
            sql.push_str(&format!(" GROUP BY {}", self.group_by.join(", ")));
        }
        
        // Add having
        if !self.having_conditions.is_empty() {
            sql.push_str(" HAVING ");
            for (i, condition) in self.having_conditions.iter().enumerate() {
                if i > 0 {
                    sql.push_str(&format!(" {} ", condition.boolean));
                }
                sql.push_str(&format!("{} {} {}", condition.column, condition.operator, condition.value));
            }
        }
        
        // Add order by
        if !self.order_by.is_empty() {
            sql.push_str(" ORDER BY ");
            let order_clauses: Vec<String> = self.order_by.iter()
                .map(|o| format!("{} {}", o.column, o.direction))
                .collect();
            sql.push_str(&order_clauses.join(", "));
        }
        
        // Add limit and offset
        if let Some(limit) = self.limit_value {
            sql.push_str(&format!(" LIMIT {}", limit));
        }
        
        if let Some(offset) = self.offset_value {
            sql.push_str(&format!(" OFFSET {}", offset));
        }
        
        sql
    }
}

// Pagination result
#[derive(Debug, Clone)]
pub struct Pagination<T> {
    pub data: Vec<T>,
    pub current_page: i64,
    pub per_page: i64,
    pub total: i64,
    pub last_page: i64,
    pub from: i64,
    pub to: i64,
}

