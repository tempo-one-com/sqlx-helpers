use crate::pagination::Pagination;

use super::types::ValueType;

pub trait SqlOperation {
    fn push_value(&mut self, sql: &str, value: ValueType);
    fn in_str<S>(&mut self, sql: &str, values: &[S])
    where
        S: Into<String> + Clone;
    fn in_int(&mut self, sql: &str, values: &[i32]);
    fn in_value_types(&mut self, sql: &str, values: &[ValueType]);
    fn bind(&mut self, value: ValueType);
    fn like_starts_with(&mut self, sql: &str, value: ValueType);
    fn like_within(&mut self, sql: &str, value: ValueType);
    fn set_pagination(&mut self, sql: &str, pagination: Pagination);
}
