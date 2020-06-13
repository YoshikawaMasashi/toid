use super::beat::Beat;

pub trait Note {
    fn get_start(&self) -> Beat;
    fn set_start(&self, start: Beat) -> Self;
}
