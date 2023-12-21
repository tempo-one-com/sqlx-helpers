use sqlx::{QueryBuilder, Postgres};

use super::ValueType;

pub struct PgBuilder<'a> {
    pub builder: QueryBuilder<'a, Postgres>
}

impl<'a> PgBuilder<'a> {
    pub fn and(&mut self, sql: &str, value: ValueType) -> &mut Self {
        match value {
            ValueType::None => {},
            _ => {
                self.builder.push(format!(" AND {sql} = "));
                self.bind(value);        
            }
        };

        self
    }

    fn bind(&mut self, value: ValueType) {
        match value {
            ValueType::String(x) => self.builder.push_bind(x),
            ValueType::Int(x) => self.builder.push_bind(x),
            ValueType::Float(x) => self.builder.push_bind(x),            
            ValueType::Bool(x) => self.builder.push_bind(x),
            ValueType::Date(x) => self.builder.push_bind(x),        
            ValueType::DateTime(x) => self.builder.push_bind(x),                    
            ValueType::None => &self.builder,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string() {
        let mut builder = PgBuilder {
            builder: QueryBuilder::new("")
        };
        
        builder.and("field", "value".to_string().into());

        assert_eq!(builder.builder.into_sql(), " AND field = $1")
    }

    #[test]
    fn int() {
        let mut builder = PgBuilder {
            builder: QueryBuilder::new("")
        };
        
        builder.and("field", 42.into());

        assert_eq!(builder.builder.into_sql(), " AND field = $1")
    }

    #[test]
    fn some() {
        let mut builder = PgBuilder {
            builder: QueryBuilder::new("")
        };
        
        builder.and("field", Some("1111".to_string()).into());

        assert_eq!(builder.builder.into_sql(), " AND field = $1")
    }    

    #[test]
    fn none() {
        let mut builder = PgBuilder {
            builder: QueryBuilder::new("")
        };
        let value:Option<String> = None;

        builder.and("field", value.into());

        assert_eq!(builder.builder.into_sql(), "")
    }    
}
