pub trait State
where
    Self: Sized + Send + Sync + 'static,
    Self::Inner: Sized + Send + Sync + 'static,
{
    type Inner;
}
