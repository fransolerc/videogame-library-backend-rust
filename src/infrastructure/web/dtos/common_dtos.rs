use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SortDTO {
    pub sorted: bool,
    pub unsorted: bool,
    pub empty: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PageableDTO {
    #[serde(rename = "pageNumber")]
    pub page_number: i32,
    #[serde(rename = "pageSize")]
    pub page_size: i32,
    pub sort: SortDTO,
}
