use actix_web::{
    HttpResponse,
    web::{Data, Json, Path},
};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::{AppData, Result};

#[derive(Deserialize)]
pub struct CustomerRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub phone_number: Option<String>,
    pub address: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(FromRow, Serialize)]
pub struct CustomerResponse {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password: String,
    pub phone_number: Option<String>,
    pub address: Option<String>,
    pub is_active: bool,
}

#[derive(Deserialize)]
pub struct CustomerLoginRequest {
    pub email: String,
    pub password: String,
}

#[actix_web::post("/api/customer")]
pub async fn create_customer(
    data: Data<AppData>,
    body: Json<CustomerRequest>,
) -> Result<HttpResponse> {
    let hashed_password = crate::util::hash(&body.password);

    sqlx::query!(
        r#"INSERT INTO customers (name, email, password, phone_number, address, is_active)
           VALUES (?, ?, ?, ?, ?, ?)"#,
        body.name,
        body.email,
        hashed_password,
        body.phone_number,
        body.address,
        body.is_active.unwrap_or(false),
    )
    .execute(&data.db_pool)
    .await?;

    let confirm_link = format!(
        "http://{}:{}/api/customer/activate/{}",
        data.config.server_host,
        data.config.server_port,
        crate::util::hash(&body.email)
    );
    let message_body = format!(
        "<b>Email de confirmação de cadastro</b><br><br>\
         Seja bem-vindo(a), {}! Clique no link abaixo para confirmar seu cadastro.<br><br>\
         <a href='{}'>Clique aqui</a>",
        body.name, confirm_link
    );

    crate::util::send_html_email(
        &data.config,
        &body.email,
        "Confirmação de novo cadastro",
        message_body,
    )?;

    Ok(HttpResponse::Ok().finish())
}

#[actix_web::put("/api/customer/{id:\\d+}")]
pub async fn update_customer(
    path: Path<i64>,
    data: Data<AppData>,
    body: Json<CustomerRequest>,
) -> Result<HttpResponse> {
    let hashed_password = crate::util::hash(&body.password);

    sqlx::query!(
        r#"UPDATE customers
           SET name=?, email=?, password=?, phone_number=?, address=?, is_active=?
           WHERE id=?"#,
        body.name,
        body.email,
        hashed_password,
        body.phone_number,
        body.address,
        body.is_active.unwrap_or(false),
        path.into_inner()
    )
    .execute(&data.db_pool)
    .await?;

    Ok(HttpResponse::Ok().finish())
}

#[actix_web::delete("/api/customer/{id:\\d+}")]
pub async fn delete_customer(path: Path<i64>, data: Data<AppData>) -> Result<HttpResponse> {
    sqlx::query!("DELETE FROM customers WHERE id=?", path.into_inner())
        .execute(&data.db_pool)
        .await?;

    Ok(HttpResponse::Ok().finish())
}

#[actix_web::get("/api/customer/{id:\\d+}")]
pub async fn get_customer(path: Path<i64>, data: Data<AppData>) -> Result<HttpResponse> {
    let customer = sqlx::query_as!(
        CustomerResponse,
        r#"SELECT id, name, email, password, phone_number, address, is_active as `is_active: _`
           FROM customers WHERE id=?"#,
        path.into_inner()
    )
    .fetch_one(&data.db_pool)
    .await?;

    Ok(HttpResponse::Ok().json(customer))
}

#[actix_web::get("/api/customer")]
pub async fn get_customers(data: Data<AppData>) -> Result<HttpResponse> {
    let customers = sqlx::query_as!(
        CustomerResponse,
        r#"SELECT id, name, email, password, phone_number, address, is_active as `is_active: _`
           FROM customers"#
    )
    .fetch_all(&data.db_pool)
    .await?;

    Ok(HttpResponse::Ok().json(customers))
}

#[actix_web::get("/api/customer/login")]
pub async fn login_customer(
    data: Data<AppData>,
    body: Json<CustomerLoginRequest>,
) -> Result<HttpResponse> {
    let hashed_password = crate::util::hash(&body.password);

    let maybe_customer = sqlx::query_as!(
        CustomerResponse,
        r#"SELECT id, name, email, password, phone_number, address, is_active as `is_active: _`
        FROM customers
        WHERE email=? AND password=? AND is_active=TRUE"#,
        body.email,
        hashed_password
    )
    .fetch_optional(&data.db_pool)
    .await?;

    match maybe_customer {
        Some(customer) => Ok(HttpResponse::Ok().json(customer)),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

#[actix_web::get("/api/customer/activate/{token}")]
pub async fn activate_customer(path: Path<String>, data: Data<AppData>) -> Result<HttpResponse> {
    let query_result = sqlx::query!(
        "UPDATE customers SET is_active=TRUE WHERE MD5(email)=?",
        path.into_inner()
    )
    .execute(&data.db_pool)
    .await?;

    if query_result.rows_affected() > 0 {
        Ok(HttpResponse::Ok().body("Cliente ativado!"))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

#[actix_web::get("/api/customer/reset-password/{email}")]
pub async fn send_password_reset(path: Path<String>, data: Data<AppData>) -> Result<HttpResponse> {
    let email = path.into_inner();

    let reset_link = format!(
        "https://{}:{}/reset-password-form?token={}",
        data.config.server_host,
        data.config.server_port,
        crate::util::hash(&email)
    );

    let message_body = format!(
        "<p>Você solicitou redefinição de senha</p>\
         <p>Olá! Clique no link abaixo para redefinir a senha:</p>\
         <p><a href='{}'>Redefinir senha</a></p>",
        reset_link
    );

    crate::util::send_html_email(&data.config, &email, "Redefinição de senha", message_body)?;

    Ok(HttpResponse::Ok().finish())
}

#[actix_web::post("/api/customer/reset-password/{token}")]
pub async fn password_reset(
    path: Path<String>,
    data: Data<AppData>,
    body: Json<String>,
) -> Result<HttpResponse> {
    let hashed_password = crate::util::hash(&body);

    let query_result = sqlx::query!(
        r#"UPDATE customers
           SET password=?
           WHERE MD5(email)=?"#,
        hashed_password,
        path.into_inner()
    )
    .execute(&data.db_pool)
    .await?;

    if query_result.rows_affected() > 0 {
        Ok(HttpResponse::Ok().body("Senha redefinida com sucesso"))
    } else {
        Ok(HttpResponse::NotFound().body("Token inválido"))
    }
}
