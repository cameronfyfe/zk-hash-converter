#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct DataToHash {
    pub data: Vec<u8>,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
pub struct HashResults {
    pub sha256: [u8; 32],
    pub blake3: [u8; 32],
}
