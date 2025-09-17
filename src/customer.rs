use actix_web::{
    HttpResponse,
    web::{Data, Json},
};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::{AppData, Result};

#[derive(Deserialize, FromRow, Serialize)]
pub struct Customer {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password: String,
    pub phone_number: Option<String>,
    pub address: Option<String>,
    pub is_active: bool,
}

#[actix_web::post("/api/customers")]
pub async fn create_customer(data: Data<AppData>, body: Json<Customer>) -> Result<HttpResponse> {
    let hashed_password = crate::util::hash(&body.password);

    sqlx::query!(
        r#"INSERT INTO customers (id, name, email, password, phone_number, address, is_active)
           VALUES (?, ?, ?, ?, ?, ?, ?)"#,
        body.id,
        body.name,
        body.email,
        hashed_password,
        body.phone_number,
        body.address,
        body.is_active,
    )
    .execute(&data.db_pool)
    .await?;

    let confirm_link = format!(
        "http://{}:{}/api/customers/activate/{}",
        data.config.smtp_host,
        data.config.smtp_port,
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
