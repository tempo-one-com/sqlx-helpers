pub fn parse_empty_as_option<S>(value: S) -> Option<String>
where
    S: Into<String>,
{
    match value.into() {
        x if x.is_empty() => None,
        x => Some(x),
    }
}
