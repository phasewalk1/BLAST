#![allow(dead_code)]
#![feature(type_alias_impl_trait)]

extern crate blast_interface;
extern crate blast_macros;
extern crate rocket;
extern crate thiserror;

#[cfg(test)]
#[cfg(feature = "rocket")]
mod interface_example {
    use blast_interface::error as blast;
    use blast_macros::{Codes, MakeResponder};
    use thiserror::Error as ThisError;

    #[derive(Codes, MakeResponder, Clone, Debug, ThisError, PartialEq, Eq)]
    #[rustfmt::skip]
    enum MyAppError {
        #[error("failed to update counter!")]
        #[give(500)] UpdateError,
        #[error("failed to get counter!")]
        #[give(404)] GetError,
        #[error("server timed out!")]
        #[give(408)] Timeout,
    }

    impl blast::Error for MyAppError {
        type Yield = rocket::http::Status;
    }

    mod interface_example_test {
        #[test]
        fn use_custom_error_interface() {
            use super::blast::Response;
            use super::MyAppError::{self, *};

            let err: MyAppError = GetError;
            let res: <MyAppError as super::blast::Error>::Yield = err.make_response();

            println!("Got response: {:?}", res);
            assert_eq!(res, rocket::http::Status::NotFound);
        }
    }
}

mod macros_example {
    use blast_macros::{Codes, MakeResponder};
    use thiserror::Error as ThisError;

    #[derive(Codes, MakeResponder, Clone, Debug, ThisError, PartialEq, Eq)]
    #[rustfmt::skip] 
    pub enum AppError {
        /// failed unlock a connection from the pool.
        #[error("failed to get connection pool!")]
        #[give(503)] UnlockError,

        /// failed a message GET request.
        #[error("failed to get message by id!")]
        #[give(404)] GetError,

        /// failed a message POST/PUT request.
        #[error("failed to send message!")]
        #[give(500)] PutError,

        /// internal server error.
        #[error("server error!")]
        #[give(500)] ServerError,

        /// failed to decode message into a valid request.
        #[error("failed to decode message!")]
        #[give(400)] BadRequest,
    }

    #[cfg(test)]
    mod crate_test {

        use super::AppError::{self, *};
        use rocket::http::Status;

        #[test]
        fn test_macro_expansion_correctness() {
            assert_eq!(
                <AppError as Into<Status>>::into(UnlockError),
                Status::ServiceUnavailable
            );
            assert_eq!(<AppError as Into<Status>>::into(GetError), Status::NotFound);
            assert_eq!(
                <AppError as Into<Status>>::into(PutError),
                Status::InternalServerError
            );
            assert_eq!(
                <AppError as Into<Status>>::into(ServerError),
                Status::InternalServerError
            );
            assert_eq!(
                <AppError as Into<Status>>::into(BadRequest),
                Status::BadRequest
            );
        }
    }
}
