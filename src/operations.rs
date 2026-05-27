use crate::pagination::Pagination;

use super::types::ValueType;

pub trait SqlOperation {
    fn push_value(&mut self, sql: &'static str, value: ValueType);
    fn in_str<S>(&mut self, sql: &'static str, values: &[S])
    where
        S: Into<String> + Clone;
    fn in_int(&mut self, sql: &'static str, values: &[i32]);
    fn in_value_types(&mut self, sql: &'static str, values: &[ValueType]);
    fn bind(&mut self, value: ValueType);
    fn like_starts_with(&mut self, sql: &'static str, value: ValueType);
    fn like_within(&mut self, sql: &'static str, value: ValueType);
    fn set_pagination(&mut self, sql: &'static str, pagination: Pagination);
}
