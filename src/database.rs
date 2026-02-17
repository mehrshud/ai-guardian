use sqlx::{Pool, Sqlite};
use log::{info, error};
from src/config.rs import Config;
from src/models.rs import User, Vulnerability, PerformanceMetric;

pub struct Database {
    pool: Pool<Sqlite>,
}

impl Database {
    /// Create a new Database instance
    pub async fn new(config: &Config) -> Result<Self, sqlx::Error> {
        let pool = Pool::connect(&config.db_url).await?;
        Ok(Database { pool })
    }

    /// Retrieve a user by id
    pub async fn get_user(&self, id: i32) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!("SELECT id, name, email FROM users WHERE id = ?", id)
            .fetch_one(&self.pool)
            .await?;
        Ok(user)
    }

    /// Store a vulnerability
    pub async fn store_vulnerability(&self, vulnerability: &Vulnerability) -> Result<(), sqlx::Error> {
        sqlx::query!("INSERT INTO vulnerabilities (id, description, severity) VALUES (?, ?, ?)", 
            vulnerability.id, vulnerability.description, vulnerability.severity)
            .execute(&self.pool)
            .await?;
        info!("Vulnerability stored successfully");
        Ok(())
    }

    /// Store a performance metric
    pub async fn store_performance_metric(&self, metric: &PerformanceMetric) -> Result<(), sqlx::Error> {
        sqlx::query!("INSERT INTO performance_metrics (id, metric, value) VALUES (?, ?, ?)", 
            metric.id, metric.metric, metric.value)
            .execute(&self.pool)
            .await?;
        info!("Performance metric stored successfully");
        Ok(())
    }
}