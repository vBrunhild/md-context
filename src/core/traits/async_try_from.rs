use std::future::Future;

pub trait AsyncTryFrom<T>: Sized {
    type Error;

    fn try_from(value: T) -> impl Future<Output = Result<Self, Self::Error>>;
}
