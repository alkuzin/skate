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

//! Product service integration tests main module.

use product_service::{
    create_product, create_category, get_product, get_category, update_product, update_category,
    delete_product, delete_category, get_product_list, get_category_list,
    service::ProductService, config, product::{Category, Product}
};
use actix_web::{test, web, App, http::StatusCode};
use serde_json::json;

async fn setup_product_service() -> ProductService {
    let service = ProductService::new(config::TEST_DATABASE_PATH).await
        .expect("ProductService::new() should succeed!");

    service
}

#[actix_web::test]
async fn test_setup_product_service() {
    let _ = setup_product_service();
}

#[actix_web::test]
async fn test_create_product() {
    // Create a test app with the service.
    let service = setup_product_service().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(service))
            .route("/products", web::post().to(create_product))
    ).await;

    let product = json!({
        "product_id"  : 0,
        "category_id" : 1234,
        "name"        : "Cool book",
        "description" : "Cool book 10/10",
        "price"       : 9999,
        "quantity"    : 1,
        "image"       : "~/images/cool-book.png",
    });

    // Send a POST request to the /products endpoint.
    let req = test::TestRequest::post()
        .uri("/products")
        .set_json(&product)
        .to_request();

    // Call a service.
    let response = test::call_service(&app, req).await;
    let status   = response.status();

    // Print the response body.
    let body_bytes  = test::read_body(response).await;
    let body_string = String::from_utf8_lossy(&body_bytes);

    println!("Received response: {}", body_string);
    assert_eq!(status, StatusCode::CREATED);

    if let Ok(id) = body_string.trim().parse::<i64>() {
        println!("Created product with ID: {}", id);
    }
    else {
        eprintln!("Error to parse product ID");
    }
}

#[actix_web::test]
async fn test_create_category() {
    // Create a test app with the service.
    let service = setup_product_service().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(service))
            .route("/categories", web::post().to(create_category))
    ).await;

    let category = json!({
        "category_id" : 0,
        "name"        : "Books",
        "image"       : "~/images/category-book.png",
    });

    // Send a POST request to the /categories endpoint.
    let req = test::TestRequest::post()
        .uri("/categories")
        .set_json(&category)
        .to_request();

    // Call a service.
    let response = test::call_service(&app, req).await;
    let status   = response.status();

    // Print the response body.
    let body_bytes  = test::read_body(response).await;
    let body_string = String::from_utf8_lossy(&body_bytes);

    println!("Received response: {}", body_string);
    assert_eq!(status, StatusCode::CREATED);

    if let Ok(id) = body_string.trim().parse::<i64>() {
        println!("Created category with ID: {}", id);
    }
    else {
        eprintln!("Error to parse category ID");
    }
}

#[actix_web::test]
async fn test_get_product() {
    // Create a test app with the service.
    let service = setup_product_service().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(service))
            .route("/products/{id}", web::get().to(get_product))
    ).await;

    let product_id = json!(1);

    // Send a GET request to the /products/{id} endpoint.
    let req = test::TestRequest::get()
        .uri(format!("/products/{}", product_id).as_str())
        .set_json(&product_id)
        .to_request();

    // Call a service.
    let response = test::call_service(&app, req).await;
    let status   = response.status();

    // Print the response body.
    let body_bytes  = test::read_body(response).await;
    let body_string = String::from_utf8_lossy(&body_bytes);

    println!("Received response: {}", body_string);
    assert_eq!(status, StatusCode::CREATED, "DB is empty");

    if let Ok(product) = serde_json::from_str::<Product>(
        body_string.into_owned().as_str()
    ) {
        println!("Get product: {:#?}", product);
    }
    else {
        eprintln!("Error to parse product");
    }
}

