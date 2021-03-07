use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct Status {
    pub status: String
}

#[derive(Deserialize)]
pub struct Host{
    pub is_paired: bool,
    pub pair_key: String,
    pub is_recording: bool,
}
