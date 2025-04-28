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

#[derive(Debug, Clone)]
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
    pub async fn get_order(&self, id: i64) -> Result<Order, sqlx::Error> {
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
    pub async fn update_order(&self, id: i64, order: Order)
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
    pub async fn delete_order(&self, id: i64) -> Result<(), sqlx::Error> {
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
    use crate::{config, order::{OrderDTO, OrderItem, OrderStatus}};
    use super::*;

    async fn setup_order_service() -> OrderService {
        let service = OrderService::new(config::TEST_DATABASE_PATH).await
            .expect("OrderService::new() should succeed!");

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
        println!("Created order with ID: {}", order_id);
    }

    #[actix_web::test]
    async fn test_get_order_correct() {
        let service = setup_order_service().await;
        let result  = service.create_order(Order::default()).await;
        assert!(result.is_ok(), "Error to create order: {:#?}", result);

        let order_id = result.unwrap();
        let result   = service.get_order(order_id).await;
        assert!(result.is_ok(), "Error to get order info: {:#?}", result);

        let order = result.unwrap();
        println!("Get order info with ID {}: {:#?}", order_id, order);
    }

    #[actix_web::test]
    async fn test_get_order_incorrect() {
        let service = setup_order_service().await;
        let result  = service.get_order(0).await;

        assert!(result.is_err(), "Should return error: {:#?}", result);
    }

    #[actix_web::test]
    async fn test_update_order_correct() {
        let service  = setup_order_service().await;
        let order_id = 1;

        // Fill order info.
        let order_dto = OrderDTO {
            order_id,
            customer_id:  0,
            order_status: OrderStatus::Assembly,
            address:      "In the middle of nowhere".to_string(),
            price:        42,
        };
        let order_items = vec![OrderItem::default(); 3];
        let order = Order::new(order_dto, order_items);

        // Try update order info.
        let result = service.update_order(order_id, order).await;
        assert!(result.is_ok(), "Error to update order info: {:#?}", result);

        // Try get updated order info.
        let result = service.get_order(order_id).await;
        assert!(result.is_ok(), "Error to get order info: {:#?}", result);

        let order = result.unwrap();
        println!("Updated order info with ID {}: {:#?}", order_id, order);
    }

    #[actix_web::test]
    async fn test_update_order_incorrect() {
        let service = setup_order_service().await;
        let result  = service.update_order(0, Order::default()).await;

        assert!(result.is_err(), "Should return error");
    }

    #[actix_web::test]
    async fn test_delete_order_correct() {
        let service = setup_order_service().await;
        let result  = service.create_order(Order::default()).await;
        assert!(result.is_ok(), "Error to create order: {:#?}", result);

        let order_id = result.unwrap();
        let result   = service.delete_order(order_id).await;
        assert!(result.is_ok(), "Error to delete order: {:#?}", result);

        println!("Deleted order with ID: {}", order_id);
    }

    #[actix_web::test]
    async fn test_delete_order_incorrect() {
        let service = setup_order_service().await;
        let result  = service.delete_order(0).await;

        assert!(result.is_err(), "Should return error");
    }

    #[actix_web::test]
    async fn test_get_order_list_correct() {
        let service = setup_order_service().await;
        let result  = service.get_order_list().await;

        assert!(result.is_ok(), "Error to get list of orders: {:#?}", result);

        let order_list = result.unwrap();
        println!("Order list with {} orders:", order_list.len());

        for (i, order) in order_list.iter().enumerate() {
            println!("Order ({}): {:?}", i, order);
        }
    }
}