#[actix_web::test]
async fn test_get_category() {
    // Create a test app with the service.
    let service = setup_product_service().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(service))
            .route("/categories/{id}", web::get().to(get_category))
    ).await;

    let category_id = json!(1);

    // Send a GET request to the /category/{id} endpoint.
    let req = test::TestRequest::get()
        .uri(format!("/categories/{}", category_id).as_str())
        .set_json(&category_id)
        .to_request();

    // Call a service.
    let response = test::call_service(&app, req).await;
    let status   = response.status();

    // Print the response body.
    let body_bytes  = test::read_body(response).await;
    let body_string = String::from_utf8_lossy(&body_bytes);

    println!("Received response: {}", body_string);
    assert_eq!(status, StatusCode::CREATED, "DB is empty");

    if let Ok(category) = serde_json::from_str::<Category>(
        body_string.into_owned().as_str()
    ) {
        println!("Get product category: {:#?}", category);
    }
    else {
        eprintln!("Error to parse product category");
    }
}

#[actix_web::test]
async fn test_update_product() {
    // Create a test app with the service.
    let service = setup_product_service().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(service))
            .route("/products/{id}", web::put().to(update_product))
    ).await;

    let product = json!({
        "product_id"  :  0,
        "category_id" : 5678,
        "name"        : "Magnificent book",
        "description" : "Magnificent book 100/10",
        "price"       : 9999999,
        "quantity"    : 1,
        "image"       : "~/images/magnificent-book.png",
    });

    let product_id = 1;

    // Send a PUT request to the /products/{id} endpoint.
    let req = test::TestRequest::put()
        .uri(&format!("/products/{}", product_id))
        .set_json(&product)
        .to_request();

    // Call a service.
    let response = test::call_service(&app, req).await;
    let status   = response.status();

    // Print the response body.
    let body_bytes  = test::read_body(response).await;
    let body_string = String::from_utf8_lossy(&body_bytes);

    println!("Received response: {}", body_string);
    assert_eq!(status, StatusCode::CREATED);
}

#[actix_web::test]
async fn test_update_category() {
    // Create a test app with the service.
    let service = setup_product_service().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(service))
            .route("/categories/{id}", web::put().to(update_category))
    ).await;

    let category = json!({
        "category_id" : 0,
        "name"        : "Tech",
        "image"       : "~/images/category-tech.png",
    });

    let category_id = 1;

    // Send a PUT request to the /products/{id} endpoint.
    let req = test::TestRequest::put()
        .uri(&format!("/categories/{}", category_id))
        .set_json(&category)
        .to_request();

    // Call a service.
    let response = test::call_service(&app, req).await;
    let status   = response.status();

    // Print the response body.
    let body_bytes  = test::read_body(response).await;
    let body_string = String::from_utf8_lossy(&body_bytes);

    println!("Received response: {}", body_string);
    assert_eq!(status, StatusCode::CREATED);
}

#[actix_web::test]
async fn test_delete_product() {
    // Create a test app with the service.
    let service = setup_product_service().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(service))
            .route("/products", web::post().to(create_product))
            .route("/products/{id}", web::delete().to(delete_product))
    ).await;

    // Create new product before deleting it.
    let product = json!({
        "product_id"  : 0,
        "category_id" : 1234,
        "name"        : "Cool book",
        "description" : "Cool book 10/10",
        "price"       : 9999,
        "quantity"    : 1,
        "image"       : "~/images/cool-book.png",
    });

    // Send a POST request to the /products endpoint.
    let req = test::TestRequest::post()
        .uri("/products")
        .set_json(&product)
        .to_request();

    // Call a service.
    let response = test::call_service(&app, req).await;
    let status   = response.status();

    // Print the response body.
    let body_bytes  = test::read_body(response).await;
    let body_string = String::from_utf8_lossy(&body_bytes);

    println!("Received response: {}", body_string);
    assert_eq!(status, StatusCode::CREATED);

    if let Ok(id) = body_string.trim().parse::<i64>() {
        println!("Created product with ID: {}", id);

        // Send a DELETE request to the /products/{id} endpoint.
        let req = test::TestRequest::delete()
            .uri(format!("/products/{}", id).as_str())
            .to_request();

        // Call a service.
        let response = test::call_service(&app, req).await;
        let status   = response.status();

        // Print the response body.
        let body_bytes  = test::read_body(response).await;
        let body_string = String::from_utf8_lossy(&body_bytes);

        println!("Received response: {}", body_string);
        assert_eq!(status, StatusCode::CREATED);
    }
    else {
        eprintln!("Error to parse product ID");
    }
}

