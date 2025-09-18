use actix_web::{
    HttpResponse,
    web::{Data, Json, Path},
};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::{AppData, Result};

#[derive(Deserialize)]
pub struct ProductRequest {
    pub name: String,
    pub description: Option<String>,
    pub price: i64,
    pub is_featured: bool,
}

#[derive(FromRow, Serialize)]
pub struct ProductResponse {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub price: i64,
    pub is_featured: bool,
}

#[actix_web::post("/api/product")]
pub async fn create_product(
    data: Data<AppData>,
    body: Json<ProductRequest>,
) -> Result<HttpResponse> {
    sqlx::query!(
        r#"INSERT INTO products (name, description, price, is_featured)
           VALUES (?, ?, ?, ?)"#,
        body.name,
        body.description,
        body.price,
        body.is_featured
    )
    .execute(&data.db_pool)
    .await?;

    Ok(HttpResponse::Ok().finish())
}

#[actix_web::put("/api/product/{id}")]
pub async fn update_product(
    path: Path<i64>,
    data: Data<AppData>,
    body: Json<ProductRequest>,
) -> Result<HttpResponse> {
    sqlx::query!(
        r#"UPDATE products
           SET name=?, description=?, price=?, is_featured=?
           WHERE id=?"#,
        body.name,
        body.description,
        body.price,
        body.is_featured,
        path.into_inner()
    )
    .execute(&data.db_pool)
    .await?;

    Ok(HttpResponse::Ok().finish())
}

#[actix_web::delete("/api/product/{id:\\d+}")]
pub async fn delete_product(path: Path<i64>, data: Data<AppData>) -> Result<HttpResponse> {
    sqlx::query!("DELETE FROM products WHERE id=?", path.into_inner())
        .execute(&data.db_pool)
        .await?;

    Ok(HttpResponse::Ok().finish())
}

#[actix_web::get("/api/product/{id:\\d+}")]
pub async fn get_product(path: Path<i64>, data: Data<AppData>) -> Result<HttpResponse> {
    let product = sqlx::query_as!(
        ProductResponse,
        r#"SELECT id, name, description, price, is_featured as `is_featured!: bool`
           FROM products WHERE id=?"#,
        path.into_inner()
    )
    .fetch_one(&data.db_pool)
    .await?;

    Ok(HttpResponse::Ok().json(product))
}

#[actix_web::get("/api/product")]
pub async fn get_products(data: Data<AppData>) -> Result<HttpResponse> {
    let products = sqlx::query_as!(
        ProductResponse,
        r#"SELECT id, name, description, price, is_featured as `is_featured!: bool`
           FROM products"#
    )
    .fetch_all(&data.db_pool)
    .await?;

    Ok(HttpResponse::Ok().json(products))
}

#[actix_web::get("/api/product/featured")]
pub async fn get_featured_products(data: Data<AppData>) -> Result<HttpResponse> {
    let products = sqlx::query_as!(
        ProductResponse,
        r#"SELECT id, name, description, price, is_featured as `is_featured: bool`
        FROM products WHERE is_featured=TRUE"#
    )
    .fetch_all(&data.db_pool)
    .await?;

    Ok(HttpResponse::Ok().json(products))
}

#[actix_web::get("/api/product/search/{term}")]
pub async fn get_products_with_search(
    path: Path<String>,
    data: Data<AppData>,
) -> Result<HttpResponse> {
    let products = sqlx::query_as!(
        ProductResponse,
        r#"SELECT id, name, description, price, is_featured as `is_featured!: bool`
           FROM products WHERE name LIKE ?"#,
        path.into_inner()
    )
    .fetch_all(&data.db_pool)
    .await?;

    Ok(HttpResponse::Ok().json(products))
}
