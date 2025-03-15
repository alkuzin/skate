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

//! Order repository declaration.

use std::{collections::LinkedList, error::Error};
use chrono::{DateTime, NaiveDateTime, Utc};
use crate::dto::{OrderDTO, ProductItemDTO};
use crate::order::{Order, OrderStatus};
use sqlx::sqlite::SqlitePool;

/// Struct for communicating with database.
pub struct OrderRepository<'a> {
    /// Connection pool for SQLite database.
    pool: &'a SqlitePool,
}

impl<'a> OrderRepository<'a> {
    /// Construct new OrderRepository object.
    ///
    /// # Parameters
    /// - `pool` - given connection pool for SQLite database.
    ///
    /// # Returns
    /// New OrderRepository object.
    pub async fn new(pool: &'a SqlitePool) -> Self {
        // Create the Orders table.
        let query =
            r#"
            CREATE TABLE IF NOT EXISTS Orders (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                customer_id INTEGER NOT NULL,
                status TEXT NOT NULL,
                address TEXT NOT NULL,
                total_amount REAL NOT NULL,
                payment_method TEXT NOT NULL,
                products TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )
            "#;

        let _ = sqlx::query(query)
        .execute(pool)
        .await;

        Self { pool }
    }

    /// Find order info by ID.
    ///
    /// # Parameters
    /// - `id` - given order ID.
    ///
    /// # Returns
    /// Order      - in case of success.
    /// SQLx error - otherwise.
    pub async fn find_by_id(&self, id: i32) -> Result<Order, sqlx::Error> {
        let query =
            r#"
            SELECT id, customer_id, status, address, total_amount,
            payment_method, products
            FROM Orders
            WHERE id = ?
            "#;

        let row = sqlx::query_as::<_, Order>(query)
            .bind(id)
            .fetch_one(self.pool)
            .await?;

        Ok(row)
    }

