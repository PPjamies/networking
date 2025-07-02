pub trait BitSerializable: Sized {
    fn to_bytes(&self) -> Vec<u8>;
    fn from_bytes(bytes: &[u8]) -> Result<Self, String>;
}
