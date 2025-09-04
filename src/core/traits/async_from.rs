use crate::core::traits::AsyncInto;

pub trait AsyncFrom<T>: Sized {
    fn from(value: T) -> impl Future<Output = Self>;
}

impl<T, U> AsyncInto<U> for T 
where
    U: AsyncFrom<T>
{
    fn into(self) -> impl Future<Output = U> {
        U::from(self)
    }
}