    /// Save order info in database.
    ///
    /// # Parameters
    /// - `order` - given order info struct.
    ///
    /// # Returns
    /// Ok         - in case of success.
    /// SQLx error - otherwise.
    pub async fn save(self, order: Order) -> Result<(), sqlx::Error> {
        let query =
            r#"
            INSERT INTO Orders (customer_id, status, address, total_amount,
            payment_method, products, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#;

        let current_time = Utc::now().to_rfc3339();

        // Insert the new order into the database.
        sqlx::query(query)
        .bind(&order.customer_id)
        .bind(&order.status)
        .bind(&order.address)
        .bind(&order.total_amount)
        .bind(&order.payment_method)
        .bind(&order.products)
        .bind(&current_time)
        .bind(&current_time)
        .execute(self.pool)
        .await?;

        println!("[order-service] Saved order:\n{:#?}", order);
        Ok(())
    }

    /// Update order info in database.
    ///
    /// # Parameters
    /// - `order` - given order info struct.
    ///
    /// # Returns
    /// Ok         - in case of success.
    /// SQLx error - otherwise.
    pub async fn update(self, order: Order) -> Result<(), sqlx::Error> {
        let query =
            r#"
            UPDATE Orders
            SET
                customer_id = ?,
                status = ?,
                address = ?,
                total_amount = ?,
                payment_method = ?,
                products = ?,
            WHERE id = ?
            "#;

        // Insert the new order into the database.
        sqlx::query(query)
            .bind(&order.customer_id)
            .bind(&order.status)
            .bind(&order.address)
            .bind(&order.total_amount)
            .bind(&order.payment_method)
            .bind(&order.products)
            .bind(&order.id)
            .execute(self.pool)
            .await?;

        println!("[order-service] Updated order:\n{:#?}", order);
        Ok(())
    }

    /// Delete specific order info in database.
    ///
    /// # Parameters
    /// - `order` - given order info struct.
    ///
    /// # Returns
    /// Ok         - in case of success.
    /// SQLx error - otherwise.
    pub async fn delete(self, order: Order) -> Result<(), sqlx::Error> {
        let query =
            r#"
            DELETE FROM Orders
            WHERE id = ?
            "#;

        // Insert the new order into the database.
        sqlx::query(query)
            .bind(&order.id)
            .execute(self.pool)
            .await?;

        println!("[order-service] Deleted order:\n{:#?}", order);
        Ok(())
    }

    /// Find all orders info by customer ID in database.
    ///
    /// # Parameters
    /// - `customer_id` - given order customer identificator.
    ///
    /// # Returns
    /// List of order info - in case of success.
    /// SQLx error         - otherwise.
    pub async fn find_all_by_customer_id(&self, customer_id: i32) -> Result<LinkedList<Order>, sqlx::Error> {
        let query =
            r#"
            SELECT id, customer_id, status, address, total_amount,
            payment_method, products
            FROM Orders
            WHERE customer_id = ?
            "#;

        let rows = sqlx::query_as::<_, Order>(query)
            .bind(customer_id)
            .fetch_all(self.pool)
            .await?;

        let orders: LinkedList<Order> = rows.into_iter().collect();
        Ok(orders)
    }

    /// Find all orders info by order status in database.
    ///
    /// # Parameters
    /// - `status` - given order status.
    ///
    /// # Returns
    /// List of order info - in case of success.
    /// SQLx error         - otherwise.
    pub async fn find_all_by_status(&self, status: OrderStatus) -> Result<LinkedList<Order>, sqlx::Error> {
        let query =
            r#"
            SELECT id, customer_id, status, address, total_amount,
            payment_method, products
            FROM Orders
            WHERE status = ?
            "#;

        let rows = sqlx::query_as::<_, Order>(query)
            .bind(status.as_str())
            .fetch_all(self.pool)
            .await?;

        let orders: LinkedList<Order> = rows.into_iter().collect();
        Ok(orders)
    }

    /// Get orders info by its identificator in database.
    ///
    /// # Parameters
    /// - `order_id` - given order identificator.
    ///
    /// # Returns
    /// Order data transfer object - in case of success.
    /// SQLx error                 - otherwise.
    pub async fn get_order_details(&self, order_id: i32) -> Result<OrderDTO, Box<dyn Error>> {
        // Get order info.
        let query =
            r#"
            SELECT id, customer_id, status, address, total_amount,
            payment_method, products
            FROM Orders
            WHERE id = ?
            "#;

        let row = sqlx::query_as::<_, Order>(query)
            .bind(order_id)
            .fetch_one(self.pool)
            .await?;

        let (created_at, updated_at) = self.get_time_info(order_id).await?;

        let order = OrderDTO {
            id: row.id,
            customer_id: row.customer_id,
            status: OrderStatus::from(row.status.as_str()),
            address: row.address,
            created_at,
            updated_at,
            payment_method: row.payment_method,
            // TODO: get courier contact using order id.
            courier_contact: "???".to_string(),
            products: self.get_products_list(row.products)?,
        };

        Ok(order)
    }

    /// Get order time info.
    ///
    /// # Parameters
    /// - `order_id` - given order identificator.
    ///
    /// # Returns
    /// Tuple of created_at & updated_at time info - in case of success.
    /// SQLx error                                 - otherwise.
    async fn get_time_info(&self, order_id: i32) -> Result<(DateTime<Utc>, DateTime<Utc>), sqlx::Error> {
        // Get order creation & last update time.
        let query =
            r#"
            SELECT created_at, updated_at
            FROM Orders
            WHERE id = ?
            "#;

        let (created_at, updated_at) = sqlx::query_as::<_, (String, String)>(query)
            .bind(order_id)
            .fetch_one(self.pool)
            .await?;

        // Convert strings to DateTime<Utc> representation (ISO 8601 format).
        let format = "%Y-%m-%dT%H:%M:%SZ";

        let created_at: DateTime<Utc> = DateTime::from_naive_utc_and_offset(
            NaiveDateTime::parse_from_str(created_at.as_str(), format).unwrap(),
            Utc,
        );

        let updated_at: DateTime<Utc> = DateTime::from_naive_utc_and_offset(
            NaiveDateTime::parse_from_str(updated_at.as_str(), format).unwrap(),
            Utc,
        );

        Ok((created_at, updated_at))
    }

    /// Get list of products.
    ///
    /// # Parameters
    /// - `products` - given products list string representation.
    ///
    /// # Returns
    /// List of product data transfer objects - in case of success.
    /// Err                                   - otherwise.
    fn get_products_list(&self, products: String) -> Result<LinkedList<ProductItemDTO>, Box<dyn Error>> {
        let mut list: LinkedList<ProductItemDTO> = LinkedList::new();

        for part in products.split(", ") {
            // TODO: replace with method that find product item DTO from product_id.
            let _product_id = part.trim().parse::<i32>()?;
            list.push_back(ProductItemDTO::default());
        }

        Ok(list)
    }
}