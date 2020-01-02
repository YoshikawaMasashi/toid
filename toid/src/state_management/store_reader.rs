use std::sync::Arc;

pub trait StoreReader<S, O> {
    fn get_store(&self) -> Arc<S>;
    fn read(&self) -> O;
}
