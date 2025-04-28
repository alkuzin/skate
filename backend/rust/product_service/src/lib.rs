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

//! Product service main module.

pub mod category_repository;
pub mod product_repository;
pub mod service;
pub mod product;
pub mod config;

use crate::{service::ProductService, product::{Product, Category}};
use actix_web::{HttpResponse, Responder, web::{Data, Json}, web};

/// Create new product.
///
/// # Parameters
/// - `service` - given product service data wrapper and extractor.
/// - `product` - given details of the product to be created.
///
/// # Returns
/// - `HttpResponse::Created` - in case of success.
/// - `HttpResponse::InternalServerError` - otherwise.
pub async fn create_product(service: Data<ProductService>,
                            product: Json<Product>) -> impl Responder
{
    match service.create_product(product.0).await {
        Ok(product_id) => HttpResponse::Created().json(product_id),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

/// Create new category.
///
/// # Parameters
/// - `service`  - given product service data wrapper and extractor.
/// - `category` - given details of the product category to be created.
///
/// # Returns
/// - `HttpResponse::Created` - in case of success.
/// - `HttpResponse::InternalServerError` - otherwise.
pub async fn create_category(service: Data<ProductService>,
                             category: Json<Category>) -> impl Responder
{
    match service.create_category(category.0).await {
        Ok(product_id) => HttpResponse::Created().json(product_id),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

/// Get product info.
///
/// # Parameters
/// - `service` - given product service data wrapper and extractor.
/// - `id`      - given product identifier.
///
/// # Returns
/// - `HttpResponse::Created` - in case of success.
/// - `HttpResponse::InternalServerError` - otherwise.
pub async fn get_product(service: Data<ProductService>, id: web::Path<i64>)
    -> impl Responder
{
    match service.get_product(id.into_inner()).await {
        Ok(product) => HttpResponse::Created().json(product),
        Err(e)      => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

/// Get category info.
///
/// # Parameters
/// - `service` - given product service data wrapper and extractor.
/// - `id`      - given product category identifier.
///
/// # Returns
/// - `HttpResponse::Created` - in case of success.
/// - `HttpResponse::InternalServerError` - otherwise.
pub async fn get_category(service: Data<ProductService>, id: web::Path<i64>)
    -> impl Responder
{
    match service.get_category(id.into_inner()).await {
        Ok(product) => HttpResponse::Created().json(product),
        Err(e)      => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

/// Update product info.
///
/// # Parameters
/// - `service` - given product service data wrapper and extractor.
/// - `id`      - given product identifier.
/// - `product` - given details of the product to be updated.
///
/// # Returns
/// - `HttpResponse::Created` - in case of success.
/// - `HttpResponse::InternalServerError` - otherwise.
pub async fn update_product(service: Data<ProductService>, id: web::Path<i64>,
                            product: Json<Product>) -> impl Responder
{
    match service.update_product(id.into_inner(), product.0).await {
        Ok(_)  => HttpResponse::Created().body("Ok"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

/// Update category info.
///
/// # Parameters
/// - `service` - given product service data wrapper and extractor.
/// - `id`      - given product category identifier.
/// - `product` - given details of the product category to be updated.
///
/// # Returns
/// - `HttpResponse::Created` - in case of success.
/// - `HttpResponse::InternalServerError` - otherwise.
pub async fn update_category(service: Data<ProductService>, id: web::Path<i64>,
                            category: Json<Category>) -> impl Responder
{
    match service.update_category(id.into_inner(), category.0).await {
        Ok(_)  => HttpResponse::Created().body("Ok"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

/// Delete product info.
///
/// # Parameters
/// - `service` - given product service data wrapper and extractor.
/// - `id`      - given product identifier.
///
/// # Returns
/// - `HttpResponse::Created` - in case of success.
/// - `HttpResponse::InternalServerError` - otherwise.
pub async fn delete_product(service: Data<ProductService>, id: web::Path<i64>)
    -> impl Responder
{
    match service.delete_product(id.into_inner()).await {
        Ok(_)  => HttpResponse::Created().json("Ok"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

/// Delete category info.
///
/// # Parameters
/// - `service` - given product service data wrapper and extractor.
/// - `id`      - given product category identifier.
///
/// # Returns
/// - `HttpResponse::Created` - in case of success.
/// - `HttpResponse::InternalServerError` - otherwise.
pub async fn delete_category(service: Data<ProductService>, id: web::Path<i64>)
    -> impl Responder
{
    match service.delete_category(id.into_inner()).await {
        Ok(_)  => HttpResponse::Created().json("Ok"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

/// Get product info list.
///
/// # Parameters
/// - `service` - given product service data wrapper and extractor.
///
/// # Returns
/// - `HttpResponse::Created` - in case of success.
/// - `HttpResponse::InternalServerError` - otherwise.
pub async fn get_product_list(service: Data<ProductService>) -> impl Responder
{
    match service.get_product_list().await {
        Ok(products) => HttpResponse::Created().json(products),
        Err(e)       => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

/// Get product category info list.
///
/// # Parameters
/// - `service` - given product service data wrapper and extractor.
///
/// # Returns
/// - `HttpResponse::Created` - in case of success.
/// - `HttpResponse::InternalServerError` - otherwise.
pub async fn get_category_list(service: Data<ProductService>) -> impl Responder
{
    match service.get_category_list().await {
        Ok(products) => HttpResponse::Created().json(products),
        Err(e)       => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

/// Get product category info list.
///
/// # Parameters
/// - `service` - given product service data wrapper and extractor.
/// - `id`      - given product category identifier.
///
/// # Returns
/// - `HttpResponse::Created` - in case of success.
/// - `HttpResponse::InternalServerError` - otherwise.
pub async fn get_products_by_category(service: Data<ProductService>,
                                      id: web::Path<i64>) -> impl Responder
{
    match service.get_products_by_category(id.into_inner()).await {
        Ok(products) => HttpResponse::Created().json(products),
        Err(e)       => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
