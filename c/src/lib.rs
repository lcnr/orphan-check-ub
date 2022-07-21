use std::any::Any;

use a::{Foreign, Storage};

pub struct C;

impl<T, U> Foreign<T, U> for C {
    type Assoc = Box<String>;
}

pub fn read_storage<T: 'static, U: 'static>(x: Box<dyn Any>) {
    if let Ok(target)= x.downcast::<Storage<C, T, U>>() {
        println!("{}", target.inner);
    }
}