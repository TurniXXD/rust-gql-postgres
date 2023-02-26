use juniper::{Executor, FieldResult, GraphQLObject, GraphQLInputObject, ToInputValue, Value, InputValue, FromInputValue};
use juniper_from_schema::graphql_schema_from_file;
use std::convert::From;

#[derive(Debug, Clone, Copy, PartialEq, Eq, /*DieselNumericOps,*/ SqlType)]
pub enum Locale {
    CZ,
    SK,
    EN,
}

// impl FromInputValue for Locale {
//   fn from_input_value(value: &InputValue) -> Option<Self> {
//       match value.as_scalar_value::<String>() {
//           Some(s) => match s.as_str() {
//               "cz" => Some(Locale::CZ),
//               "sk" => Some(Locale::SK),
//               "em" => Some(Locale::EN),
//               _ => None,
//           },
//           None => None,
//       }
//   }
// }

impl FromInputValue for Locale {
  fn from_input_value(v: &InputValue) -> Option<Self> {
      if let Some(s) = v.as_scalar_value::<String>() {
          match s.to_lowercase().as_str() {
              "cz" => Some(Locale::CZ),
              "sk" => Some(Locale::SK),
              "en" => Some(Locale::EN),
              _ => None,
          }
      } else {
          None
      }
  }
}

impl ToInputValue for Locale {
  fn to_input_value(&self) -> InputValue {
      match self {
          Locale::CZ => InputValue::scalar("cz"),
          Locale::SK => InputValue::scalar("sk"),
          Locale::EN => InputValue::scalar("en"),
      }
  }
}


// use diesel::sql_types::Text;
// use diesel::serialize::{ToSql, Output};
// use diesel::deserialize::{FromSql, Result};
// use diesel::expression::AsExpression;
// use crate::schema::translations::SqlType;

// impl<T> AsExpression<T> for Locale where
//     T: SqlType,
//     Self: ToSql<T>,
// {
//     type Expression = Text;

//     fn as_expression(self) -> Self::Expression {
//         Self::Expression::from(self.value)
//     }
// }

// impl<T> FromSql<T> for Locale
// where
//     T: SqlType,
//     Self: FromSql<T>,
// {
//     fn from_sql(value: Option<&[u8]>) -> Result<Self> {
//         let value = String::from_sql(value)?;
//         match value.as_str() {
//             "CZ" => Ok(Locale::CZ),
//             "SK" => Ok(Locale::SK),
//             "EN" => Ok(Locale::EN),
//             _ => Err(Box::new(Error::Parse(format!(
//                 "Unknown Locale value: {}",
//                 value
//             )))),
//         }
//     }
// }

//#[derive(GraphQLInputObject)]
pub struct GetAllStaticTranslationsInput {
    pub locale: Locale,
}

///Translation res type
#[derive(GraphQLObject)]
pub struct Translation {
    uid_path: String,
    translation: String,
}


