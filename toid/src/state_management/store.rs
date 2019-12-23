/// Storeはstateを保持し、必要に応じてアップデートをする窓口を提供します。
pub trait Store<T: Clone> {
    fn update_state(&mut self, state: T);
    fn get_state(&self) -> T;
}
