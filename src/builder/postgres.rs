use std::fmt::Display;
use sqlx::{Postgres, QueryBuilder};

use super::{operations::SqlOperation, types::ValueType};

pub struct Builder<'a> {
    pub internal: QueryBuilder<'a, Postgres>,
}

impl<'a> Builder<'a> {
    pub fn new(init: impl Into<String>) -> Self {
        Builder {
            internal: QueryBuilder::new(init),
        }
    }
}

impl<'a> SqlOperation for Builder<'a> {
    fn push(&mut self, sql: impl Display) -> &mut Self {
        self.internal.push(format!(" {sql} "));

        self
    }

    fn and(&mut self, sql: &str, value: ValueType) -> &mut Self {
        match value {
            ValueType::None => {}
            _ => {
                self.internal.push(format!(" AND {sql} = "));
                self.bind(value);
            }
        };

        self
    }

    fn and_starts_like(&mut self, sql: &str, value: ValueType) -> &mut Self {
        match value {
            ValueType::None => {}
            _ => {
                self.internal.push(format!(" AND {sql} LIKE "));
                self.bind(value);
            }
        };

        self
    }

    fn and_in_str_arr(&mut self, sql: &str, values: &[&str]) -> &mut Self {        
        if values.is_empty() {
            return self
        }

        self.internal.push(format!(" AND {sql} IN ("));

        let mut sep = self.internal.separated(",");
        for v in values {
            sep.push_bind(v.to_string());
        }
        sep.push_unseparated(") ");

        self      
    }

    fn and_in_str(&mut self, sql: &str, value: &str) -> &mut Self {        
        if value.is_empty() {
            return self
        }

        let values = 
            value.split(',')
            .map(|x| x.trim())
            .collect::<Vec<_>>();

        self.and_in_str_arr(sql, &values)
    }

    fn bind(&mut self, value: ValueType) {
        match value {
            ValueType::String(x) => self.internal.push_bind(x),
            ValueType::Int(x) => self.internal.push_bind(x),
            ValueType::Float(x) => self.internal.push_bind(x),
            ValueType::Bool(x) => self.internal.push_bind(x),
            ValueType::Date(x) => self.internal.push_bind(x),
            ValueType::DateTime(x) => self.internal.push_bind(x),
            ValueType::None => &self.internal,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string() {
        let mut builder = Builder {
            internal: QueryBuilder::new(""),
        };

        builder.and("field", "value".to_string().into());

        assert_eq!(builder.internal.into_sql(), " AND field = $1")
    }

    #[test]
    fn int() {
        let mut builder = Builder::new("");

        builder.and("field", 42.into());

        assert_eq!(builder.internal.into_sql(), " AND field = $1")
    }

    #[test]
    fn some() {
        let mut builder = Builder::new("");

        builder.and("field", Some("1111".to_string()).into());

        assert_eq!(builder.internal.into_sql(), " AND field = $1")
    }

    #[test]
    fn none() {
        let mut builder = Builder::new("");

        let value: Option<String> = None;

        builder.and("field", value.into());

        assert_eq!(builder.internal.into_sql(), "")
    }

    #[test]
    fn push() {
        let mut builder = Builder::new("");
        builder.push("SELECT * FROM");

        assert_eq!(builder.internal.into_sql(), " SELECT * FROM ")
    }

    #[test]
    fn starts_like() {
        let mut builder = Builder::new("");
        let name_like = "hank";
        let name_like = format!("{name_like}%");

        builder.and_starts_like("field", name_like.into());

        assert_eq!(builder.internal.into_sql(), " AND field LIKE $1")
    }

    #[test]
    fn and_in_str_arr() {
        let mut builder = Builder::new("");
        builder.and_in_str_arr("code", &["a","b"]);

        assert_eq!(builder.internal.sql(), " AND code IN ($1,$2) ")
    }

    #[test]
    fn and_in_str() {
        let mut builder = Builder::new("");
        builder.and_in_str("code", "a,b");

        assert_eq!(builder.internal.sql(), " AND code IN ($1,$2) ")
    }         
}
