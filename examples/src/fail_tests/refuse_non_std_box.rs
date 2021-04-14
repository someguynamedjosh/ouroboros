use ouroboros::self_referencing;
use std::ops::Deref;

struct Box<T>(T);

impl<T> Box<T> {
    pub fn new(data: T) -> Self {
        Self(data)
    }
}

impl<T> Deref for Box<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

unsafe impl<T> stable_deref_trait::StableDeref for Box<T> {}

#[self_referencing]
struct Simple {
    data: Box<String>,
    #[borrows(data)]
    data_ref: &'this String,
}

fn main() {
    let simple = Simple::new(Box::new(format!("Hello world")), |data_ref| data_ref);
}
