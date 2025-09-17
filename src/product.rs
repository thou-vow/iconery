use actix_web::{
    HttpResponse,
    web::{Data, Json, Path},
};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::{AppData, Result};

#[derive(Deserialize, FromRow, Serialize)]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub price: i64,
    pub is_featured: bool,
}

#[actix_web::post("/api/products")]
pub async fn create_product(data: Data<AppData>, body: Json<Product>) -> Result<HttpResponse> {
    sqlx::query!(
        r#"INSERT INTO products (id, name, description, price, is_featured)
           VALUES (?, ?, ?, ?, ?)"#,
        body.id,
        body.name,
        body.description,
        body.price,
        body.is_featured
    )
    .execute(&data.db_pool)
    .await?;

    Ok(HttpResponse::Ok().finish())
}

#[actix_web::put("/api/products")]
pub async fn update_product(data: Data<AppData>, body: Json<Product>) -> Result<HttpResponse> {
    sqlx::query!(
        r#"UPDATE products
           SET name=?, description=?, price=?, is_featured=?
           WHERE id=?"#,
        body.name,
        body.description,
        body.price,
        body.is_featured,
        body.id
    )
    .execute(&data.db_pool)
    .await?;

    Ok(HttpResponse::Ok().finish())
}

#[actix_web::delete("/api/products/{id}")]
pub async fn delete_product(path: Path<i64>, data: Data<AppData>) -> Result<HttpResponse> {
    sqlx::query!("DELETE FROM products WHERE id=?", path.into_inner())
        .execute(&data.db_pool)
        .await?;

    Ok(HttpResponse::Ok().finish())
}

#[actix_web::get("/api/products/{id}")]
pub async fn get_product(path: Path<i64>, data: Data<AppData>) -> Result<HttpResponse> {
    let query_result = sqlx::query_as!(
        Product,
        r#"SELECT id, name, description, price, is_featured as `is_featured!: _`
           FROM products WHERE id=?"#,
        path.into_inner()
    )
    .fetch_one(&data.db_pool)
    .await;

    match query_result {
        Ok(product) => Ok(HttpResponse::Ok().json(product)),
        Err(sqlx::Error::RowNotFound) => Ok(HttpResponse::NotFound().finish()),
        Err(why) => Err(why.into()),
    }
}

#[actix_web::get("/api/products")]
pub async fn get_products(data: Data<AppData>) -> Result<HttpResponse> {
    let products = sqlx::query_as!(
        Product,
        r#"SELECT id, name, description, price, is_featured as `is_featured!: _`
           FROM products"#
    )
    .fetch_all(&data.db_pool)
    .await?;

    Ok(HttpResponse::Ok().json(products))
}

#[actix_web::get("/api/products/featured")]
pub async fn get_featured_products(data: Data<AppData>) -> Result<HttpResponse> {
    let products = sqlx::query_as!(
        Product,
        r#"SELECT id, name, description, price, is_featured as `is_featured!: bool`
        FROM products WHERE is_featured=TRUE"#
    )
    .fetch_all(&data.db_pool)
    .await?;

    Ok(HttpResponse::Ok().json(products))
}

#[actix_web::get("/api/products/search/{term}")]
pub async fn get_products_with_search(
    path: Path<String>,
    data: Data<AppData>,
) -> Result<HttpResponse> {
    let products = sqlx::query_as!(
        Product,
        r#"SELECT id, name, description, price, is_featured as `is_featured!: _`
           FROM products WHERE name LIKE ?"#,
        path.into_inner()
    )
    .fetch_all(&data.db_pool)
    .await?;

    Ok(HttpResponse::Ok().json(products))
}
