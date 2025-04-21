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

//! Order service main struct declaration.

use crate::{repository::OrderRepository, order::Order};
use std::{fs::File, path::Path};
use sqlx::SqlitePool;

#[derive(Debug)]
pub struct OrderService {
    /// Struct for communicating with database.
    repository: OrderRepository,
}

impl OrderService {
    /// Construct new OrderService object.
    ///
    /// # Parameters
    /// - `path` - given database path.
    ///
    /// # Returns
    /// - New `OrderRepository` object - in case of success.
    /// - `SQLx error` - otherwise.
    pub async fn new(path: &str) -> Result<Self, sqlx::Error> {
        // Check whether database file is exist.
        if !Path::new(path).exists() {
            // Create new database file if it is not exist.
            if let Err(err) = File::create(path) {
                return Err(sqlx::Error::Io(err))
            }
        }

        let pool = SqlitePool::connect(path).await?;
        Ok(Self { repository: OrderRepository::new(pool).await })
    }

    /// Initialize order service.
    ///
    /// # Returns
    /// - `Ok` - in case of success.
    /// - `SQLx error` - otherwise.
    pub async fn init(&mut self) -> Result<(), sqlx::Error> {
        Ok(())
    }

    /// Create new order.
    ///
    /// # Parameters
    /// - `order` - given order info struct.
    ///
    /// # Returns
    /// - Order ID     - in case of success.
    /// - `SQLx error` - otherwise.
    pub async fn create_order(&self, order: Order) -> Result<i64, sqlx::Error> {
        self.repository.save(order).await
    }

    /// Get order info.
    ///
    /// # Parameters
    /// - `id` - given order identifier.
    ///
    /// # Returns
    /// - Order info   - in case of success.
    /// - `SQLx error` - otherwise.
    pub async fn get_order(&self, id: i32) -> Result<Order, sqlx::Error> {
        self.repository.find_by_id(id).await
    }

    /// Update order info.
    ///
    /// # Parameters
    /// - `id`    - given order identifier.
    /// - `order` - given order info struct.
    ///
    /// # Returns
    /// - `Ok`         - in case of success.
    /// - `SQLx error` - otherwise.
    pub async fn update_order(&self, id: i32, order: Order)
        -> Result<(), sqlx::Error>
    {
        self.repository.update(id, order).await
    }

    /// Delete order.
    ///
    /// # Parameters
    /// - `id` - given order identifier.
    ///
    /// # Returns
    /// - `Ok`         - in case of success.
    /// - `SQLx error` - otherwise.
    pub async fn delete_order(&self, id: i32) -> Result<(), sqlx::Error> {
        self.repository.delete(id).await
    }

    /// Get list of all orders.
    ///
    /// # Returns
    /// - List of order data transfer objects - in case of success.
    /// - `SQLx error` - otherwise.
    pub async fn get_order_list(&self) -> Result<Vec<Order>, sqlx::Error> {
        self.repository.get_all_orders().await
    }
}

#[cfg(test)]
mod tests {
    use crate::config;
    use super::*;

    async fn setup_order_service() -> OrderService {
        let mut service = OrderService::new(config::TEST_DATABASE_PATH).await
            .expect("OrderService::new() should succeed!");

        let init_result = service.init().await
            .expect("OrderService::init_result() should succeed!");

        service
    }

    #[actix_web::test]
    async fn test_setup_order_service() {
        let _ = setup_order_service();
    }

    #[actix_web::test]
    async fn test_create_order() {
        let service = setup_order_service().await;
        let result  = service.create_order(Order::default()).await;

        assert!(result.is_ok(), "Error to create order: {:#?}", result);

        let order_id = result.unwrap();
        println!("Created order with ID: {:#?}", order_id);
    }
}
