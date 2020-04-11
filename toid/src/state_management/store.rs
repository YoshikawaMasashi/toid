use std::sync::Arc;

pub trait Store<S, E> {
    fn get_state(&self) -> Arc<S>;
    fn update_state(&self, event: E);
}
