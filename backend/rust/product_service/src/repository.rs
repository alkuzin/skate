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
                image       TEXT NOT NULL
            );
            "#;

        if let Err(error) = sqlx::query(query).execute(&pool).await {
            eprintln!("Products creation error: {:#?}", error);
        }

        Self { pool }
    }
}