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

//! Product service entry point.

use product_service::{
    config, service::ProductService, create_category, create_product,
    delete_category, delete_product, get_category, get_category_list,
    get_product, get_product_list, get_products_by_category, update_category,
    update_product
};
use actix_web::{web, web::Data, App, HttpServer};
use std::error::Error;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("[product-service] Running server");
    println!("[product-service] Connecting database");

    let service = ProductService::new(config::DATABASE_PATH).await?;

    // Run HTTP server.
    let _ = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(service.clone()))
            .route("/products", web::post().to(create_product))
            .route("/categories", web::post().to(create_category))
            .route("/products/{id}", web::get().to(get_product))
            .route("/categories/{id}", web::get().to(get_category))
            .route("/products/{id}", web::put().to(update_product))
            .route("/categories/{id}", web::put().to(update_category))
            .route("/products/{id}", web::delete().to(delete_product))
            .route("/categories/{id}", web::delete().to(delete_category))
            .route("/products", web::get().to(get_product_list))
            .route("/categories", web::get().to(get_category_list))
            .route(
                "/categories/{id}/products",
                web::get().to(get_products_by_category)
            )
    })
        .bind(config::BIND_ADDRESS)?
        .run()
        .await;

    Ok(())
}