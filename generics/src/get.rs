pub struct Wrapper<T> {
    pub value: T,
}

impl<T> Wrapper<T>{
    pub fn get_value(&self) -> &T {
        &self.value
    }
}