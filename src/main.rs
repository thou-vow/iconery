pub mod customer;
pub mod error;
pub mod order;
pub mod product;
pub mod util;

use std::sync::Arc;

use actix_web::{App, HttpServer, web::Data};
use serde::Deserialize;
use sqlx::{MySqlPool, mysql::MySqlPoolOptions};

pub type Result<T> = std::result::Result<T, crate::error::Error>;

#[derive(Clone, Deserialize)]
pub struct Config {
    pub server_host: String,
    pub server_port: u16,
    pub db_client: String,
    pub db_user: String,
    pub db_password: String,
    pub db_host: String,
    pub db_port: u16,
    pub db_name: String,
    pub smtp_user: String,
    pub smtp_password: String,
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_from: String,
}

#[derive(Clone)]
pub struct AppData {
    pub config: Arc<Config>,
    pub db_pool: MySqlPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let serialized_config =
        std::fs::read_to_string("./config.json").expect("Could not find config.json");

    let config = Arc::new(
        serde_json::from_str::<Config>(&serialized_config)
            .expect("Failed to deserialize config.json"),
    );

    let db_pool = MySqlPoolOptions::new()
        .connect(&format!(
            "{}://{}:{}@{}:{}/{}",
            config.db_client,
            config.db_user,
            config.db_password,
            config.db_host,
            config.db_port,
            config.db_name
        ))
        .await
        .expect("Failed to connect to the DB");

    let bind_host = config.server_host.clone();
    let bind_port = config.server_port;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppData {
                config: config.clone(),
                db_pool: db_pool.clone(),
            }))
            .service(product::create_product)
            .service(product::update_product)
            .service(product::delete_product)
            .service(product::get_product)
            .service(product::get_products)
            .service(product::get_featured_products)
            .service(product::get_products_with_search)
            .service(customer::create_customer)
            .service(customer::update_customer)
            .service(customer::delete_customer)
            .service(customer::get_customer)
            .service(customer::get_customers)
            .service(customer::login_customer)
            .service(customer::activate_customer)
            .service(customer::send_password_reset)
            .service(customer::password_reset)
            .service(order::create_order)
            .service(order::update_order)
            .service(order::delete_order)
            .service(order::get_order)
            .service(order::get_orders)
            .service(order::get_orders_by_customer)
    })
    .bind((bind_host, bind_port))?
    .run()
    .await
}
