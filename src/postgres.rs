use sqlx::{Postgres, QueryBuilder};

use crate::pagination::Pagination;

use super::{operations::SqlOperation, types::ValueType};

impl<'a> SqlOperation for QueryBuilder<'a, Postgres> {
    fn push_value(&mut self, sql: &str, value: ValueType) {
        match value {
            ValueType::None => {}
            _ => {
                self.push(format!("{sql}"));
                self.bind(value);
            }
        };
    }

    fn like_starts_with(&mut self, sql: &str, value: ValueType) {
        match value {
            ValueType::None => {}
            _ => {
                self.push(format!("{sql} ILIKE CONCAT("));
                self.bind(value);
                self.push(",'%')");
            }
        };
    }

    fn like_within(&mut self, sql: &str, value: ValueType) {
        match value {
            ValueType::None => {}
            _ => {
                self.push(format!("{sql} ILIKE CONCAT('%',"));
                self.bind(value);
                self.push(",'%')");
            }
        };
    }

    fn in_str<S>(&mut self, sql: &str, values: &[S])
    where
        S: Into<String> + Clone,
    {
        let types: Vec<ValueType> = values
            .iter()
            .map(|x| ValueType::String((*x).clone().into()))
            .collect::<Vec<_>>();

        self.in_value_types(sql, &types);
    }

    fn in_int(&mut self, sql: &str, values: &[i32]) {
        let types: Vec<ValueType> = values.into_iter().map(|x| (*x).into()).collect::<Vec<_>>();

        self.in_value_types(sql, &types);
    }

    fn in_value_types(&mut self, sql: &str, values: &[ValueType]) {
        if values.is_empty() {
            return;
        }

        self.push(format!("{sql} ("));

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

    ///sql est du style: " ORDER BY s.sDateCreation DESC LIMIT "
    fn set_pagination(&mut self, sql: &str, pagination: Pagination) {
        self.push(sql);
        self.push_value(" LIMIT ", (pagination.limit as i32).into());
        self.push(" OFFSET ");
        self.push_bind(pagination.get_offset_for_page(pagination.page));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string() {
        let mut builder: QueryBuilder<'_, Postgres> = QueryBuilder::new("");

        builder.push_value("AND field=", "value".to_string().into());

        assert_eq!(builder.into_sql(), "AND field=$1")
    }

    #[test]
    fn int() {
        let mut builder: QueryBuilder<'_, Postgres> = QueryBuilder::new("");
        builder.push_value("AND field=", 42.into());

        assert_eq!(builder.into_sql(), "AND field=$1")
    }

    #[test]
    fn some() {
        let mut builder: QueryBuilder<'_, Postgres> = QueryBuilder::new("");
        builder.push_value("AND field=", Some("1111".to_string()).into());

        assert_eq!(builder.into_sql(), "AND field=$1")
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
        //let name_like = format!("{name_like}");

        builder.like_starts_with("AND field", name_like.to_string().into());

        assert_eq!(builder.into_sql(), "AND field ILIKE CONCAT($1,'%')")
    }

    #[test]
    fn in_str_arr() {
        let mut builder: QueryBuilder<'_, Postgres> = QueryBuilder::new("");
        builder.in_str("AND code IN", &["a", "b"]);

        assert_eq!(builder.sql(), "AND code IN ($1,$2)")
    }

    #[test]
    fn not_in_str_arr() {
        let mut builder: QueryBuilder<'_, Postgres> = QueryBuilder::new("");
        builder.in_str("AND code NOT IN", &["a", "b"]);

        assert_eq!(builder.sql(), "AND code NOT IN ($1,$2)")
    }

    #[test]
    fn in_str_arr_string() {
        let mut builder: QueryBuilder<'_, Postgres> = QueryBuilder::new("");
        builder.in_str("AND code IN", &["a".to_string(), "b".to_string()]);

        assert_eq!(builder.sql(), "AND code IN ($1,$2)")
    }

    #[test]
    fn pagination() {
        let mut builder: QueryBuilder<'_, Postgres> = QueryBuilder::new("");
        let pagination = Pagination {
            page: 5,
            limit: 10,
            nb_items: 100,
        };

        builder.set_pagination(" ORDER BY s.position", pagination);

        assert_eq!(builder.sql(), " ORDER BY s.position LIMIT $1 OFFSET $2")
    }
}
