use sqlx::{MySql, QueryBuilder};

use super::{operations::SqlOperation, types::ValueType};

impl<'a> SqlOperation for QueryBuilder<'a, MySql> {
    fn push_value(&mut self, sql: &str, value: ValueType) {
        match value {
            ValueType::None => {}
            _ => {
                self.push(format!("{sql} ="));
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

    fn starts_like(&mut self, sql: &str, value: ValueType) {
        match value {
            ValueType::None => {}
            _ => {
                self.push(format!("{sql} LIKE "));
                self.bind(value);
            }
        };
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

        builder.push_value("AND field", "value".to_string().into());

        assert_eq!(builder.into_sql(), "AND field =?")
    }

    #[test]
    fn int() {
        let mut builder: QueryBuilder<'_, MySql> = QueryBuilder::new("");

        builder.push_value("AND field", 42.into());

        assert_eq!(builder.into_sql(), "AND field =?")
    }

    #[test]
    fn some() {
        let mut builder: QueryBuilder<'_, MySql> = QueryBuilder::new("");

        builder.push_value("AND field", Some("1111".to_string()).into());

        assert_eq!(builder.into_sql(), "AND field =?")
    }

    #[test]
    fn none() {
        let mut builder: QueryBuilder<'_, MySql> = QueryBuilder::new("");
        let value: Option<String> = None;

        builder.push_value("field", value.into());

        assert_eq!(builder.into_sql(), "")
    }

    #[test]
    fn push() {
        let mut builder: QueryBuilder<'_, MySql> = QueryBuilder::new("");
        builder.push("SELECT * FROM");

        assert_eq!(builder.into_sql(), "SELECT * FROM")
    }

    #[test]
    fn in_str_arr() {
        let mut builder: QueryBuilder<'_, MySql> = QueryBuilder::new("");
        builder.in_str("AND code", &["a", "b"]);

        assert_eq!(builder.into_sql(), "AND code IN (?,?)")
    }
}
