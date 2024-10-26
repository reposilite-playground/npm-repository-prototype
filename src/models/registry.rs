#[derive(serde::Serialize, serde::Deserialize)]
struct Registry {
    db_name: String,
    doc_count: i32,
    doc_del_count: i32,
    update_seq: i32,
    purge_seq: i32,
    compact_running: bool,
    disk_size: i32,
    data_size: i32,
    instance_start_time: String,
    disk_format_version: i32,
    committed_update_seq: i32,
}