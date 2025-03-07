use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PrivateInputs {
    pub msg: [u8; 32],
    pub x: Vec<u8>,
    pub hid: Vec<u8>,
    pub htau: Vec<u8>,
    pub pk: Vec<u8>,
}
