use super::types::ValueType;

pub trait SqlOperation {
    fn push_value(&mut self, sql: &str, value: ValueType);
    fn in_str(&mut self, sql: &str, values: &[&str]);
    fn in_int(&mut self, sql: &str, values: &[i32]);
    fn in_value_types(&mut self, sql: &str, values: &[ValueType]);
    fn bind(&mut self, value: ValueType);
    fn starts_like(&mut self, sql: &str, value: ValueType);
}
