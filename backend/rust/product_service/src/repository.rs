// Skate - courier delivery service.
// Copyright (C) 2025 Alexander (@alkuzin).
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! Product repository declaration.

use crate::product::Product;
use sqlx::sqlite::SqlitePool;

/// Struct for communicating with database.
#[derive(Debug, Clone)]
pub struct ProductRepository {
    /// Connection pool for SQLite database.
    pool: SqlitePool,
}

impl ProductRepository {
    /// Construct new ProductRepository object.
    ///
    /// # Parameters
    /// - `pool` - given connection pool for SQLite database.
    ///
    /// # Returns
    /// - New `ProductRepository` object.
    pub async fn new(pool: SqlitePool) -> Self {
        // Create the Products table.
        let query =
            r#"
            CREATE TABLE IF NOT EXISTS Products (
                product_id  INTEGER PRIMARY KEY AUTOINCREMENT,
                name        TEXT NOT NULL,
                description TEXT NOT NULL,
                price       INTEGER NOT NULL,
                quantity    INTEGER NOT NULL,
                image       TEXT
            );
            "#;

        if let Err(error) = sqlx::query(query).execute(&pool).await {
            eprintln!("Products creation error: {:#?}", error);
        }

        Self { pool }
    }

    /// Save product in database.
    ///
    /// # Parameters
    /// - `product` - given product info struct.
    ///
    /// # Returns
    /// - Product ID   - in case of success.
    /// - `SQLx error` - otherwise.
    pub async fn save(&self, product: Product) -> Result<i64, sqlx::Error> {
        let query =
            r#"
            INSERT INTO Products
            (name, description, price, quantity, image)
            VALUES (?, ?, ?, ?, ?)
            RETURNING product_id;
            "#;

        // Insert the new product into the database.
        let row: (i64,) = sqlx::query_as(query)
            .bind(&product.name)
            .bind(&product.description)
            .bind(&product.price)
            .bind(&product.quantity)
            .fetch_one(&self.pool)
            .await?;

        let product_id = row.0;

        Ok(product_id)
    }

}