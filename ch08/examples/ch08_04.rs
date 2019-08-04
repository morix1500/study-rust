trait Init<T> {
    fn init(t: T) -> Self;
}

impl<T> Init<TP> for Box<T> {
    fn init(t: T) -> Self {
        Box::new(t)
    }
}

fn main() {
    let data = Box::init("foo");
    let data = Box::<i32>::init(0.1);
}