#[actix_web::test]
async fn test_delete_category() {
    // Create a test app with the service.
    let service = setup_product_service().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(service))
            .route("/categories", web::post().to(create_category))
            .route("/categories/{id}", web::delete().to(delete_category))
    ).await;

    // Create new product category before deleting it.
    let category = json!({
        "category_id" : 0,
        "name"        : "Books",
        "image"       : "~/images/category-book.png",
    });

    // Send a POST request to the /categories endpoint.
    let req = test::TestRequest::post()
        .uri("/categories")
        .set_json(&category)
        .to_request();

    // Call a service.
    let response = test::call_service(&app, req).await;
    let status   = response.status();

    // Print the response body.
    let body_bytes  = test::read_body(response).await;
    let body_string = String::from_utf8_lossy(&body_bytes);

    println!("Received response: {}", body_string);
    assert_eq!(status, StatusCode::CREATED);

    if let Ok(id) = body_string.trim().parse::<i64>() {
        println!("Created category with ID: {}", id);

        // Send a DELETE request to the /categories/{id} endpoint.
        let req = test::TestRequest::delete()
            .uri(format!("/categories/{}", id).as_str())
            .to_request();

        // Call a service.
        let response = test::call_service(&app, req).await;
        let status   = response.status();

        // Print the response body.
        let body_bytes  = test::read_body(response).await;
        let body_string = String::from_utf8_lossy(&body_bytes);

        println!("Received response: {}", body_string);
        assert_eq!(status, StatusCode::CREATED);
    }
    else {
        eprintln!("Error to parse product ID");
    }
}

#[actix_web::test]
async fn test_get_product_list() {
    // Create a test app with the service.
    let service = setup_product_service().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(service))
            .route("/products", web::get().to(get_product_list))
    ).await;

    // Send a GET request to the /products endpoint.
    let req = test::TestRequest::get()
        .uri("/products")
        .to_request();

    // Call a service.
    let response = test::call_service(&app, req).await;
    let status   = response.status();

    // Print the response body.
    let body_bytes  = test::read_body(response).await;
    let body_string = String::from_utf8_lossy(&body_bytes);

    println!("Received response: {}", body_string);
    assert_eq!(status, StatusCode::CREATED);

    if let Ok(product_list) = serde_json::from_str::<Vec<Product>>(
        body_string.into_owned().as_str()
    ) {
        println!("Product list with {} products:", product_list.len());

        for (i, product) in product_list.iter().enumerate() {
            println!("Product ({}): {:?}", i, product);
        }
    }
    else {
        eprintln!("Error to parse product");
    }
}

#[actix_web::test]
async fn test_get_category_list() {
    // Create a test app with the service.
    let service = setup_product_service().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(service))
            .route("/categories", web::get().to(get_category_list))
    ).await;

    // Send a GET request to the /categories endpoint.
    let req = test::TestRequest::get()
        .uri("/categories")
        .to_request();

    // Call a service.
    let response = test::call_service(&app, req).await;
    let status   = response.status();

    // Print the response body.
    let body_bytes  = test::read_body(response).await;
    let body_string = String::from_utf8_lossy(&body_bytes);

    println!("Received response: {}", body_string);
    assert_eq!(status, StatusCode::CREATED);

    if let Ok(category_list) = serde_json::from_str::<Vec<Category>>(
        body_string.into_owned().as_str()
    ) {
        println!("Category list with {} categories:", category_list.len());

        for (i, category) in category_list.iter().enumerate() {
            println!("Category ({}): {:?}", i, category);
        }
    }
    else {
        eprintln!("Error to parse category");
    }
}