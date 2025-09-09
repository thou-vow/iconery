use actix_web::{
    App, HttpResponse, HttpServer, Responder,
    web::{Data, Json},
};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, postgres::PgPoolCopyExt, prelude::FromRow};
use uuid::Uuid;

#[derive(Serialize, FromRow, Debug)]
struct Product {
    id: Uuid,
    name: String,
    description: Option<String>,
    price_cents: i64,
    is_featured: bool,
}

#[derive(Deserialize)]
struct CreateProduct {
    name: String,
    description: Option<String>,
    price_cents: i64,
    is_featured: Option<bool>,
}

#[derive(Deserialize)]
struct UpdateProduct {
    name: Option<String>,
    description: Option<String>,
    price_cents: Option<i64>,
    is_featured: Option<bool>,
}

// async fn create_product(pool: Data<PgPool>, body: Json<CreateProduct>) -> impl Responder {
//     let id = Uuid::new_v4();
//     let is_featured = body.is_featured.unwrap_or(false);

//     let q = sqlx::query!(
//         r#"INSERT INTO products (id, name, description, price_cents, is_featured)
//            VALUES ($1, $2, $3, $4, $5)"#,
//         id,
//         body.name.body.description,
//         body.price_cents,
//         is_featured
//     )
//     .execute(&pool)
//     .await;
// }

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/hey", actix_web::web::get().to(hello)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
