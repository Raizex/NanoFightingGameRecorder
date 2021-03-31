use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct Status {
    pub status: String
}

#[derive(Deserialize, Serialize)]
pub struct Host{
    pub is_paired: bool,
    pub pair_key: String,
    pub is_recording: bool,
}

#[derive(Deserialize, Serialize)]
pub struct Client{
    pub key: String
}
