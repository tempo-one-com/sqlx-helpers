pub trait PaginatedRequest {
    fn get_page(&self) -> Option<i32>;
    fn get_page_size(&self) -> Option<i32>;
}

#[derive(Clone, Debug, Default, Copy, PartialEq)]
pub struct Pagination {
    pub page: i32,
    pub nb_items: i32,
    pub limit: i32,
}

impl Pagination {
    pub fn new() -> Self {
        Self {
            page: 1,
            nb_items: 0,
            limit: i32::MAX,
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

    pub fn from_request(req: &dyn PaginatedRequest, limit: i32) -> Self {
        Self {
            page: req.get_page().unwrap_or(1),
            limit: req.get_page_size().unwrap_or(limit),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Request {
        page: Option<i32>,
        page_size: Option<i32>,
    }

    impl PaginatedRequest for Request {
        fn get_page(&self) -> Option<i32> {
            self.page
        }

        fn get_page_size(&self) -> Option<i32> {
            self.page_size
        }
    }

    #[test]
    fn get_pagination() {
        let req = Request {
            page: Some(2),
            page_size: Some(10),
        };
        let pagination = Pagination::from_request(&req, 42);

        assert_eq!(
            pagination,
            Pagination {
                page: 2,
                nb_items: 0,
                limit: 10
            }
        )
    }

    #[test]
    fn get_pagination_no_page_size() {
        let req = Request {
            page: Some(2),
            page_size: None,
        };
        let pagination = Pagination::from_request(&req, 42);

        assert_eq!(
            pagination,
            Pagination {
                page: 2,
                nb_items: 0,
                limit: 42
            }
        )
    }

    #[test]
    fn get_pagination_no_page() {
        let req = Request {
            page: None,
            page_size: Some(100),
        };
        let pagination = Pagination::from_request(&req, 42);

        assert_eq!(
            pagination,
            Pagination {
                page: 1,
                nb_items: 0,
                limit: 100
            }
        )
    }
}
