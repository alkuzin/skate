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

//! Order service entry point.

#[allow(dead_code)]
mod repository;
mod service;
mod order;
mod config;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use service::OrderService;
use std::error::Error;

async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Order Management Microservice")
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("[order-service] Running server");
    println!("[order-service] Connecting database");

    let mut service = OrderService::new(config::DATABASE_PATH).await?;
    service.init().await?;

    // Run HTTP server.
    let _ = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet)) // GET /
    })
    .bind(config::BIND_ADDRESS)?
    .run()
    .await;

    Ok(())
}
