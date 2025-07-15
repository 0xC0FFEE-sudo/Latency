use sqlx::{sqlite::SqlitePoolOptions, FromRow, Sqlite, Pool, Row};
use anyhow::Result;
use crate::models::{Order, Fill, Trade};
use std::collections::HashMap;

#[derive(FromRow, Clone, Debug, serde::Serialize)]
pub struct TradeRow {
    pub id: String,
    pub order_id: String,
    pub symbol: String,
    pub side: String,
    pub amount: f64,
    pub price: f64,
    pub source: String,
    pub executed_at: String,
}

pub struct DatabaseManager {
    pool: Pool<Sqlite>,
}

impl DatabaseManager {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;
        Ok(Self { pool })
    }

    pub async fn init(&self) -> Result<()> {
        self.create_trades_table().await?;
        self.create_fills_table().await?;
        self.create_orders_table().await?;
        self.create_positions_table().await?;
        Ok(())
    }

    async fn create_trades_table(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS trades (
                id TEXT PRIMARY KEY,
                order_id TEXT NOT NULL,
                symbol TEXT NOT NULL,
                side TEXT NOT NULL,
                amount REAL NOT NULL,
                price REAL NOT NULL,
                source TEXT NOT NULL,
                executed_at TEXT NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn create_fills_table(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS fills (
                order_id TEXT NOT NULL,
                symbol TEXT NOT NULL,
                side TEXT NOT NULL,
                price REAL NOT NULL,
                quantity REAL NOT NULL,
                source TEXT NOT NULL,
                executed_at TEXT NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn create_orders_table(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS orders (
                id TEXT PRIMARY KEY,
                symbol TEXT NOT NULL,
                side TEXT NOT NULL,
                order_type TEXT NOT NULL,
                amount REAL NOT NULL,
                price REAL,
                status TEXT NOT NULL,
                source TEXT NOT NULL,
                created_at TEXT NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn create_positions_table(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS positions (
                symbol TEXT PRIMARY KEY,
                amount REAL NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_trades(&self) -> Result<Vec<TradeRow>> {
        let rows = sqlx::query_as::<_, TradeRow>(
            "SELECT id, order_id, symbol, side, amount, price, source, executed_at FROM trades ORDER BY executed_at DESC LIMIT 100"
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    pub async fn get_positions(&self) -> Result<HashMap<String, f64>> {
        let rows = sqlx::query("SELECT symbol, amount FROM positions")
            .fetch_all(&self.pool)
            .await?;

        let mut positions = HashMap::new();
        for row in rows {
            let symbol: String = row.try_get("symbol")?;
            let amount: f64 = row.try_get("amount")?;
            positions.insert(symbol, amount);
        }
        Ok(positions)
    }

    pub async fn set_positions(&self, positions: &HashMap<String, f64>) -> Result<()> {
        let mut tx = self.pool.begin().await?;
        for (symbol, amount) in positions {
            sqlx::query(
                r#"
                INSERT INTO positions (symbol, amount)
                VALUES (?, ?)
                ON CONFLICT(symbol) DO UPDATE SET amount = excluded.amount
                "#,
            )
            .bind(symbol)
            .bind(amount)
            .execute(&mut *tx)
            .await?;
        }
        tx.commit().await?;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn save_order(&self, order: &Order) -> Result<()> {
        let mut conn = self.pool.acquire().await?;
        sqlx::query(
            r#"
            INSERT INTO orders (id, symbol, side, order_type, amount, price, status, source, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&order.id.to_string())
        .bind(&order.symbol)
        .bind(&order.side.to_string())
        .bind(&order.order_type.to_string())
        .bind(order.amount)
        .bind(order.price)
        .bind(&order.status.to_string())
        .bind(order.source.to_string())
        .bind(order.created_at.to_rfc3339())
        .execute(&mut *conn)
        .await?;
        Ok(())
    }

    pub async fn save_fill(&self, fill: &Fill) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO fills (order_id, symbol, side, price, quantity, source, executed_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(fill.order_id.to_string())
        .bind(&fill.symbol)
        .bind(fill.side.to_string())
        .bind(fill.price)
        .bind(fill.quantity)
        .bind(fill.source.to_string())
        .bind(fill.executed_at.to_rfc3339())
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn save_trade(&self, trade: &Trade) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO trades (id, order_id, symbol, side, amount, price, source, executed_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(trade.id.to_string())
        .bind(trade.order_id.to_string())
        .bind(&trade.symbol)
        .bind(trade.side.to_string())
        .bind(trade.amount)
        .bind(trade.price)
        .bind(trade.source.to_string())
        .bind(trade.executed_at.to_rfc3339())
        .execute(&self.pool)
        .await?;
        Ok(())
    }
} 