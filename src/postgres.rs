use sqlx::{Postgres, QueryBuilder};

use super::{operations::SqlOperation, types::ValueType};

impl<'a> SqlOperation for QueryBuilder<'a, Postgres> {
    fn push_value(&mut self, sql: &str, value: ValueType) {
        match value {
            ValueType::None => {}
            _ => {
                self.push(format!("{sql} ="));
                self.bind(value);
            }
        };
    }

    fn starts_like(&mut self, sql: &str, value: ValueType) {
        match value {
            ValueType::None => {}
            _ => {
                self.push(format!("{sql} ILIKE "));
                self.bind(value);
            }
        };
    }

    fn in_str(&mut self, sql: &str, values: &[&str]) {
        let types = values
            .into_iter()
            .map(|x| ValueType::String((*x).to_owned()))
            .collect::<Vec<_>>();

        self.in_value_types(sql, &types);
    }

    fn in_int(&mut self, sql: &str, values: &[i32]) {
        let types = values
            .into_iter()
            .map(|x| ValueType::Int(*x))
            .collect::<Vec<_>>();

        self.in_value_types(sql, &types);
    }

    fn in_value_types(&mut self, sql: &str, values: &[ValueType]) {
        if values.is_empty() {
            return;
        }

        self.push(format!("{sql} IN ("));

        let mut sep = self.separated(",");
        for v in values.to_vec() {
            match v {
                ValueType::String(x) => sep.push_bind(x),
                ValueType::Int(x) => sep.push_bind(x),
                ValueType::Float(x) => sep.push_bind(x),
                ValueType::Bool(x) => sep.push_bind(x),
                ValueType::Date(x) => sep.push_bind(x),
                ValueType::DateTime(x) => sep.push_bind(x),
                ValueType::None => &mut sep,
            };
        }
        sep.push_unseparated(")");
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
        let mut builder: QueryBuilder<'_, Postgres> = QueryBuilder::new("");

        builder.push_value("AND field", "value".to_string().into());

        assert_eq!(builder.into_sql(), "AND field =$1")
    }

    #[test]
    fn int() {
        let mut builder: QueryBuilder<'_, Postgres> = QueryBuilder::new("");
        builder.push_value("AND field", 42.into());

        assert_eq!(builder.into_sql(), "AND field =$1")
    }

    #[test]
    fn some() {
        let mut builder: QueryBuilder<'_, Postgres> = QueryBuilder::new("");
        builder.push_value("AND field", Some("1111".to_string()).into());

        assert_eq!(builder.into_sql(), "AND field =$1")
    }

    #[test]
    fn none() {
        let mut builder: QueryBuilder<'_, Postgres> = QueryBuilder::new("");
        let value: Option<String> = None;

        builder.push_value("AND field", value.into());

        assert_eq!(builder.into_sql(), "")
    }

    #[test]
    fn push() {
        let mut builder: QueryBuilder<'_, Postgres> = QueryBuilder::new("");
        builder.push("SELECT * FROM");

        assert_eq!(builder.into_sql(), "SELECT * FROM")
    }

    #[test]
    fn starts_like() {
        let mut builder: QueryBuilder<'_, Postgres> = QueryBuilder::new("");
        let name_like = "hank";
        let name_like = format!("{name_like}%");

        builder.starts_like("AND field", name_like.into());

        assert_eq!(builder.into_sql(), "AND field ILIKE $1")
    }

    #[test]
    fn and_in_str_arr() {
        let mut builder: QueryBuilder<'_, Postgres> = QueryBuilder::new("");
        builder.in_str("AND code", &["a", "b"]);

        assert_eq!(builder.sql(), "AND code IN ($1,$2)")
    }
}
