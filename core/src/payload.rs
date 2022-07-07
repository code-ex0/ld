
pub type Payloads = Vec<Payload>;

#[derive(Debug, Clone, PartialEq)]
pub struct Payload {
    pub version: u32,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub public_key: Vec<u8>,
    pub signature: Vec<u8>,
}

impl Payload {
    pub fn new(version: u32,data: Vec<u8>, timestamp: u64, public_key: Vec<u8>, signature: Vec<u8>) -> Payload {
        Payload {
            version,
            data,
            timestamp,
            public_key,
            signature,
        }
    }

    ///
    /// getter
    ///
    pub fn get_version(&self) -> u32 {
        self.version
    }

    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn get_public_key(&self) -> &Vec<u8> {
        &self.public_key
    }

    pub fn get_signature(&self) -> &Vec<u8> {
        &self.signature
    }
}