use std::any::Any;

use a::{Foreign, Storage};

pub struct C;

impl<T> Foreign<T, C> for C {
    type Assoc = Box<String>;
}

pub fn read_storage<T: 'static>(x: Box<dyn Any>) {
    if let Ok(target) = x.downcast::<Storage<C, T, C>>() {
        println!("{}", target.inner);
    }
}
