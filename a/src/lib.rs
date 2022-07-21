pub trait Foreign<T, U> {
    type Assoc;
}

pub struct Storage<T: Foreign<U, V>, U, V> {
    pub inner: <T as Foreign<U, V>>::Assoc,
}
