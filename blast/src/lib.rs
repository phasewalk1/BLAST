#[allow(unused_imports)]
#[macro_use]
pub extern crate blast_macros;
pub extern crate blast_interface;
pub extern crate blast_proc_macros;

pub use blast_interface as interface;

pub mod macros {
    pub use blast_macros::{catchers, maperr};
    pub use blast_proc_macros::{
        make_stateful, snake_case_catcher as snake_trap, Limiter, MakeResponder,
    };
}

#[cfg(test)]
mod rocket_test {
    use crate::interface::error::*;
    use crate::macros::MakeResponder as Response;
    use rocket::http::Status;

    #[derive(Debug, Response, Clone, PartialEq, Eq)]
    pub enum MyErr {
        NotFound,
        InternalError,
    }

    use MyErr::*;

    impl Error for MyErr {}

    crate::macros::maperr! {
        on MyErr ...
        NotFound => 404,
        InternalError => 500,
    }

    mod catcher {
        use super::MyErr;
        crate::macros::catchers! (
            using MyErr ...
            not_found_catcher,      NotFound      => ((404, R)),
            internal_error_catcher, InternalError => ((500, R)),
        );
    }

    #[test]
    fn test_err() {
        let not_found: Status = NotFound.wrap();
        let internal: Status = InternalError.wrap();

        assert_eq!(Status::NotFound, not_found);
        assert_eq!(Status::InternalServerError, internal);

        #[allow(unused_imports)]
        use catcher::{default_catcher, internal_error_catcher, not_found_catcher};
    }
}
