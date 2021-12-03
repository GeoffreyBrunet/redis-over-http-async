use serde::Deserialize;

#[derive(Deserialize)]
pub struct Info {
    pub key: String,
}

#[derive(Deserialize)]
pub struct Infos {
    pub key: String,
    pub set_ttl: usize,
}

#[derive(Deserialize)]
pub struct HInfo {
    pub userid: String,
    pub key: String,
}

#[derive(Deserialize)]
pub struct HInfos {
    pub userid: String,
    pub key: usize,
    pub set_ttl: usize,
}
