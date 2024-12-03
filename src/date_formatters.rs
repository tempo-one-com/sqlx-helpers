use sqlx::types::chrono::NaiveDate;

pub fn format_date_to_teliway(value: NaiveDate) -> String {
    value.format("%Y%m%d").to_string()
}

pub fn format_date_to_iso(value: NaiveDate) -> String {
    value.format("%Y-%m-%d").to_string()
}
