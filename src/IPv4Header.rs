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
    pub options: Vec<u8>,         // 0 - 320 bits
}

impl BitSerializable for IPv4Header {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::with_capacity(20 + self.options.len());

        // version (4 bits) + ihl (4 bits) = 1 byte
        bytes.push((self.version << 4) | (self.ihl & 0x0F));

        // dscp (6 bits) + ecn (2 bits) = 1 byte
        bytes.push((self.dscp << 2) | (self.ecn & 0x03));

        // total length (16 bits) = 2 bytes
        bytes.extend(&self.total_length.to_be_bytes());

        // identification (16 bits) = 2 bytes
        bytes.extend(&self.identification.to_be_bytes());

        // flags (3 bits) + fragment offset (13 bits) = 2 bytes
        let flag_and_offset = ((self.flags as u16) << 13) | (self.fragment_offset & 0x1FFF);
        bytes.extend(&flag_and_offset.to_be_bytes());

        // ttl (8 bits) = 1 byte
        bytes.push(self.ttl);

        // protocol (8 bits) = 1 byte
        bytes.push(self.protocol);

        // header checksum (16 bits) = 2 bytes
        bytes.extend(&self.header_checksum.to_be_bytes());

        // source address (32 bits) = 8 bytes
        bytes.extend(&self.source_address.to_be_bytes());

        // destination address (32 bits) = 8 bytes
        bytes.extend(&self.destination_address.to_be_bytes());

        // options (0 - 320 bits) = 0 - 40 bytes
        bytes.extend(&self.options);

        bytes
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        if (bytes.len() < 20) {
            // todo: handle error
        }
    }
}
