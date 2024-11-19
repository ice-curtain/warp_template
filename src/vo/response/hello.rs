use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct HelloResult {
    pub id:String,
    pub tag_define_id:String,
    pub code: String,
    pub name: String,
}
