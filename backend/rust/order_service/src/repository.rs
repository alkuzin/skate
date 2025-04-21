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

use crate::order::{Order, OrderDTO, OrderItem};
use sqlx::sqlite::SqlitePool;

/// Struct for communicating with database.
#[derive(Debug)]
pub struct OrderRepository {
    /// Connection pool for SQLite database.
    pool: SqlitePool,
}

impl OrderRepository {
    /// Construct new OrderRepository object.
    ///
    /// # Parameters
    /// - `pool` - given connection pool for SQLite database.
    ///
    /// # Returns
    /// - New `OrderRepository` object.
    pub async fn new(pool: SqlitePool) -> Self {
        // Create the Orders table.
        let query =
            r#"
            CREATE TABLE IF NOT EXISTS Orders (
                order_id     INTEGER PRIMARY KEY AUTOINCREMENT,
                customer_id  INTEGER NOT NULL,
                order_status INTEGER NOT NULL,
                address      TEXT NOT NULL,
                price        INTEGER NOT NULL
            );
            "#;

        if let Err(error) = sqlx::query(query).execute(&pool).await {
            eprintln!("Orders creation error: {:#?}", error);
        }

        // Create the OrderItems table.
        let query =
            r#"
            CREATE TABLE IF NOT EXISTS OrderItems (
                order_id    INTEGER NOT NULL,
                product_id  INTEGER NOT NULL,
                quantity    INTEGER NOT NULL,
                unit_price  INTEGER NOT NULL,
                total_price INTEGER NOT NULL
            );
            "#;

        if let Err(error) = sqlx::query(query).execute(&pool).await {
            eprintln!("OrderItems creation error: {:#?}", error);
        }

        Self { pool }
    }

    /// Find order info by ID.
    ///
    /// # Parameters
    /// - `id` - given order ID.
    ///
    /// # Returns
    /// - `Order`      - in case of success.
    /// - `SQLx error` - otherwise.
    pub async fn find_by_id(&self, id: i64) -> Result<Order, sqlx::Error> {
        // Get order DTO.
        let query =
            r#"
            SELECT order_id, customer_id, order_status, address, price
            FROM Orders
            WHERE order_id = ?
            "#;

        let order_dto = sqlx::query_as::<_, OrderDTO>(query)
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        // Get order items.
        let query = "SELECT * FROM OrderItems WHERE order_id = ?";

        let order_items: Vec<OrderItem> = sqlx::query_as::<_, OrderItem>(query)
            .bind(id)
            .fetch_all(&self.pool)
            .await?;

        Ok(Order::new(order_dto, order_items))
    }

    /// Save order in database.
    ///
    /// # Parameters
    /// - `order` - given order info struct.
    ///
    /// # Returns
    /// - Order ID     - in case of success.
    /// - `SQLx error` - otherwise.
    pub async fn save(&self, order: Order) -> Result<i64, sqlx::Error> {
        let query =
            r#"
            INSERT INTO Orders
            (customer_id, order_status, address, price)
            VALUES (?, ?, ?, ?)
            RETURNING order_id;
            "#;

        // Insert the new order DTO into the database.
        let row: (i64,) = sqlx::query_as(query)
            .bind(&order.dto.customer_id)
            .bind(&order.dto.order_status)
            .bind(&order.dto.address)
            .bind(&order.dto.price)
            .fetch_one(&self.pool)
            .await?;

        // Insert the new order items list into the database.
        let query =
            r#"
            INSERT INTO OrderItems
            (order_id, product_id, quantity, unit_price, total_price)
            VALUES (?, ?, ?, ?, ?)
            "#;

        for item in &order.items {
            sqlx::query(query)
                .bind(&order.dto.order_id)
                .bind(&item.product_id)
                .bind(&item.quantity)
                .bind(&item.unit_price)
                .bind(&item.total_price)
                .execute(&self.pool)
                .await?;
        }

        Ok(row.0)
    }

