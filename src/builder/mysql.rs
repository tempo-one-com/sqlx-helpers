use sqlx::{MySql, QueryBuilder};
use std::fmt::Display;

use super::{operations::SqlOperation, types::ValueType};

impl<'a> SqlOperation for QueryBuilder<'a, MySql> {
    fn push(&mut self, sql: impl Display) -> &mut Self {
        self.push(format!(" {sql} "));

        self
    }

    fn and(&mut self, sql: &str, value: ValueType) -> &mut Self {
        match value {
            ValueType::None => {}
            _ => {
                self.push(format!(" AND {sql} ="));
                self.bind(value);
            }
        };

        self
    }

    fn and_in_str(&mut self, sql: &str, values: &[&str]) -> &mut Self {
        if values.is_empty() {
            return self;
        }

        self.push(format!(" AND {sql} IN ("));

        let mut sep = self.separated(",");
        for v in values {
            sep.push_bind(v.to_string());
        }
        sep.push_unseparated(") ");

        self
    }

    fn and_starts_like(&mut self, sql: &str, value: ValueType) -> &mut Self {
        match value {
            ValueType::None => {}
            _ => {
                self.push(format!(" AND {sql} LIKE "));
                self.bind(value);
            }
        };

        self
    }

    fn bind(&mut self, value: ValueType) {
        match value {
            ValueType::String(x) => self.push_bind(x),
            ValueType::Int(x) => self.push_bind(x),
            ValueType::Float(x) => self.push_bind(x),
            ValueType::Bool(x) => self.push_bind(x),
            ValueType::Date(x) => self.push_bind(x),
            ValueType::DateTime(x) => self.push_bind(x),
            ValueType::None => self,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn string() {
        let mut builder: QueryBuilder<'_, MySql> = QueryBuilder::new("");

        builder.and("field", "value".to_string().into());

        assert_eq!(builder.into_sql(), " AND field =?")
    }

    #[test]
    fn int() {
        let mut builder: QueryBuilder<'_, MySql> = QueryBuilder::new("");

        builder.and("field", 42.into());

        assert_eq!(builder.into_sql(), " AND field =?")
    }

    #[test]
    fn some() {
        let mut builder: QueryBuilder<'_, MySql> = QueryBuilder::new("");

        builder.and("field", Some("1111".to_string()).into());

        assert_eq!(builder.into_sql(), " AND field =?")
    }

    #[test]
    fn none() {
        let mut builder: QueryBuilder<'_, MySql> = QueryBuilder::new("");
        let value: Option<String> = None;

        builder.and("field", value.into());

        assert_eq!(builder.into_sql(), "")
    }

    #[test]
    fn push() {
        let mut builder: QueryBuilder<'_, MySql> = QueryBuilder::new("");
        builder.push("SELECT * FROM");

        assert_eq!(builder.into_sql(), "SELECT * FROM")
    }

    #[test]
    fn and_in_str_arr() {
        let mut builder: QueryBuilder<'_, MySql> = QueryBuilder::new("");
        builder.and_in_str("code", &["a", "b"]);

        assert_eq!(builder.into_sql(), " AND code IN (?,?) ")
    }
}
