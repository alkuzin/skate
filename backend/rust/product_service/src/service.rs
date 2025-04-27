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

use std::{fs::File, path::Path};
use sqlx::SqlitePool;

#[derive(Debug, Clone)]
pub struct ProductService {
    // TODO: add struct for communicating with database.
    // repository: ProductRepository,
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

        let _pool = SqlitePool::connect(path).await?;
        Ok(Self { /* TODO: Init ProductRepository. */ })
    }
}
