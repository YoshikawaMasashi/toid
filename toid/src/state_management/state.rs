pub trait State<E> {
    fn reduce(&self, event: E) -> Self;
}
