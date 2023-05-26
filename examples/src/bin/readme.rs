// This is an example of using BLAST! util with the Rocket framework.

extern crate blast;
extern crate thiserror;
#[macro_use]
extern crate rocket;

use blast::macros::{Codes, MakeResponder}; // Handle our errors with Rocket gracefully.
use rocket::serde::json::Json;
use thiserror::Error as ThisError;

#[derive(Codes, Clone, Debug, Eq, PartialEq, MakeResponder, ThisError)]
enum AppErr {
    #[error("Not found")]
    #[give(404)] _NotFound,
    #[error("Invalid parameter")]
    #[give(400)] InvalidField,
    #[error("Internal server error")]
    #[give(500)] _Internal,
}

// Our sample app only has one route that returns a Json<String>.
#[repr(transparent)]
#[derive(Debug, serde::Serialize, PartialEq, Eq)]
struct AppResponse(String);

// Sample validation function.
fn invalid_parameter(s: &str) -> bool {
    return s.len() > 30 || s.is_empty();
}

// Sample route handler guts.
fn index_fn(name: &str) -> Result<Json<AppResponse>, AppErr> {
    match invalid_parameter(name) {
        true => return Err(AppErr::InvalidField),
        false => return Ok(Json(AppResponse(name.to_string()))),
    }
}

#[cfg(test)]
#[test] fn demonstrate_functionality() {
    // Because we derived Codes on our custom AppErr, we get 'Status: From<AppErr>' for free. 
    // Which also implies that we have 'AppErr: Into<Status>'.
    use AppErr::*;
    use rocket::http::Status;

    let invalid_field: Status = InvalidField.into();
    let not_found: Status = _NotFound.into();
    let internal: Status = _Internal.into();

    assert_eq!(invalid_field.code, 400u16);
    assert_eq!(not_found.code, 404u16);
    assert_eq!(internal.code, 500u16);
    
    // A more verbose way of doing the same thing.
    assert_eq!(<AppErr as Into<Status>>::into(InvalidField).code, 400u16);
}

// Our Rocket route handler that returns our custom AppErr on failure.
// this is made possible by the MakeResponder and Code derive macros.
//
// Upon failure, the AppErr is converted into a Status code and returned to the client,
// along with Rocket's default HTTP body that is generated from the Status code.
#[get("/?<name>")]
fn index(name: String) -> Result<Json<AppResponse>, AppErr> {
    return index_fn(&name);
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
