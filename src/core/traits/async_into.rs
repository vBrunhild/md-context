pub trait AsyncInto<T>: Sized {
    fn into(self) -> impl Future<Output = T>;
}
