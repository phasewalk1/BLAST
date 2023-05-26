# Blast - HTTP Status Code Derivation

Welcome to the Blast library! This Rust library provides a procedural macro to derive the `From` trait for an enum to convert it into `rocket::http::Status`. Each variant of the enum can be annotated with an HTTP status code using the `give` attribute.

## Features

- Automatic implementation of `From<YourEnum>` for `rocket::http::Status`.
- Easy annotation of enum variants with HTTP status codes.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
blast = "0.1.0"
