pub trait JsonSerialize {
    fn json_serialize(&self) -> &str;
    fn json_deserialize(&self, json_str: &str) -> Self;
}
