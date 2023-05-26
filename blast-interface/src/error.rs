#[cfg(test)]
extern crate blast_macros;
#[cfg(feature = "rocket")]
extern crate rocket;
#[cfg(test)]
extern crate thiserror;
#[cfg(feature = "rocket")]
use rocket::response::Responder;

/// An interface to a type that wraps an error and responds to a client.
///
/// An example Response interface would be: `rocket::http::Status`.
pub trait Response<E: Error>
where
    Self: From<E>,
{
    fn make(error: E) -> Self {
        return Self::from(error);
    }
}

/// An interface to a type that defines an app error.
pub trait Error
where
    Self: std::fmt::Debug + std::error::Error + Clone + PartialEq + Eq,
{
    type Yield: Response<Self>;
    fn make_response(self) -> Self::Yield {
        return Self::Yield::make(self);
    }
}

#[cfg(feature = "rocket")]
impl<E: Error> Response<E> for rocket::http::Status
where
    Self: From<E>,
{
    fn make(error: E) -> Self {
        return Self::from(error);
    }
}
