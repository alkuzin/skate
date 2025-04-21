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

//! Order service main module.

pub mod repository;
pub mod service;
pub mod order;
pub mod config;

use actix_web::{HttpResponse, Responder, web::{Data, Json}};
use service::OrderService;
use order::Order;

/// Create new order.
///
/// # Parameters
/// - `service` - given order service data wrapper and extractor.
/// - `order`   - given details of the order to be created.
///
/// # Returns
/// - `HttpResponse::Created` - in case of success.
/// - `HttpResponse::InternalServerError` - otherwise.
pub async fn create_order(service: Data<OrderService>, order: Json<Order>)
    -> impl Responder
{
    match service.create_order(order.0).await {
        Ok(order_id) => HttpResponse::Created().json(order_id),
        Err(e)       => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

/// Get order info.
///
/// # Parameters
/// - `service` - given order service data wrapper and extractor.
/// - `id`      - given order identifier.
///
/// # Returns
/// - `HttpResponse::Created` - in case of success.
/// - `HttpResponse::InternalServerError` - otherwise.
pub async fn get_order(service: Data<OrderService>, id: Json<i64>)
    -> impl Responder
{
    match service.get_order(id.0).await {
        Ok(order) => HttpResponse::Created().json(order),
        Err(e)    => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
