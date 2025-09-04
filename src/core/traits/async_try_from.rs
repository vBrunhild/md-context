use crate::core::traits::AsyncTryInto;

pub trait AsyncTryFrom<T>: Sized {
    type Error;

    fn try_from(value: T) -> impl Future<Output = Result<Self, Self::Error>>;
}

impl<T, U> AsyncTryInto<U> for T
where
    U: AsyncTryFrom<T>
{
    type Error = U::Error;

    fn try_into(self) -> impl Future<Output = Result<U, Self::Error>> {
        U::try_from(self)
    }
}
