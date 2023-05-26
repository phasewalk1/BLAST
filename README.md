
# Project Title

A brief description of what this project does and who it's for

# BLAST!

Idomatic Rust utility that makes backend fun again.


## Overview
_BLAST!_ is a collection of utilities I've slowly been amassing that I find myself reaching for across various backend domains; I've decided to encapsulate them into a library partly as convenience for me but mainly to share them with you, in hopes they might find some use in your code too.
## Using _BLAST!_
_BLAST!_ provides macros that utilize `rocket`'s `Responder` trait, which define an interface for constructing HTTP responses. When coupled with your custom error enum, _BLAST!_'s `Codes` and `MakeResponder` make handling custom error logic for your server a breeze.

```Rust
extern crate blast;
extern crate thiserror;
#[macro_use]
extern crate rocket;

use blast::macros::{Codes, MakeResponder};
use rocket::serde::json::Json;
use thiserror::Error as ThisError;

// Our custom application errors.
#[derive(Codes, Clone, Debug, Eq, PartialEq, MakeResponder, ThisError)]
enum AppErr {
    #[error("Not found")]
    #[give(404)]
    _NotFound,
    #[error("Invalid parameter")]
    #[give(400)]
    InvalidField,
    #[error("Internal server error")]
    #[give(500)]
    _Internal,
}

// Our sample app has only one route that returns a JSON string.
#[repr(transparent)]
#[derive(Debug, serde::Serialize, PartialEq, Eq)]
struct AppResponse(String);

// Sample validation
fn invalid_parameter(s: &str) -> bool {
    return s.len() > 30 || s.is_empty();
}

// Sample index route handler.
//
// This is made possible by the `MakeResponder` and `Codes` traits,
// which implements `rocket::response::Responder` for us.
#[get("/?<name>")]
fn index_fn(name: String) -> Result<Json<AppResponse>, AppErr> {
    match invalid_parameter(&name) {
        // Return a custom error on failure, which gets decoded
        // to an http status code, and returned with a default body.
        true => return Err(AppErr::InvalidField),
        false => return Ok(Json(AppResponse(name.to_string()))),
    }
}

// Demonstrate `Codes` functionality
#[cfg(test)]
#[test] fn demonstrate() {
    // Because we derived `Codes`, we get `rocket::http::Status: From<AppErr>` for free. 
    use rocket::http::Status;
    use AppErr::*;

    // Use the `Status: From<AppErr>` impl to convert our custom error to status codes.
    let invalid_field: Status = InvalidField.into();
    let internal: Status = _Internal.into();
    let not_found: Status = _NotFound.into();

    assert_eq!(invalid_field.code, 400u16);
    assert_eq!(internal.code, 500u16);
    assert_eq!(not_found.code, 404u16);

    // Or, a more verbose way of saying the same thing.
    assert_eq!(<AppErr as Into<Status>>::into(InvalidField).code, 400u16);
}

// Build and launch our sample app as usual.
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index_fn])
}

```
