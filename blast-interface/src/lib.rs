#![feature(associated_type_defaults)]
pub mod error;
pub mod operation;

#[cfg(test)]
extern crate blast_macros;
#[cfg(test)]
extern crate blast_proc_macros;
extern crate rocket;
#[cfg(test)]
extern crate thiserror;
