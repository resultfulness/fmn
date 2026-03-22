use std::fmt::Display;

#[derive(Debug)]
pub struct DBError {
    pub detail: String,
}

impl From<sqlx::Error> for DBError {
    fn from(value: sqlx::Error) -> Self {
        Self {
            detail: value.to_string(),
        }
    }
}

impl Display for DBError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DBError({})", self.detail)
    }
}
