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

//! Order related declarations.

use sqlx::FromRow;

/// Order data transfer object struct.
#[derive(Debug, Default, FromRow)]
pub struct OrderDTO {
    /// Order identifier.
    pub order_id: i32,
    /// Order customer identifier.
    pub customer_id: i32,
    /// Order status.
    pub order_status: OrderStatus,
    /// Order delivery address.
    pub address: String,
    /// Order price.
    pub price: i64,
}

/// Order info struct.
#[derive(Debug, Default)]
pub struct Order {
    /// Order data transfer object.
    pub dto: OrderDTO,
    /// List of products associated with this order.
    pub items: Vec<OrderItem>,
}

impl Order {
    pub fn new(dto: OrderDTO, items: Vec<OrderItem>) -> Self {
        Self { dto, items }
    }
}

/// Struct for storing information about specific products in an order.
#[derive(Debug, Default, FromRow)]
pub struct OrderItem {
    /// Order identifier.
    pub order_id: i32,
    /// Product identifier.
    pub product_id: i32,
    /// Number of products.
    pub quantity: i32,
    /// Price of each product.
    pub unit_price: i64,
    /// Total price of an item.
    pub total_price: i64,
}

/// Order status enumeration.
#[derive(Debug, Default, sqlx::Type)]
#[repr(i32)]
pub enum OrderStatus {
    #[default]
    Processing = 0,
    Accepted   = 1,
    Assembly   = 2,
    InProgress = 3,
    Completed  = 4,
    Cancelled  = 5,
}

impl From<i32> for OrderStatus {
    /// Convert string slice to order status.
    ///
    /// # Parameters
    /// - `value` - given unsigned char to convert.
    ///
    /// # Returns
    /// Order status enumeration representation of given value.
    fn from(value: i32) -> Self {
        match value {
            0 => OrderStatus::Processing,
            1 => OrderStatus::Accepted,
            2 => OrderStatus::Assembly,
            3 => OrderStatus::InProgress,
            4 => OrderStatus::Completed,
            5 => OrderStatus::Cancelled,
            _ => OrderStatus::Cancelled,
        }
    }
}