    /// Update order in database.
    ///
    /// # Parameters
    /// - `id`    - given order identifier.
    /// - `order` - given order info struct.
    ///
    /// # Returns
    /// Ok         - in case of success.
    /// SQLx error - otherwise.
    pub async fn update(&self, id: i64, order: Order) -> Result<(), sqlx::Error> {
        let query =
            r#"
            UPDATE Orders
            SET
                customer_id = ?,
                order_status = ?,
                address = ?,
                price = ?
            WHERE order_id = ?
            "#;

        // Insert the new order into the database.
        let result = sqlx::query(query)
            .bind(&order.dto.customer_id)
            .bind(&order.dto.order_status)
            .bind(&order.dto.address)
            .bind(&order.dto.price)
            .bind(&id)
            .execute(&self.pool)
            .await?;

        // Handle incorrect order ID.
        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        let query =
            r#"
            UPDATE OrderItems
            SET
                product_id = ?,
                quantity = ?,
                unit_price = ?,
                total_price = ?
            WHERE order_id = ?
            "#;

            for item in &order.items {
                sqlx::query(query)
                    .bind(&item.product_id)
                    .bind(&item.quantity)
                    .bind(&item.unit_price)
                    .bind(&item.total_price)
                    .bind(&id)
                    .execute(&self.pool)
                    .await?;
            }

        Ok(())
    }

    /// Delete specific order from database.
    ///
    /// # Parameters
    /// - `id` - given order identifier.
    ///
    /// # Returns
    /// - `Ok`         - in case of success.
    /// - `SQLx error` - otherwise.
    pub async fn delete(&self, id: i64) -> Result<(), sqlx::Error> {
        let query = r#"DELETE FROM Orders WHERE order_id = ?"#;
        sqlx::query(query).bind(&id).execute(&self.pool).await?;

        let query = r#"DELETE FROM OrderItems WHERE order_id = ?"#;
        sqlx::query(query).bind(&id).execute(&self.pool).await?;

        println!("[order-service] Deleted order with ID: {}", id);
        Ok(())
    }

    /// Find all orders info by customer ID in database.
    ///
    /// # Parameters
    /// - `customer_id` - given order customer identifier.
    ///
    /// # Returns
    /// - `List of order info` - in case of success.
    /// - `SQLx error`         - otherwise.
    pub async fn find_all_by_customer_id(&self, customer_id: i64)
        -> Result<Vec<Order>, sqlx::Error>
    {
        let query =
            r#"
            SELECT (order_id, customer_id, order_status, address, price)
            FROM Orders
            WHERE customer_id = ?
            "#;

        let rows = sqlx::query_as::<_, OrderDTO>(query)
            .bind(customer_id)
            .fetch_all(&self.pool)
            .await?;

        let orders_dto: Vec<OrderDTO> = rows.into_iter().collect();
        let mut orders: Vec<Order> = Vec::with_capacity(orders_dto.len());

        // Find all products associated with orders_dto.
        for order_dto in orders_dto {
            let items = self.get_order_items_by_id(order_dto.order_id).await?;
            orders.push(Order::new(order_dto, items));
        }

        Ok(orders)
    }

    /// Get all orders from database.
    ///
    /// # Returns
    /// - List of order info - in case of success.
    /// - `SQLx error` - otherwise.
    pub async fn get_all_orders(&self) -> Result<Vec<Order>, sqlx::Error> {
        let query =
            r#"
            SELECT (order_id, customer_id, order_status, address, price)
            FROM Orders
            "#;

        let rows = sqlx::query_as::<_, OrderDTO>(query)
            .fetch_all(&self.pool)
            .await?;

        let orders_dto: Vec<OrderDTO> = rows.into_iter().collect();
        let mut orders: Vec<Order> = Vec::with_capacity(orders_dto.len());

        // Find all products associated with orders_dto.
        for order_dto in orders_dto {
            let items = self.get_order_items_by_id(order_dto.order_id).await?;
            orders.push(Order::new(order_dto, items));
        }

        Ok(orders)
    }

    /// Get all order items from database by order ID.
    ///
    /// # Parameters
    /// - `id` - given order identifier.
    ///
    /// # Returns
    /// - List of order items - in case of success.
    /// - `SQLx error` - otherwise.
    pub async fn get_order_items_by_id(&self, id: i64)
        -> Result<Vec<OrderItem>, sqlx::Error>
    {
        let query =
            r#"
            SELECT (order_id, product_id, quantity, unit_price, total_price)
            FROM OrderItems
            WHERE order_id = ?
            "#;

        let items = sqlx::query_as::<_, OrderItem>(query)
            .bind(id)
            .fetch_all(&self.pool)
            .await?;

        Ok(items)
    }
}