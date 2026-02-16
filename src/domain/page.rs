use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page<T> {
    pub content: Vec<T>,
    pub page: i32,
    pub size: i32,
    pub total_elements: i64,
    pub total_pages: i32,
}

impl<T> Page<T> {
    pub fn new(content: Vec<T>, page: i32, size: i32, total_elements: i64) -> Self {
        let total_pages = if size > 0 {
            (total_elements as f64 / size as f64).ceil() as i32
        } else {
            0
        };

        Self {
            content,
            page,
            size,
            total_elements,
            total_pages,
        }
    }
}
