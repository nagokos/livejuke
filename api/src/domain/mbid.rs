use std::marker::PhantomData;

use uuid::Uuid;

#[derive(Debug)]
pub struct Mbid<T> {
    pub mbid: Uuid,
    _phantom: PhantomData<T>,
}

impl<T> Clone for Mbid<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Mbid<T> {}

impl<T> Mbid<T> {
    pub fn new(mbid: Uuid) -> Self {
        Self {
            mbid,
            _phantom: PhantomData,
        }
    }
    pub fn get(&self) -> Uuid {
        self.mbid
    }
}
