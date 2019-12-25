pub trait Serialize {
    fn serialize(&self) -> String;
    fn deserialize(serialized: String) -> Self;
}
