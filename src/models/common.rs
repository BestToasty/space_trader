use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    pub total: i32,
    pub page: i32,
    pub limit: i32,
}
