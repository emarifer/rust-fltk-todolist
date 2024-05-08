use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ListItem {
    pub completed: bool,
    pub description: String,
    pub datetime: String,
}
