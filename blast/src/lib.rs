#[allow(unused_imports)]
#[macro_use]
pub extern crate blast_macros;
pub extern crate blast_interface;
pub extern crate blast_proc_macros;

/// _BLAST!_ interfaces.
pub mod interface {
    /// _BLAST!_ error interfaces.
    pub mod error {
        pub use blast_interface::error::*;
    }
    /// _BLAST!_ state interfaces.
    pub mod state {
        pub use blast_interface::state::*;
    }
}

/// _BLAST!_ macros.
pub mod macros {
    /// _BLAST!_ macros.
    pub use blast_macros::{catchers, maperr};
    /// _BLAST!_ procedural macros.
    pub use blast_proc_macros::{
        make_stateful, snake_trap, Limiter, MakeResponder,
    };
}