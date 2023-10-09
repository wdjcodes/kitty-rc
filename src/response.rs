use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct Response {
    ok: bool,
    data: String,
}

impl Response {

    pub fn data(&self) -> &String {
        &self.data
    }
}

impl TryFrom<&[u8]> for Response {
    type Error = serde_json::Error;

    fn try_from(value: &[u8]) -> std::result::Result<Self, Self::Error> {
        serde_json::from_slice::<Response>(value)
    }
}