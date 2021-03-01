use serde::{Serialize};

#[derive(Serialize)]
pub struct Status {
    pub status: String
}

#[derive(Clone)]
pub struct Host{
    pub is_paired: bool,
    pub pair_key: String,
}

impl Host{
    pub fn new(paired: bool, key:String)-> Self{
        Host{
            is_paired: paired,
            pair_key: key,
        }
    }

    pub fn copy(&self) -> Host{
        Host{
            is_paired: self.is_paired.clone(),
            pair_key: self.pair_key.clone(),
        }
    }
}