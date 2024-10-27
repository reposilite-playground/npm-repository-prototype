use std::collections::HashMap;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PackageRequest {
    #[serde(rename = "_attachments")]
    pub attachments: HashMap<String, Attachment>,
    #[serde(rename = "_id")]
    pub id: String,
    pub access: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "dist-tags")]
    pub dist_tags: DistTags,
    pub name: String,
    pub versions: HashMap<String, Version>
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Attachment {
    pub content_type: String,
    pub data: String,
    pub length: usize,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DistTags {
    pub latest: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Version {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(rename = "_nodeVersion")]
    pub node_version: String,
    #[serde(rename = "_npmVersion")]
    pub npm_version: String,
    pub author: Author,
    pub description: Option<String>,
    pub dist: Dist,
    #[serde(rename = "gitHead")]
    pub git_head: String,
    pub license: String,
    pub main: String,
    pub name: String,
    pub readme: Option<String>,
    pub repository: Option<Repository>,
    pub scripts: HashMap<String, String>,
    pub version: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Author {
    pub name: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Dist {
    pub integrity: String,
    pub shasum: String,
    pub tarball: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Repository {
    #[serde(rename = "type")]
    pub type_: String,
    pub url: String,
}