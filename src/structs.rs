use serde::Deserialize;

#[derive(Deserialize)]
pub struct Info {
    pub(crate) key: String,
}
