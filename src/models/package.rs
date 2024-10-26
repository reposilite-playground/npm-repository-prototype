#[derive(serde::Serialize, serde::Deserialize)]
struct Package {
    #[serde(rename = "_id")]
    id: String,
    #[serde(rename = "_rev")]
    rev: String,
    name: String,
    description: String,
    #[serde(rename = "dist-tags")]
    dist_tags: serde_json::Value,
    versions: serde_json::Value,
    time: serde_json::Value,
    author: serde_json::Value,
    repository: serde_json::Value,
    #[serde(rename = "_attachments")]
    attachments: serde_json::Value,
    readme: String,
}