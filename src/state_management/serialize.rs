use std::marker::Sized;

pub trait Serialize<T: Sized> {
    fn serialize(&self) -> Result<String, String>;
    fn deserialize(serialized: String) -> Result<T, String>;
}
