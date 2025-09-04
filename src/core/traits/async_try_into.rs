pub trait AsyncTryInto<T>: Sized {
    type Error;

    fn try_into(self) -> impl Future<Output = Result<T, Self::Error>>;
}
