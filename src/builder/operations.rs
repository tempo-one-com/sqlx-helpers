use std::fmt::Display;

use super::types::ValueType;

pub trait SqlOperation {
   fn push(&mut self, sql: impl Display) -> &mut Self;
   fn and(&mut self, sql: &str, value: ValueType) -> &mut Self;
   fn and_in_str(&mut self, sql: &str, values: &[&str]) -> &mut Self;
   fn bind(&mut self, value: ValueType);
   fn and_starts_like(&mut self, sql: &str, value: ValueType) -> &mut Self;
}