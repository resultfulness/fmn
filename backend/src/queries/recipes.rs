use std::collections::HashMap;

use crate::{
    models::{
        requests::{RecipeCreateRequest, RecipeUpdateRequest},
        responses::Recipe,
        schema::RecipeItem,
    },
    queries::{Queries, errors::DBError},
};
impl Queries {
    pub async fn recipe_select_many(
        &self,
    ) -> Result<HashMap<i32, Recipe>, DBError> {
        let recipes =
            sqlx::query_file_as!(Recipe, "queries/recipes/select_many.sql")
                .fetch_all(&self.pool)
                .await?;
        Ok(HashMap::from_iter(recipes.into_iter().map(|v| (v.recipe_id, v))))
    }

    pub async fn recipe_select_one(
        &self,
        recipe_id: i32,
    ) -> Result<Option<Recipe>, DBError> {
        Ok(sqlx::query_file_as!(
            Recipe,
            "queries/recipes/select_one.sql",
            recipe_id
        )
        .fetch_optional(&self.pool)
        .await?)
    }
    pub async fn recipe_insert_one(
        &self,
        recipe: RecipeCreateRequest,
    ) -> Result<i32, DBError> {
        Ok(sqlx::query_file_scalar!(
            "queries/recipes/insert_one.sql",
            recipe.name,
            recipe.icon,
            recipe.description,
            recipe.servings
        )
        .fetch_one(&self.pool)
        .await?)
    }
    pub async fn recipe_update_one(
        &self,
        recipe_id: i32,
        recipe: RecipeUpdateRequest,
    ) -> Result<(), DBError> {
        sqlx::query_file!(
            "queries/recipes/update_one.sql",
            recipe_id,
            recipe.name,
            recipe.icon,
            recipe.description,
            recipe.servings
        )
        .execute(&self.pool)
        .await?;
        if let Some(items) = recipe.items {
            sqlx::query_file!(
                "queries/recipes/delete_many_item.sql",
                recipe_id
            )
            .execute(&self.pool)
            .await?;
            let mut item_ids: Vec<i32> = vec![];
            let mut quantities: Vec<i32> = vec![];
            let mut orders: Vec<i32> = vec![];
            for (i, item) in items.iter().enumerate() {
                item_ids.push(item.item_id);
                quantities.push(item.quantity);
                orders.push(i as i32);
            }
            sqlx::query_file!(
                "queries/recipes/insert_many_item.sql",
                recipe_id,
                &item_ids,
                &quantities,
                &orders
            )
            .execute(&self.pool)
            .await?;
        }
        Ok(())
    }
    pub async fn recipe_delete_one(
        &self,
        recipe_id: i32,
    ) -> Result<(), DBError> {
        sqlx::query_file!("queries/recipes/delete_one.sql", recipe_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
    pub async fn recipe_select_one_by_name(
        &self,
        name: &str,
    ) -> Result<Option<i32>, DBError> {
        Ok(sqlx::query_file_scalar!(
            "queries/recipes/select_one_by_name.sql",
            name
        )
        .fetch_optional(&self.pool)
        .await?)
    }
}
