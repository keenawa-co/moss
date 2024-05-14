#[derive(Debug, Serialize, Deserialize)]
pub struct Serial {
    pub version: u64,
    pub hash: String,
    pub data: String,
}
