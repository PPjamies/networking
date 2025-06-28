use crate::BitSerializable::BitSerializable;

#[derive(Debug, Clone)]
pub struct IPv4Header {
    pub version: u8,              // 4 bits
    pub ihl: u8,                  // 4 bits
    pub dscp: u8,                 // 6 bits
    pub ecn: u8,                  // 2 bits
    pub total_length: u16,        // 16 bits
    pub identification: u16,      // 16 bits
    pub flags: u8,                // 3 bits
    pub fragment_offset: u16,     // 13 bits
    pub ttl: u8,                  // 8 bits
    pub protocol: u8,             // 8 bits
    pub header_checksum: u16,     // 16 bits
    pub source_address: u32,      // 32 bits
    pub destination_address: u32, // 32 bits
    pub options: Vec<u8>,         // 0 - 320bits
}

impl BitSerializable for IPv4Header {
    fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        todo!()
    }
}
