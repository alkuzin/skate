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

/// Order info struct.
#[derive(Debug, Default, FromRow)]
pub struct Order {
    /// Order identificator.
    pub id: i32,
    /// Order customer identificator.
    pub customer_id: i32,
    /// Order status.
    pub status: String,
    /// Order destination address.
    pub address: String,
    /// Order price.
    pub total_amount: f32,
    /// Order method of payment.
    pub payment_method: String,
    /// List of products IDs.
    pub products: String,
}

/// Order status enumeration.
#[derive(Debug, Default)]
pub enum OrderStatus {
    #[default]
    Processing,
    Accepted,
    Assembly,
    InProgress,
    Completed,
    Cancelled,
}

impl OrderStatus {
    /// Get order status as string representation.
    ///
    /// # Returns
    /// String slice representation of order status.
    pub fn as_str(&self) -> &str {
        match self {
            OrderStatus::Processing => "В обработке",
            OrderStatus::Accepted => "Принят",
            OrderStatus::Assembly => "В сборке",
            OrderStatus::InProgress => "В доставке",
            OrderStatus::Completed => "Завершен",
            OrderStatus::Cancelled => "Отменен",
        }
    }
}

impl From<&str> for OrderStatus {
    /// Convert string slice to order status.
    ///
    /// # Parameters
    /// - `value` - given string slice to convert.
    ///
    /// # Returns
    /// Order status enumeration representation of given value.
    fn from(value: &str) -> Self {
        match value {
             "В обработке" => OrderStatus::Processing,
             "Принят"      => OrderStatus::Accepted,
             "В сборке"    => OrderStatus::Assembly,
             "В доставке"  => OrderStatus::InProgress,
             "Завершен"    => OrderStatus::Completed,
             "Отменен"     => OrderStatus::Cancelled,
            _              => OrderStatus::Cancelled,
        }
    }
}
