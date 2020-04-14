pub trait State<E> {
    fn new() -> Self;
    fn reduce(&self, event: E) -> Self;
}
