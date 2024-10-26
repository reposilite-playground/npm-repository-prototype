#[derive(serde::Serialize, serde::Deserialize)]
struct Version {
    name: String,
    version: String,
    homepage: String,
    repository: serde_json::Value,
    dependencies: serde_json::Value,
    #[serde(rename = "devDependencies")]
    dev_dependencies: serde_json::Value,
    scripts: serde_json::Value,
    author: serde_json::Value,
    license: String,
    readme: String,
    #[serde(rename = "readmeFilename")]
    readme_filename: String,
    #[serde(rename = "_id")]
    id: String,
    description: String,
    dist: serde_json::Value,
    #[serde(rename = "_npmVersion")]
    npm_version: String,
    #[serde(rename = "_npmUser")]
    npm_user: serde_json::Value,
    maintainers: serde_json::Value,
    directories: serde_json::Value,
}