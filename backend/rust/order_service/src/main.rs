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

use order_service::{service::OrderService, config, create_order, get_order};
use actix_web::{web::{self, Data}, App, HttpServer};
use std::error::Error;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("[order-service] Running server");
    println!("[order-service] Connecting database");

    let mut service = OrderService::new(config::DATABASE_PATH).await?;
    service.init().await?;

    // Run HTTP server.
    let _ = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(service.clone()))
            .route("/orders",       web::post().to(create_order))
            .route("/orders/{id}",  web::get().to(get_order))
    })
    .bind(config::BIND_ADDRESS)?
    .run()
    .await;

    Ok(())
}

// TODO: implement:
// .route("/orders/{id}", web::put().to(update_order))
// .route("/orders/{id}", web::delete().to(delete_order))
// .route("/orders", web::get().to(list_orders))