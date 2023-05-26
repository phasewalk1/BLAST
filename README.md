
# _BLAST!_

Idomatic Rust utility that makes backend fun again.


## Overview
_BLAST!_ is a collection of utilities I've slowly been amassing that I find myself reaching for across various backend domains; I've decided to encapsulate them into a library partly as convenience for me but mainly to share them with you, in hopes they might find some use in your code too.

## Using _BLAST!_
Below is a simple example of using _BLAST!_ utility in an HTTP server,

```Rust

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

#[repr(transparent)]
#[derive(Debug, serde::Serialize, PartialEq, Eq)]
struct AppResponse(String);

fn invalid_parameter(s: &str) -> bool {
    return s.len() > 30 || s.is_empty();
}

fn index_fn(name: &str) -> Result<Json<AppResponse>, AppErr> {
    match invalid_parameter(name) {
        true => return Err(AppErr::InvalidField),
        false => return Ok(Json(AppResponse(name.to_string()))),
    }
}

#[cfg(test)]
#[test] fn test_index_fn() {
    use rocket::http::Status;
    use AppErr::*;

    let mut invalid = String::new();
    for _ in 0..31 {
        invalid.push('a');
    }
    assert_eq!(index_fn(&invalid), Err(InvalidField));
    assert_eq!(index_fn(""), Err(InvalidField));
    assert_eq!(<AppErr as Into<Status>>::into(InvalidField).code, 400u16);
}

#[get("/?<name>")]
fn index(name: String) -> Result<Json<AppResponse>, AppErr> {
    return index_fn(&name);
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

```