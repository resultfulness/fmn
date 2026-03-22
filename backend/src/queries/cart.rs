use uuid::Uuid;

use crate::{
    models::{
        responses::CartItem,
        schema::{Event, EventPayload},
    },
    queries::{Queries, errors::DBError},
};

impl Queries {
    pub async fn cart_delete_many_item(&self) -> Result<(), DBError> {
        sqlx::query_file_as!(Item, "queries/cart/delete_many_item.sql")
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn cart_insert_one_event(
        &self,
        event: EventPayload,
    ) -> Result<(), DBError> {
        sqlx::query_file!(
            "queries/cart/insert_one_event.sql",
            serde_json::to_value(event).unwrap()
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
    pub async fn cart_insert_many_item(
        &self,
        items: Vec<CartItem>,
    ) -> Result<(), DBError> {
        let mut item_ids: Vec<i32> = vec![];
        let mut descriptions: Vec<Option<String>> = vec![];
        let mut quantities: Vec<Option<i32>> = vec![];
        let mut orders: Vec<i32> = vec![];
        for (i, item) in items.into_iter().enumerate() {
            item_ids.push(item.item_id);
            quantities.push(item.quantity);
            descriptions.push(item.description);
            orders.push(i as i32);
        }
        sqlx::query_file!(
            "queries/cart/insert_many_item.sql",
            &item_ids,
            &descriptions as &[Option<String>],
            &quantities as &[Option<i32>],
            &orders
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
    pub async fn cart_select_many_event(&self) -> Result<Vec<Event>, DBError> {
        Ok(sqlx::query_file_as!(Event, "queries/cart/select_many_event.sql")
            .fetch_all(&self.pool)
            .await?)
    }
    pub async fn cart_select_many_item(
        &self,
    ) -> Result<Vec<CartItem>, DBError> {
        Ok(sqlx::query_file_as!(CartItem, "queries/cart/select_many_item.sql")
            .fetch_all(&self.pool)
            .await?)
    }
    pub async fn cart_update_one_event(
        &self,
        event_id: Uuid,
        is_future: bool,
    ) -> Result<(), DBError> {
        sqlx::query_file!(
            "queries/cart/update_one_event.sql",
            event_id,
            is_future
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
    pub async fn cart_delete_future_event(&self) -> Result<(), DBError> {
        sqlx::query_file!("queries/cart/delete_future_event.sql")
            .execute(&self.pool)
            .await?;
        Ok(())
    }
    pub async fn cart_delete_many_event(&self) -> Result<(), DBError> {
        sqlx::query_file!("queries/cart/delete_many_event.sql")
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
