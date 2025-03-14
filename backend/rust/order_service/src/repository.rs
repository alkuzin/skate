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

// class "OrderRepository" {
//     +findById(id: int): Order
//     +save(order: Order): void
//     +update(order: Order): void
//     +delete(order: Order): void
// TODO:
//     +findAllByCustomerId(customer_id: int): List<Order>
//     +findAllByStatus(status: OrderStatus): List<Order>
//     +getOrderDetails(order_id: int): OrderDTO;
// }

use sqlx::sqlite::SqlitePool;
use crate::order::Order;

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
                products TEXT NOT NULL
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
            payment_method, products)
            VALUES (?, ?, ?, ?, ?, ?)
            "#;

        // Insert the new order into the database.
        sqlx::query(query)
        .bind(&order.customer_id)
        .bind(&order.status)
        .bind(&order.address)
        .bind(&order.total_amount)
        .bind(&order.payment_method)
        .bind(&order.products)
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
}