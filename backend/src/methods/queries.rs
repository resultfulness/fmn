use crate::models::{
    requests::{ItemCreateRequest, ItemUpdateRequest},
    responses::ItemResponse,
};

pub trait Queries {
    fn item_insert_one(
        &mut self,
        item: ItemCreateRequest,
    ) -> impl Future<Output = Result<usize, String>>;

    fn item_update_one(
        &mut self,
        item_id: usize,
        item: ItemUpdateRequest,
    ) -> impl Future<Output = Result<(), String>>;

    fn item_select_one(
        &self,
        item_id: usize,
    ) -> impl Future<Output = Result<Option<ItemResponse>, String>>;

    fn item_select_one_by_name(
        &self,
        name: &str,
    ) -> impl Future<Output = Result<Option<usize>, String>>;

    fn item_delete_one(
        &mut self,
        item_id: usize,
    ) -> impl Future<Output = Result<(), String>>;

    fn item_select_many(
        &self,
    ) -> impl Future<Output = Result<Vec<ItemResponse>, String>>;
}
