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

use blast::macros::MakeResponder;
use rocket::serde::json::Json;
use rocket::http::Status;
use thiserror::Error as ThisError;

// Our custom application error
#[derive(MakeResponder)]
enum AppError {
    NotFound,
    InternalError,
}

// Generate a Status: From<AppError> implementation
blast::macros::maperr! {
    on AppError ...
    NotFound => 404,
    InternalError => 500,
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
// This is made possible by the `MakeResponder` and `maperr` macros,
// which implements `Status: From<AppErr>` and `rocket::response::Responder` for us.
#[get("/?<name>")]
fn index_fn(name: String) -> Result<Json<AppResponse>, AppError> {
    match invalid_parameter(&name) {
        // Return a custom error on failure, which gets decoded
        // to an http status code, and returned with a default body.
        true => return Err(AppErr::InvalidField),
        false => return Ok(Json(AppResponse(name.to_string()))),
    }
}

// Build and launch our sample app as usual.
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index_fn])
}

#[cfg(test)]
mod blast_test {
    use super::*;
    use AppError::*;

    #[test]
    fn showoff_blast() {
        let not_found: Status = NotFound.into();
        let internal: Status = InternalError.into();

        assert_eq!(Status::NotFound, not_found);
        assert_eq!(Status::InternalServerError, internal);
    }
}

```
