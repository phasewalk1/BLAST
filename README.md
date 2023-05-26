
# _BLAST!_

Idomatic Rust utility that makes backend fun again.


## Overview
_BLAST!_ is a collection of utilities I've slowly been amassing that I find myself reaching for across various backend domains; I've decided to encapsulate them into a library partly as convenience for me but mainly to share them with you, in hopes they might find some use in your code too.

## Using _BLAST!_
Below is a simple example of using _BLAST!_ utility in an HTTP server,

```Rust
extern crate blast;
extern crate thiserror;
#[macro_use] extern crate rocket;

use blast::macros::{Codes, Responder};
use thiserror::Error as ThisError;
use rocket::serde::json::Json;

#[derive(Codes, Clone, Debug, Eq, PartialEq, Responder, ThisError)]
enum AppErr {
    #[error("Not found")]
    #[give(404)] _NotFound,
    #[error("Invalid parameter")]
    #[give(400)] InvalidField,
    #[error("Internal server error")]
    #[give(500)] _Internal,
}

#[repr(transparent)]
#[derive(serde::Serialize)]
struct AppResponse(String);

fn invalid_parameter(s: &str) -> bool {
    return s.len() > 30 || s.is_empty();
}

#[get("/?<name>")] fn index(name: String) -> Result<Json<AppResponse>, AppErr> {
    match invalid_parameter(&name) {
        true => return Err(AppErr::InvalidField),
        false => return Ok(Json(AppResponse(name))),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

```