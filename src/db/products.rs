use crate::domain::models::Product;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateProduct {
    pub name: String,
    pub description: String,
    pub price: i64,
}

pub async fn create_product(
    db_pool: &PgPool,
    new_product: CreateProduct,
) -> Result<Product, sqlx::Error> {
    let product = sqlx::query_as!(
        Product,
        r#"
        INSERT INTO products (id, name, description, price)
        VALUES ($1, $2, $3, $4)
        RETURNING id, name, description, price, created_at, updated_at
        "#,
        Uuid::new_v4(),
        new_product.name,
        new_product.description,
        new_product.price,
    )
    .fetch_one(db_pool)
    .await?;

    Ok(product)
}

pub async fn list_products(db_pool: &PgPool) -> Result<Vec<Product>, sqlx::Error> {
    let products = sqlx::query_as!(
        Product,
        r#"
        SELECT id, name, description, price, created_at, updated_at
        FROM products
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(db_pool)
    .await?;

    Ok(products)
}

pub async fn get_product(db_pool: &PgPool, id: Uuid) -> Result<Option<Product>, sqlx::Error> {
    let product = sqlx::query_as!(
        Product,
        r#"
        SELECT id, name, description, price, created_at, updated_at
        FROM products
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(db_pool)
    .await?;

    Ok(product)
}
