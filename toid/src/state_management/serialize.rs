pub trait Serialize {
    fn serialize(&self) -> String;
    fn deserialize(&self, serialized: String) -> Self;
}
