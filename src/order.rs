use actix_web::{
    HttpResponse,
    web::{Data, Json, Path},
};
use serde::{Deserialize, Serialize};

use crate::{AppData, Result};

#[derive(Deserialize)]
pub struct OrderRequest {
    pub customer_id: i64,
    pub items: Vec<OrderItemRequest>,
}

#[derive(Deserialize)]
pub struct OrderItemRequest {
    pub product_id: i64,
    pub amount: i64,
}

#[derive(Serialize)]
pub struct OrderResponse {
    pub id: i64,
    pub customer_id: i64,
    pub items: Vec<OrderItemResponse>,
}

#[derive(Serialize)]
pub struct OrderItemResponse {
    pub id: i64,
    pub product_id: i64,
    pub amount: i64,
}

#[actix_web::post("/api/order")]
pub async fn create_order(data: Data<AppData>, body: Json<OrderRequest>) -> Result<HttpResponse> {
    let query_result = sqlx::query!(
        r#"INSERT INTO orders (customer_id)
           VALUES (?)"#,
        body.customer_id
    )
    .execute(&data.db_pool)
    .await?;

    for item in &body.items {
        sqlx::query!(
            r#"INSERT INTO order_items (order_id, product_id, amount)
               VALUES (?, ?, ?)"#,
            query_result.last_insert_id(),
            item.product_id,
            item.amount,
        )
        .execute(&data.db_pool)
        .await?;
    }

    let mut total_order_price = 0i64;
    let mut items_html = String::new();
    for item in body.items.iter() {
        let product_record = sqlx::query!(
            r#"SELECT name, price FROM products WHERE id=?"#,
            item.product_id
        )
        .fetch_one(&data.db_pool)
        .await?;

        let total_price = item.amount * product_record.price;
        total_order_price += total_price;

        items_html.push_str(&format!(
            "<li>Nome do produto: {} - quantidade: {} - preço total: R${}",
            product_record.name, item.amount, total_price
        ));
    }

    let customer_record = sqlx::query!(
        r#"SELECT name, email FROM customers WHERE id=?"#,
        body.customer_id
    )
    .fetch_one(&data.db_pool)
    .await?;

    let message_body = format!(
        "<h3>Confirmação de pedido</h3>\
         <p>Olá, {}! Seu pedido (ID: {}) foi recebido.<br>\
         Total: {}<br></p>\
         <ul>{}</ul>\
         <p>Obrigado por comprar conosco!</p>",
        customer_record.name,
        query_result.last_insert_id(),
        total_order_price,
        items_html
    );

    crate::util::send_html_email(
        &data.config,
        &customer_record.email,
        "Confirmação de pedido",
        message_body,
    )?;

    Ok(HttpResponse::Ok().finish())
}

#[actix_web::put("/api/order/{id:\\d+}")]
pub async fn update_order(
    path: Path<i64>,
    data: Data<AppData>,
    body: Json<OrderRequest>,
) -> Result<HttpResponse> {
    let id = path.into_inner();

    sqlx::query!(
        r#"UPDATE orders
           SET customer_id=?
           WHERE id=?"#,
        body.customer_id,
        id
    )
    .execute(&data.db_pool)
    .await?;

    sqlx::query!("DELETE FROM order_items WHERE order_id=?", id)
        .execute(&data.db_pool)
        .await?;

    for item in &body.items {
        sqlx::query!(
            r#"INSERT INTO order_items (order_id, product_id, amount)
               VALUES (?, ?, ?)"#,
            id,
            item.product_id,
            item.amount
        )
        .execute(&data.db_pool)
        .await?;
    }

    Ok(HttpResponse::Ok().finish())
}

#[actix_web::delete("/api/order/{id:\\d+}")]
pub async fn delete_order(path: Path<i64>, data: Data<AppData>) -> Result<HttpResponse> {
    sqlx::query!("DELETE FROM orders WHERE id=?", path.into_inner())
        .execute(&data.db_pool)
        .await?;

    Ok(HttpResponse::Ok().finish())
}

#[actix_web::get("/api/order/{id:\\d+}")]
pub async fn get_order(path: Path<i64>, data: Data<AppData>) -> Result<HttpResponse> {
    let id = path.into_inner();

    let order_row = sqlx::query!(
        r#"SELECT customer_id
           FROM orders
           WHERE id=?"#,
        id
    )
    .fetch_one(&data.db_pool)
    .await?;

    let order_item_rows = sqlx::query!(
        r#"SELECT id, product_id, amount
           FROM order_items
           WHERE order_id=?"#,
        id
    )
    .fetch_all(&data.db_pool)
    .await?;

    let order_items = order_item_rows
        .iter()
        .map(|row| OrderItemResponse {
            id: row.id,
            product_id: row.product_id,
            amount: row.amount,
        })
        .collect::<Vec<_>>();

    let order = OrderResponse {
        id,
        customer_id: order_row.customer_id,
        items: order_items,
    };

    Ok(HttpResponse::Ok().json(order))
}

#[actix_web::get("/api/order")]
pub async fn get_orders(data: Data<AppData>) -> Result<HttpResponse> {
    let rows = sqlx::query!("SELECT id, customer_id FROM orders")
        .fetch_all(&data.db_pool)
        .await?;

    let mut orders = Vec::new();

    for row in rows.iter() {
        let item_rows = sqlx::query!(
            r#"SELECT id, product_id, amount
               FROM order_items
               WHERE order_id=?"#,
            row.id
        )
        .fetch_all(&data.db_pool)
        .await?;

        let items = item_rows
            .iter()
            .map(|row| OrderItemResponse {
                id: row.id,
                product_id: row.product_id,
                amount: row.amount,
            })
            .collect::<Vec<_>>();

        orders.push(OrderResponse {
            id: row.id,
            customer_id: row.customer_id,
            items,
        });
    }

    Ok(HttpResponse::Ok().json(orders))
}

#[actix_web::get("/api/order/customer/{customer_id:\\d+}")]
pub async fn get_orders_by_customer(path: Path<i64>, data: Data<AppData>) -> Result<HttpResponse> {
    let customer_id = path.into_inner();

    let rows = sqlx::query!(
        r#"SELECT id
           FROM orders
           WHERE customer_id=?"#,
        customer_id
    )
    .fetch_all(&data.db_pool)
    .await?;

    let mut orders = Vec::new();

    for row in rows.iter() {
        let item_rows = sqlx::query!(
            r#"SELECT id, product_id, amount
               FROM order_items
               WHERE order_id=?"#,
            row.id
        )
        .fetch_all(&data.db_pool)
        .await?;

        let items = item_rows
            .iter()
            .map(|row| OrderItemResponse {
                id: row.id,
                product_id: row.product_id,
                amount: row.amount,
            })
            .collect::<Vec<_>>();

        orders.push(OrderResponse {
            id: row.id,
            customer_id,
            items,
        });
    }

    Ok(HttpResponse::Ok().json(orders))
}
