/// Reduceでは、あるeventが会った時に、stateの変更の仕方をユーザーが決めることができるインターフェースです。
pub trait Reduce<T, S> {
    fn reduce(&self, state: T, event: S) -> T;
}

pub trait Reducer<T, S> {
    fn reduce(&self, event: S);
}
