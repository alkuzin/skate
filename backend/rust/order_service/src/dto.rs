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

//! Data Transfer Objects (DTO) structs declarations.

use std::collections::LinkedList;
use crate::order::OrderStatus;
use chrono::{DateTime, Utc};

/// Product filter data transfer object struct.
#[derive(Debug, Default)]
pub struct ProductFilterDTO {
    /// Product category.
    pub category: String,
    /// Product minimum price.
    pub min_price: f32,
    /// Product maximum price.
    pub max_price: f32,
}

impl ProductFilterDTO {
    /// Construct new ProductFilterDTO object.
    ///
    /// # Parameters
    /// - `category`  - given product filter category.
    /// - `min_price` - given minimum price of product.
    /// - `max_price` - given maximum price of product.
    ///
    /// # Returns
    /// New ProductFilterDTO object.
    pub fn new(category: String, min_price: f32, max_price: f32) -> Self {
        Self { category, max_price, min_price }
    }
}

/// Product item data transfer object struct.
#[derive(Debug, Default)]
pub struct ProductItemDTO {
    /// Product identificator.
    pub product_id: i32,
    /// Product name.
    pub name: String,
    /// Total number of products.
    pub quantity: i32,
    /// Product price.
    pub price: f32,
}

/// Order data transfer object struct.
#[derive(Debug, Default)]
pub struct OrderDTO {
    /// Order identificator.
    pub id: i32,
    /// Order customer identificator.
    pub customer_id: i32,
    /// Order status.
    pub status: OrderStatus,
    /// Order destination address.
    pub address: String,
    /// Order creation time.
    pub created_at: DateTime<Utc>,
    /// Order last update time.
    pub updated_at: DateTime<Utc>,
    /// Order method of payment.
    pub payment_method: String,
    /// Courier phone number.
    pub courier_contact: String,
    /// List of products.
    pub products: LinkedList<ProductItemDTO>,
}