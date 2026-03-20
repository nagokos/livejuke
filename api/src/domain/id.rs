use std::marker::PhantomData;

#[derive(Debug)]
pub struct Id<T> {
    pub id: i64,
    _phantom: PhantomData<T>,
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for Id<T> {}

impl<T> Id<T> {
    pub fn new(id: i64) -> Self {
        Self {
            id,
            _phantom: PhantomData,
        }
    }
    pub fn get(&self) -> i64 {
        self.id
    }
}
