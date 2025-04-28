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

//! Product service main struct declaration.

use crate::{repository::ProductRepository, product::Product};
use std::{fs::File, path::Path};
use sqlx::SqlitePool;

#[derive(Debug, Clone)]
pub struct ProductService {
    /// Struct for communicating with database.
    repository: ProductRepository,
}

impl ProductService {
    /// Construct new ProductService object.
    ///
    /// # Parameters
    /// - `path` - given database path.
    ///
    /// # Returns
    /// - New `ProductService` object - in case of success.
    /// - `SQLx error` - otherwise.
    pub async fn new(path: &str) -> Result<Self, sqlx::Error> {
        // Check whether database file is exist.
        if !Path::new(path).exists() {
            // Create new database file if it is not exist.
            if let Err(err) = File::create(path) {
                return Err(sqlx::Error::Io(err))
            }
        }

        let pool       = SqlitePool::connect(path).await?;
        let repository = ProductRepository::new(pool).await;

        Ok(Self { repository })
    }

    /// Create new product.
    ///
    /// # Parameters
    /// - `product` - given product info struct.
    ///
    /// # Returns
    /// - Product ID   - in case of success.
    /// - `SQLx error` - otherwise.
    pub async fn create_product(&self, product: Product)
        -> Result<i64, sqlx::Error>
    {
        self.repository.save(product).await
    }

    /// Get product info.
    ///
    /// # Parameters
    /// - `id` - given product identifier.
    ///
    /// # Returns
    /// - Product info - in case of success.
    /// - `SQLx error` - otherwise.
    pub async fn get_product(&self, id: i64) -> Result<Product, sqlx::Error> {
        self.repository.find_by_id(id).await
    }

    /// Update product info.
    ///
    /// # Parameters
    /// - `id`      - given product identifier.
    /// - `product` - given product info struct.
    ///
    /// # Returns
    /// - `Ok`         - in case of success.
    /// - `SQLx error` - otherwise.
    pub async fn update_product(&self, id: i64, product: Product)
        -> Result<(), sqlx::Error>
    {
        todo!()
    }

    /// Delete product.
    ///
    /// # Parameters
    /// - `id` - given product identifier.
    ///
    /// # Returns
    /// - `Ok`         - in case of success.
    /// - `SQLx error` - otherwise.
    pub async fn delete_product(&self, id: i64) -> Result<(), sqlx::Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{config, product::Product};
    use super::*;

    async fn setup_product_service() -> ProductService {
        let mut service = ProductService::new(config::TEST_DATABASE_PATH).await
            .expect("ProductService::new() should succeed!");

        service
    }

    #[actix_web::test]
    async fn test_setup_product_service() {
        let _ = setup_product_service();
    }

    #[actix_web::test]
    async fn test_create_product() {
        let service = setup_product_service().await;
        let result  = service.create_product(Product::default()).await;

        assert!(result.is_ok(), "Error to create product: {:#?}", result);

        let product_id = result.unwrap();
        println!("Created product with ID: {}", product_id);
    }
}