#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct LoginRequest {
    #[serde(rename = "_id")]
    id: String,
    date: String,
    pub(crate) email: String,
    name: String,
    password: String,
    roles: Vec<String>,
    #[serde(rename = "type")]
    type_: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct LoginResponse {
    pub(crate) token: String,
    pub(crate) ok: bool,
}