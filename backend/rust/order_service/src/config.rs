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

//! Service configuration data.

/// Service bind address.
pub const BIND_ADDRESS: &str = "127.0.0.1:8080";

/// Service SQLite database path.
pub const DATABASE_PATH: &str = "../db/orders.db";

/// Service SQLite database path for unit tests.
pub const TEST_DATABASE_PATH: &str = "../db/test_orders.db";