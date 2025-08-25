use num::Unsigned;

#[derive(Debug, Clone)]
pub struct Section<T: Unsigned> {
    start: T,
    end: T,
}

impl<T: Unsigned> for Section<T> {
    fn new(start: Option<T>, end: Option<T>) -> Self {
        unimplemented!("implement `new` for `Section`")
    }
}
