use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
}
