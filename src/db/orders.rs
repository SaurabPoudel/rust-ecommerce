use crate::domain::models::{Order, OrderItem};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateOrderItem {
    pub product_id: Uuid,
    pub quantity: i32,
    pub price: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrder {
    pub items: Vec<CreateOrderItem>,
    pub total_amount: i64,
}

pub async fn create_order(
    db_pool: &PgPool,
    user_id: Uuid,
    new_order: CreateOrder,
) -> Result<Order, Box<dyn std::error::Error>> {
    let mut tx = db_pool.begin().await?;

    let order_id = Uuid::new_v4();
    let order = sqlx::query_as!(
        Order,
        r#"
        INSERT INTO orders (id, user_id, total_amount, status)
        VALUES ($1, $2, $3, $4)
        RETURNING id, user_id, total_amount, status, created_at, updated_at
        "#,
        order_id,
        user_id,
        new_order.total_amount,
        "pending"
    )
    .fetch_one(&mut *tx)
    .await?;

    for item in new_order.items {
        sqlx::query!(
            r#"
            INSERT INTO order_items (id, order_id, product_id, quantity, price)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            Uuid::new_v4(),
            order_id,
            item.product_id,
            item.quantity,
            item.price
        )
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    Ok(order)
}

pub async fn list_orders_by_user(
    db_pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<Order>, sqlx::Error> {
    let orders = sqlx::query_as!(
        Order,
        r#"
        SELECT id, user_id, total_amount, status, created_at, updated_at
        FROM orders
        WHERE user_id = $1
        ORDER BY created_at DESC
        "#,
        user_id
    )
    .fetch_all(db_pool)
    .await?;

    Ok(orders)
}

pub async fn get_order(
    db_pool: &PgPool,
    order_id: Uuid,
) -> Result<Option<(Order, Vec<OrderItem>)>, sqlx::Error> {
    let order = sqlx::query_as!(
        Order,
        r#"
        SELECT id, user_id, total_amount, status, created_at, updated_at
        FROM orders
        WHERE id = $1
        "#,
        order_id
    )
    .fetch_optional(db_pool)
    .await?;

    if let Some(order) = order {
        let items = sqlx::query_as!(
            OrderItem,
            r#"
            SELECT id, order_id, product_id, quantity, price, created_at, updated_at
            FROM order_items
            WHERE order_id = $1
            "#,
            order_id
        )
        .fetch_all(db_pool)
        .await?;

        Ok(Some((order, items)))
    } else {
        Ok(None)
    }
}
