use sqlx::types::time::{Date, PrimitiveDateTime};
pub enum ValueType {
   String(String),
   Int(i32),
   Float(f32),
   Date(Date),
   DateTime(PrimitiveDateTime),
   Bool(bool),
   None,
}

impl From<Option<String>> for ValueType {
   fn from(value: Option<String>) -> Self {
       match value {
           Some(x) => Self::String(x),
           _ => Self::None,
       }
   }
}

impl From<String> for ValueType {
   fn from(value: String) -> Self {
       Self::String(value)
   }
}

impl From<Option<i32>> for ValueType {
   fn from(value: Option<i32>) -> Self {
       match value {
           Some(x) => Self::Int(x),
           _ => Self::None,
       }
   }
}

impl From<i32> for ValueType {
   fn from(value: i32) -> Self {
       Self::Int(value)
   }
}

impl From<Option<bool>> for ValueType {
   fn from(value: Option<bool>) -> Self {
       match value {
           Some(x) => Self::Bool(x),
           _ => Self::None,
       }
   }
}

impl From<bool> for ValueType {
   fn from(value: bool) -> Self {
       Self::Bool(value)
   }
}

impl From<Option<Date>> for ValueType {
   fn from(value: Option<Date>) -> Self {
       match value {
           Some(x) => Self::Date(x),
           _ => Self::None,
       }
   }
}

impl From<Date> for ValueType {
   fn from(value: Date) -> Self {
       Self::Date(value)
   }
}

impl From<Option<PrimitiveDateTime>> for ValueType {
   fn from(value: Option<PrimitiveDateTime>) -> Self {
       match value {
           Some(x) => Self::DateTime(x),
           _ => Self::None,
       }
   }
}

impl From<PrimitiveDateTime> for ValueType {
   fn from(value: PrimitiveDateTime) -> Self {
       Self::DateTime(value)
   }
}