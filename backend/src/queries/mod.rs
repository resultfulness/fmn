use sqlx::{Pool, Postgres};

pub mod errors;
mod items;
mod recipes;
mod cart;

pub struct Queries {
    pool: Pool<Postgres>,
}

impl Queries {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}
