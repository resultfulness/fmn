use crate::{
    models::{
        requests::{ItemCreateRequest, ItemUpdateRequest},
        responses::Item,
    },
    queries::{Queries, errors::DBError},
};
impl Queries {
    pub async fn item_select_many(&self) -> Result<Vec<Item>, DBError> {
        Ok(sqlx::query_file_as!(Item, "queries/items/select_many.sql")
            .fetch_all(&self.pool)
            .await?)
    }

    pub async fn item_select_one(
        &self,
        item_id: i32,
    ) -> Result<Option<Item>, DBError> {
        Ok(
            sqlx::query_file_as!(Item, "queries/items/select_one.sql", item_id)
                .fetch_optional(&self.pool)
                .await?,
        )
    }
    pub async fn item_insert_one(
        &self,
        item: ItemCreateRequest,
    ) -> Result<i32, DBError> {
        Ok(sqlx::query_file_scalar!(
            "queries/items/insert_one.sql",
            item.name,
            item.icon,
            item.unit
        )
        .fetch_one(&self.pool)
        .await?)
    }
    pub async fn item_update_one(
        &self,
        item_id: i32,
        item: ItemUpdateRequest,
    ) -> Result<(), DBError> {
        sqlx::query_file!(
            "queries/items/update_one.sql",
            item_id,
            item.name,
            item.icon,
            item.unit
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
    pub async fn item_delete_one(&self, item_id: i32) -> Result<(), DBError> {
        sqlx::query_file!("queries/items/delete_one.sql", item_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
    pub async fn item_select_one_by_name(
        &self,
        name: &str,
    ) -> Result<Option<i32>, DBError> {
        Ok(sqlx::query_file_scalar!(
            "queries/items/select_one_by_name.sql",
            name
        )
        .fetch_optional(&self.pool)
        .await?)
    }
}
