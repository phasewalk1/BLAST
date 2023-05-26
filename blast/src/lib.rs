#![cfg(feature = "rocket")]
#[allow(unused_imports)]
#[macro_use]
pub extern crate blast_macros;
pub extern crate blast_interface;

pub use blast_interface as interface;

pub mod macros {
    pub use blast_macros::*;

    #[cfg(feature = "rocket")]
    #[macro_export]
    macro_rules! maperr {
    (on $enum:ident ... $($variant:ident => $code:expr),* $(,)?) => {
        impl From<$enum> for rocket::http::Status {
            fn from(error: $enum) -> rocket::http::Status {
                match error {
                    $($enum::$variant => rocket::http::Status::from_code($code).unwrap(),)*
                }
            }
        }
    };
	}
}

#[cfg(feature = "rocket")]
#[cfg(test)]
mod macro_test {
    use super::macros::*;
    use rocket::http::Status;

    #[derive(Debug, PartialEq)]
    pub enum AppError {
        NotFound,
        InternalError,
    }

    crate::maperr! {
        on AppError ...
        NotFound => 404,
        InternalError => 500,
    }

    #[test]
    fn test_maperr() {
        // fat enum
        use AppError::*;

        let not_found: Status = NotFound.into();
        let internal: Status = InternalError.into();

        assert_eq!(Status::NotFound, not_found);
        assert_eq!(Status::InternalServerError, internal);
    }

    #[test]
    fn readme() {
        #[derive(MakeResponder)]
        enum AppError {
            NotFound,
            InternalError,
        }
        crate::maperr! {
            on AppError ...
            NotFound => 404,
            InternalError => 500,
        }
        
        let not_found: Status = AppError::NotFound.into();
        let internal: Status = AppError::InternalError.into();

        assert_eq!(Status::NotFound, not_found);
        assert_eq!(Status::InternalServerError, internal);
    }
}
