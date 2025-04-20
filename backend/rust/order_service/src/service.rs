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

use crate::{config, repository::OrderRepository};
use sqlx::SqlitePool;

#[derive(Debug)]
pub struct OrderService {
    /// Struct for communicating with database.
    repository: OrderRepository,
}

impl OrderService {
    /// Construct new OrderService object.
    ///
    /// # Returns
    /// - New `OrderRepository` object - in case of success.
    /// - `SQLx error` - otherwise.
    pub async fn new() -> Result<Self, sqlx::Error> {
        let pool = SqlitePool::connect(config::DATABASE_PATH).await?;
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
}