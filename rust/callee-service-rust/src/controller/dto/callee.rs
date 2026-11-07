use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct Callee {
    pub id: Option<String>,
    pub message: String,
}
