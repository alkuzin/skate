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

//! Order service integration tests main module.

use order_service::{
    service::OrderService, config, order::Order,
    create_order, get_order, update_order, delete_order, get_order_list
};
use actix_web::{test, web, App, http::StatusCode};
use serde_json::json;

async fn setup_order_service() -> OrderService {
    let mut service = OrderService::new(config::TEST_DATABASE_PATH).await
        .expect("OrderService::new() should succeed!");

    let _ = service.init().await
        .expect("OrderService::init() should succeed!");

    service
}

#[actix_web::test]
async fn test_setup_order_service() {
    let _ = setup_order_service();
}

#[actix_web::test]
async fn test_create_order() {
    // Create a test app with the service.
    let service = setup_order_service().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(service))
            .route("/orders", web::post().to(create_order))
    ).await;

    let order = json!({
        "dto": {
            "order_id":     1,
            "customer_id":  123,
            "order_status": "Completed",
            "address":      "123 Main St, Anytown, USA",
            "price":        1000
        },
        "items": [
            {
                "order_id":    0,
                "product_id":  456,
                "quantity":    2,
                "unit_price":  500,
                "total_price": 1000
            }
        ]
    });

    // Send a POST request to the /orders endpoint.
    let req = test::TestRequest::post()
        .uri("/orders")
        .set_json(&order)
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
        println!("Created order with ID: {}", id);
    }
    else {
        eprintln!("Error to parse order ID");
    }
}

#[actix_web::test]
async fn test_get_order() {
    // Create a test app with the service.
    let service = setup_order_service().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(service))
            .route("/orders/{id}", web::get().to(get_order))
    ).await;

    let order_id = json!(1);

    // Send a GET request to the /orders/{id} endpoint.
    let req = test::TestRequest::get()
        .uri(format!("/orders/{}", order_id).as_str())
        .set_json(&order_id)
        .to_request();

    // Call a service.
    let response = test::call_service(&app, req).await;
    let status   = response.status();

    // Print the response body.
    let body_bytes  = test::read_body(response).await;
    let body_string = String::from_utf8_lossy(&body_bytes);

    println!("Received response: {}", body_string);
    assert_eq!(status, StatusCode::CREATED);

    if let Ok(order) = serde_json::from_str::<Order>(
        body_string.into_owned().as_str()
    ) {
        println!("Get order: {:#?}", order);
    }
    else {
        eprintln!("Error to parse order");
    }
}

#[actix_web::test]
async fn test_update_order() {
    // Create a test app with the service.
    let service = setup_order_service().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(service))
            .route("/orders/{id}", web::put().to(update_order))
    ).await;

    let order = json!({
        "dto": {
            "order_id":     1,
            "customer_id":  123,
            "order_status": "Completed",
            "address":      "123 Main St, Anytown, USA",
            "price":        1000
        },
        "items": [
            {
                "order_id":    0,
                "product_id":  456,
                "quantity":    2,
                "unit_price":  500,
                "total_price": 1000
            }
        ]
    });

    let order_id = 1;

    // Send a PUT request to the /orders/{id} endpoint.
    let req = test::TestRequest::put()
        .uri(&format!("/orders/{}", order_id))
        .set_json(&order)
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
async fn test_delete_order() {
    // Create a test app with the service.
    let service = setup_order_service().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(service))
            .route("/orders", web::post().to(create_order))
            .route("/orders/{id}", web::delete().to(delete_order))
    ).await;

    // Create new order before deleting it.
    let order = json!({
        "dto": {
            "order_id":     1,
            "customer_id":  123,
            "order_status": "Completed",
            "address":      "123 Main St, Anytown, USA",
            "price":        1000
        },
        "items": [
            {
                "order_id":    0,
                "product_id":  456,
                "quantity":    2,
                "unit_price":  500,
                "total_price": 1000
            }
        ]
    });

    // Send a POST request to the /orders endpoint.
    let req = test::TestRequest::post()
        .uri("/orders")
        .set_json(&order)
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
        println!("Created order with ID: {}", id);

        // Send a DELETE request to the /orders/{id} endpoint.
        let req = test::TestRequest::delete()
            .uri(format!("/orders/{}", id).as_str())
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
        eprintln!("Error to parse order ID");
    }
}

#[actix_web::test]
async fn test_get_order_list() {
    // Create a test app with the service.
    let service = setup_order_service().await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(service))
            .route("/orders", web::get().to(get_order_list))
    ).await;

    // Send a GET request to the /orders endpoint.
    let req = test::TestRequest::get()
        .uri("/orders")
        .to_request();

    // Call a service.
    let response = test::call_service(&app, req).await;
    let status   = response.status();

    // Print the response body.
    let body_bytes  = test::read_body(response).await;
    let body_string = String::from_utf8_lossy(&body_bytes);

    println!("Received response: {}", body_string);
    assert_eq!(status, StatusCode::CREATED);

    if let Ok(order_list) = serde_json::from_str::<Vec<Order>>(
        body_string.into_owned().as_str()
    ) {
        println!("Order list with {} orders:", order_list.len());

        for (i, order) in order_list.iter().enumerate() {
            println!("Order ({}): {:?}", i, order);
        }
    }
    else {
        eprintln!("Error to parse order");
    }
}