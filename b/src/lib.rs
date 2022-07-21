use std::any::Any;

use a::{Foreign, Storage};

trait Id {
    type Assoc;
}

impl<T> Id for T {
    type Assoc =T;
}

pub struct B;
impl<T> Foreign<B, T> for <T as Id>::Assoc {
    type Assoc = usize;
}

pub fn init_storage<T: 'static>(x: usize) -> Box<dyn Any> {
    Box::new(Storage::<T, B, T> {
        inner: x,
    })
}