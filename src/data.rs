pub struct Data<T> {
    pub values: Vec<T>,
}

impl<T> Data<T> {
    pub fn new() -> Self {
        Self { values: Vec::new() }
    }
}