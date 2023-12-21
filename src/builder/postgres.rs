use std::fmt::Display;

use sqlx::{QueryBuilder, Postgres};

use super::ValueType;

pub struct Builder<'a> {
    pub internal: QueryBuilder<'a, Postgres>
}

impl<'a> Builder<'a> {
    pub fn new(init: impl Into<String>) -> Self {
        Builder {
            internal: QueryBuilder::new(init)
        }
    }

    pub fn push(&mut self, sql: impl Display) -> &mut Self {
        self.internal.push(format!(" {sql} "));
        
        self
    }

    pub fn and(&mut self, sql: &str, value: ValueType) -> &mut Self {
        match value {
            ValueType::None => {},
            _ => {
                self.internal.push(format!(" AND {sql} = "));
                self.bind(value);        
            }
        };

        self
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
            internal: QueryBuilder::new("")
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

        let value:Option<String> = None;

        builder.and("field", value.into());

        assert_eq!(builder.internal.into_sql(), "")
    }    

    #[test]
    fn push() {
        let mut builder = Builder::new("");
        builder.push("SELECT * FROM");

        assert_eq!(builder.internal.into_sql(), " SELECT * FROM ")
    }
}
