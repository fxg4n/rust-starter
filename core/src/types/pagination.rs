use serde::{Deserialize, Serialize};
use crate::core::constants::api::{DEFAULT_PAGE_SIZE, MAX_PAGE_SIZE};

#[derive(Debug, Deserialize)]
pub struct PageRequest {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub sort: Option<String>,
    pub order: Option<String>,
}

impl Default for PageRequest {
    fn default() -> Self {
        Self {
            page: Some(1),
            page_size: Some(DEFAULT_PAGE_SIZE),
            sort: None,
            order: None,
        }
    }
}

impl PageRequest {
    pub fn get_page(&self) -> u32 {
        self.page.unwrap_or(1)
    }

    pub fn get_page_size(&self) -> u32 {
        self.page_size
            .map(|size| size.min(MAX_PAGE_SIZE))
            .unwrap_or(DEFAULT_PAGE_SIZE)
    }

    pub fn get_offset(&self) -> u64 {
        ((self.get_page() - 1) * self.get_page_size()) as u64
    }
}

#[derive(Debug, Serialize)]
pub struct PageMeta {
    pub page: u32,
    pub page_size: u32,
    pub total_items: u64,
    pub total_pages: u64,
    pub has_next_page: bool,
    pub has_previous_page: bool,
}

impl PageMeta {
    pub fn new(page: u32, page_size: u32, total_items: u64) -> Self {
        let total_pages = ((total_items + page_size as u64 - 1) / page_size as u64) as u64;
        Self {
            page,
            page_size,
            total_items,
            total_pages,
            has_next_page: (page as u64) < total_pages,
            has_previous_page: page > 1,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PageResult<T> {
    pub data: Vec<T>,
    pub meta: PageMeta,
}

impl<T> PageResult<T> {
    pub fn new(data: Vec<T>, page_request: &PageRequest, total_items: u64) -> Self {
        Self {
            data,
            meta: PageMeta::new(
                page_request.get_page(),
                page_request.get_page_size(),
                total_items,
            ),
        }
    }
}
