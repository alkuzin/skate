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

use crate::{
    product_repository::ProductRepository,
    category_repository::CategoryRepository,
    product::{Product, Category}
};
use std::{fs::File, path::Path};
use sqlx::SqlitePool;

#[derive(Debug, Clone)]
pub struct ProductService {
    /// Struct for communicating with products database.
    product_repository: ProductRepository,
    /// Struct for communicating with product categories database.
    category_repository: CategoryRepository,
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

        let product_pool        = SqlitePool::connect(path).await?;
        let product_repository  = ProductRepository::new(product_pool).await;

        let category_pool       = SqlitePool::connect(path).await?;
        let category_repository = CategoryRepository::new(category_pool).await;

        Ok(Self { product_repository, category_repository })
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
        self.product_repository.save(product).await
    }

    /// Create new product category.
    ///
    /// # Parameters
    /// - `category` - given product category info struct.
    ///
    /// # Returns
    /// - Category ID  - in case of success.
    /// - `SQLx error` - otherwise.
    pub async fn create_category(&self, category: Category)
        -> Result<i64, sqlx::Error>
    {
        self.category_repository.save(category).await
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
        self.product_repository.find_by_id(id).await
    }

    /// Get product category info.
    ///
    /// # Parameters
    /// - `id` - given product category identifier.
    ///
    /// # Returns
    /// - Product category info - in case of success.
    /// - `SQLx error` - otherwise.
    pub async fn get_category(&self, id: i64) -> Result<Category, sqlx::Error> {
        self.category_repository.find_by_id(id).await
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
        self.product_repository.update(id, product).await
    }

    /// Update product category info.
    ///
    /// # Parameters
    /// - `id`       - given product category identifier.
    /// - `category` - given product category info struct.
    ///
    /// # Returns
    /// - `Ok`         - in case of success.
    /// - `SQLx error` - otherwise.
    pub async fn update_category(&self, id: i64, category: Category)
        -> Result<(), sqlx::Error>
    {
        self.category_repository.update(id, category).await
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
        self.product_repository.delete(id).await
    }

    /// Delete product category.
    ///
    /// # Parameters
    /// - `id` - given product category identifier.
    ///
    /// # Returns
    /// - `Ok`         - in case of success.
    /// - `SQLx error` - otherwise.
    pub async fn delete_category(&self, id: i64) -> Result<(), sqlx::Error> {
        self.category_repository.delete(id).await
    }

    /// Get list of all products.
    ///
    /// # Returns
    /// - List of product info - in case of success.
    /// - `SQLx error` - otherwise.
    pub async fn get_product_list(&self) -> Result<Vec<Product>, sqlx::Error> {
        self.product_repository.get_all_products().await
    }

    /// Get list of all product categories.
    ///
    /// # Returns
    /// - List of product categories info - in case of success.
    /// - `SQLx error` - otherwise.
    pub async fn get_category_list(&self)
        -> Result<Vec<Category>, sqlx::Error>
    {
        self.category_repository.get_all_categories().await
    }

    /// Get list of products associated to specific product category.
    ///
    /// # Parameters
    /// - `id` - given product category identifier.
    ///
    /// # Returns
    /// - List of product info - in case of success.
    /// - `SQLx error` - otherwise.
    pub async fn get_products_by_category(&self, id: i64)
        -> Result<Vec<Product>, sqlx::Error>
    {
        self.product_repository.find_by_category(id).await
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

    // Tests for product repository related methods.

    #[actix_web::test]
    async fn test_create_product() {
        let service = setup_product_service().await;
        let result  = service.create_product(Product::default()).await;

        assert!(result.is_ok(), "Error to create product: {:#?}", result);

        let product_id = result.unwrap();
        println!("Created product with ID: {}", product_id);
    }

    #[actix_web::test]
    async fn test_get_product_correct() {
        let service = setup_product_service().await;
        let result  = service.create_product(Product::default()).await;
        assert!(result.is_ok(), "Error to create product: {:#?}", result);

        let product_id = result.unwrap();
        let result     = service.get_product(product_id).await;
        assert!(result.is_ok(), "Error to get product info: {:#?}", result);

        let product = result.unwrap();
        println!("Get product info with ID {}: {:#?}", product_id, product);
    }

    #[actix_web::test]
    async fn test_get_product_incorrect() {
        let service = setup_product_service().await;
        let result  = service.get_product(0).await;

        assert!(result.is_err(), "Should return error: {:#?}", result);
    }

    #[actix_web::test]
    async fn test_update_product_correct() {
        let service = setup_product_service().await;
        let result  = service.create_product(Product::default()).await;

        assert!(result.is_ok(), "Error to create product: {:#?}", result);

        let product_id = result.unwrap();

        // Fill product info.
        let product = Product {
            product_id,
            category_id: 0,
            name: "PC".to_string(),
            description: "Cool PC 10/10".to_string(),
            price: 9999,
            quantity: 42,
            image: "~/images/cool-pc.png".to_string(),
        };

        // Try update product info.
        let result = service.update_product(product_id, product).await;
        assert!(result.is_ok(), "Error to update product info: {:#?}", result);

        // Try get updated product info.
        let result = service.get_product(product_id).await;
        assert!(result.is_ok(), "Error to get product info: {:#?}", result);

        let product = result.unwrap();
        println!("Updated product info with ID {}: {:#?}", product_id, product);
    }

    #[actix_web::test]
    async fn test_update_product_incorrect() {
        let service = setup_product_service().await;
        let result  = service.update_product(0, Product::default()).await;

        assert!(result.is_err(), "Should return error");
    }

    #[actix_web::test]
    async fn test_delete_product_correct() {
        let service = setup_product_service().await;
        let result  = service.create_product(Product::default()).await;
        assert!(result.is_ok(), "Error to create product: {:#?}", result);

        let product_id = result.unwrap();
        let result     = service.delete_product(product_id).await;
        assert!(result.is_ok(), "Error to delete product: {:#?}", result);

        println!("Deleted product with ID: {}", product_id);
    }

    #[actix_web::test]
    async fn test_delete_product_incorrect() {
        let service = setup_product_service().await;
        let result  = service.delete_product(0).await;

        assert!(result.is_err(), "Should return error");
    }

    #[actix_web::test]
    async fn test_get_product_list() {
        let service = setup_product_service().await;
        let result  = service.get_product_list().await;

        assert!(result.is_ok(), "Error to get list of products: {:#?}", result);

        let product_list = result.unwrap();
        println!("Product list with {} products:", product_list.len());

        for (i, product) in product_list.iter().enumerate() {
            println!("Product ({}): {:?}", i, product);
        }
    }

    // Tests for category repository related methods.

    #[actix_web::test]
    async fn test_create_category() {
        let service = setup_product_service().await;
        let result  = service.create_category(Category::default()).await;

        assert!(result.is_ok(), "Error to create category: {:#?}", result);

        let category_id = result.unwrap();
        println!("Created category with ID: {}", category_id);
    }

    #[actix_web::test]
    async fn test_get_category_correct() {
        let service = setup_product_service().await;
        let result  = service.create_category(Category::default()).await;
        assert!(result.is_ok(), "Error to create category: {:#?}", result);

        let category_id = result.unwrap();
        let result     = service.get_category(category_id).await;
        assert!(result.is_ok(), "Error to get category info: {:#?}", result);

        let category = result.unwrap();
        println!("Get category info with ID {}: {:#?}", category_id, category);
    }

    #[actix_web::test]
    async fn test_get_category_incorrect() {
        let service = setup_product_service().await;
        let result  = service.get_category(0).await;

        assert!(result.is_err(), "Should return error: {:#?}", result);
    }

    #[actix_web::test]
    async fn test_update_category_correct() {
        let service = setup_product_service().await;
        let result  = service.create_category(Category::default()).await;

        assert!(result.is_ok(), "Error to create category: {:#?}", result);

        let category_id = result.unwrap();

        // Fill category info.
        let category = Category {
            category_id,
            name: "Tech".to_string(),
            image: "~/images/tech.png".to_string(),
        };

        // Try update category info.
        let result = service.update_category(category_id, category).await;
        assert!(result.is_ok(), "Error to update category info: {:#?}", result);

        // Try get updated category info.
        let result = service.get_category(category_id).await;
        assert!(result.is_ok(), "Error to get category info: {:#?}", result);

        let category = result.unwrap();
        println!("Updated category info with ID {}: {:#?}", category_id, category);
    }

    #[actix_web::test]
    async fn test_update_category_incorrect() {
        let service = setup_product_service().await;
        let result  = service.update_category(0, Category::default()).await;

        assert!(result.is_err(), "Should return error");
    }

    #[actix_web::test]
    async fn test_delete_category_correct() {
        let service = setup_product_service().await;
        let result  = service.create_category(Category::default()).await;
        assert!(result.is_ok(), "Error to create category: {:#?}", result);

        let category_id = result.unwrap();
        let result      = service.delete_category(category_id).await;
        assert!(result.is_ok(), "Error to delete category: {:#?}", result);

        println!("Deleted category with ID: {}", category_id);
    }

    #[actix_web::test]
    async fn test_delete_category_incorrect() {
        let service = setup_product_service().await;
        let result  = service.delete_category(0).await;

        assert!(result.is_err(), "Should return error");
    }

    #[actix_web::test]
    async fn test_get_category_list() {
        let service = setup_product_service().await;
        let result  = service.get_category_list().await;

        assert!(result.is_ok(), "Error to get list of categories: {:#?}", result);

        let category_list = result.unwrap();
        println!("Category list with {} categories:", category_list.len());

        for (i, category) in category_list.iter().enumerate() {
            println!("Category ({}): {:?}", i, category);
        }
    }
}