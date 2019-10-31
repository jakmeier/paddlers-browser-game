use juniper::*;
use juniper::IntoFieldError;

#[derive(Debug)]
pub enum ReadableInterfaceError {
    NotAllowed,
}

impl IntoFieldError for ReadableInterfaceError {
    fn into_field_error(self) -> FieldError {
        match self {
            ReadableInterfaceError::NotAllowed => FieldError::new(
                "Player not allowed to read this field.",
                graphql_value!({ "internal_error": "Not allowed" }),
            ),
        }
    }
}

impl std::fmt::Display for ReadableInterfaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}