use std::sync::Arc;

pub trait Reducer<S, E> {
    fn reduce(&self, state: Arc<S>, event: E) -> S;
}
