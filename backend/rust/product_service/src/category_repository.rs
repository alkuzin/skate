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

//! Product category repository declaration.

use crate::product::Category;
use sqlx::sqlite::SqlitePool;

/// Struct for communicating with database.
#[derive(Debug, Clone)]
pub struct CategoryRepository {
    /// Connection pool for SQLite database.
    pool: SqlitePool,
}

impl CategoryRepository {
    /// Construct new CategoryRepository object.
    ///
    /// # Parameters
    /// - `pool` - given connection pool for SQLite database.
    ///
    /// # Returns
    /// - New `CategoryRepository` object.
    pub async fn new(pool: SqlitePool) -> Self {
        // Create the Categories table.
        let query =
            r#"
            CREATE TABLE IF NOT EXISTS Categories (
                category_id INTEGER PRIMARY KEY AUTOINCREMENT,
                name        TEXT NOT NULL,
                image       TEXT
            );
            "#;

        if let Err(error) = sqlx::query(query).execute(&pool).await {
            eprintln!("Categories creation error: {:#?}", error);
        }

        Self { pool }
    }

    /// Save category in database.
    ///
    /// # Parameters
    /// - `category` - given category info struct.
    ///
    /// # Returns
    /// - Category ID  - in case of success.
    /// - `SQLx error` - otherwise.
    pub async fn save(&self, category: Category) -> Result<i64, sqlx::Error> {
        let query =
            r#"
            INSERT INTO Categories
            (name, image)
            VALUES (?, ?)
            RETURNING category_id;
            "#;

        // Insert the new category into the database.
        let row: (i64,) = sqlx::query_as(query)
            .bind(&category.category_id)
            .bind(&category.name)
            .bind(&category.image)
            .fetch_one(&self.pool)
            .await?;

        let category_id = row.0;

        Ok(category_id)
    }

    /// Find category info by ID.
    ///
    /// # Parameters
    /// - `id` - given category ID.
    ///
    /// # Returns
    /// - `Category`   - in case of success.
    /// - `SQLx error` - otherwise.
    pub async fn find_by_id(&self, id: i64) -> Result<Category, sqlx::Error> {
        // Get category DTO.
        let query =
            r#"
            SELECT category_id, name, image
            FROM Categories
            WHERE category_id = ?
            "#;

        let category = sqlx::query_as::<_, Category>(query)
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        Ok(category)
    }

    /// Update category in database.
    ///
    /// # Parameters
    /// - `id`       - given category identifier.
    /// - `category` - given category info struct.
    ///
    /// # Returns
    /// - `Ok`         - in case of success.
    /// - `SQLx error` - otherwise.
    pub async fn update(&self, id: i64, category: Category)
        -> Result<(), sqlx::Error>
    {
        let query =
            r#"
            UPDATE Categories
            SET
                name = ?,
                image = ?
            WHERE category_id = ?
            "#;

        // Insert the new category into the database.
        let result = sqlx::query(query)
            .bind(&category.name)
            .bind(&category.image)
            .bind(&id)
            .execute(&self.pool)
            .await?;

        // Handle incorrect category ID.
        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
    }

    /// Delete specific category from database.
    ///
    /// # Parameters
    /// - `id` - given category identifier.
    ///
    /// # Returns
    /// - `Ok`         - in case of success.
    /// - `SQLx error` - otherwise.
    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        let query  = "DELETE FROM Categories WHERE category_id = ?";
        let result = sqlx::query(query).bind(&id).execute(&self.pool).await?;

        // Handle incorrect category ID.
        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
    }

    /// Get all categories from database.
    ///
    /// # Returns
    /// - List of category info - in case of success.
    /// - `SQLx error` - otherwise.
    pub async fn get_all_categories(&self)
        -> Result<Vec<Category>, sqlx::Error>
    {
        let query =
            r#"
            SELECT category_id, name, image
            FROM Categories;
            "#;

        let rows = sqlx::query_as::<_, Category>(query)
            .fetch_all(&self.pool)
            .await?;

        let categories: Vec<Category> = rows.into_iter().collect();

        Ok(categories)
    }
}
