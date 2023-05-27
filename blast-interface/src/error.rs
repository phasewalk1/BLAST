/// Wraps an Error interface in a type that can be passed back to the client.
pub trait Respondable<E: Error>
where
    Self: Sized,
    Self::Payload: From<Self>,
{
    type Payload: From<E>;
    fn wrap(self) -> Self::Payload {
        return self.into();
    }
}

/// A very simple and extendable error interface.
pub trait Error
where
    Self: Clone + core::fmt::Debug + Sized + PartialEq + Eq + 'static,
{
}
