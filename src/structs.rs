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
