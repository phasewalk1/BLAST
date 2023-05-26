extern crate blast;
extern crate rocket;
extern crate thiserror;

use blast::ToStatus as Blast;
use thiserror::Error as ThisError;

#[derive(Blast, Clone, Debug, ThisError, PartialEq, Eq)]
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
