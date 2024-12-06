pub trait PaginatedRequest {
    fn get_page(&self) -> Option<i32>;
    fn get_page_size(&self) -> Option<i32>;
}

#[derive(Clone, Debug, Copy)]
pub struct Pagination {
    pub page: Option<i32>,
    pub nb_items: i32,
    pub limit: i32,
}

impl Pagination {
    pub fn new(page: Option<i32>, nb_items: i32, limit: i32) -> Self {
        Self {
            page,
            nb_items,
            limit,
        }
    }

    pub fn get_offset_for_page(&self, page_index: i32) -> i32 {
        if self.limit == i32::MAX {
            0
        } else {
            (page_index - 1) * self.limit
        }
    }

    pub fn page_count(&self) -> i32 {
        if self.limit == i32::MAX {
            1
        } else {
            (self.nb_items + self.limit - 1) / self.limit
        }
    }
}
