//! Handling GraphQL response parsing.
//!
//! The hard work of parsing is done externally in the graphql_client crate.
//! This module takes the parsed values, checks for errors and passes on the data to the next layer.
//!

use paddlers_shared_lib::prelude::PadlApiError;

use crate::prelude::{PadlError, PadlErrorCode, PadlResult};

/// Reads a response, converts the errors to PaddleErrors and extracts the response-specific data.
///
///  We expect a GQL answer body which look something like this:
/// ```
/// {
///  data: null,
///  errors: [
///      {
///          extensions: { padlcode: 255 }
///          message: "error description",
///          locations: [...],
///          path: [...],
///      },
///  ]
/// }
/// ```
///
///
/// graphql_client::Response<DATA> parses the data and error fields, where DATA is defined differently for each query.
/// In this module, the data is just returned as received. Only the errors are handled further in here.
///
/// The interesting bit is the `padlcode` in the error extension, as well as the error message.
///
///
pub fn gql_extract_data<DATA>(response: graphql_client::Response<DATA>) -> PadlResult<DATA> {
    if let Some(data) = response.data {
        Ok(data)
    } else {
        // GraphQL standard defines that either errors or data is present.
        let errors = response
            .errors
            .ok_or(PadlError::dev_err(PadlErrorCode::GraphQlNoDataOrErrors))?;

        // Pragmatic solution: For simplicity, only the first error in the array is considered.
        let error = errors
            .first()
            .ok_or(PadlError::dev_err(PadlErrorCode::GraphQlNoDataOrErrors))?;

        // Custom error extension for Paddlers allow to convert error messages to typed errors.
        // Extracting necessary fields.
        let extensions = error.extensions.as_ref().ok_or_else(|| {
            PadlError::dev_err(PadlErrorCode::GraphQlGenericResponseError(format!(
                "Response is missing extensions. Error obj: {:?}",
                error
            )))
        })?;
        let code_value = extensions.get("padlcode").ok_or_else(|| {
            PadlError::dev_err(PadlErrorCode::GraphQlGenericResponseError(format!(
                "Response error extension has no padlecode. Error obj: {:?}",
                error
            )))
        })?;
        let numeric_code = code_value.as_f64().ok_or_else(|| {
            PadlError::dev_err(PadlErrorCode::GraphQlGenericResponseError(format!(
                "Padldecode is not numeric. Error obj: {:?}",
                error
            )))
        })?;
        let code = PadlApiError::try_from_num(numeric_code as u8).ok_or_else(|| {
            PadlError::dev_err(PadlErrorCode::GraphQlGenericResponseError(format!(
                "Unknown API error code. Error obj: {:?}",
                error
            )))
        })?;
        PadlErrorCode::GraphQlResponseError(code).dev()
    }
}
